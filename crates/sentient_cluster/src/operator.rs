//! ─── Kubernetes Operator ───

use kube::{Api, Client, Config};
use kube::runtime::{Controller, controller::Action, watcher::Config as WatcherConfig};
use futures::StreamExt;
use std::time::Duration;

use crate::crds::{SentientAgent, SentientAgentStatus, AgentPhase};

/// SENTIENT Operator
pub struct Operator {
    client: Client,
    namespace: String,
}

impl Operator {
    /// Create new operator
    pub async fn new(namespace: impl Into<String>) -> Result<Self, OperatorError> {
        let config = Config::infer().await
            .map_err(|e| OperatorError::Config(e.to_string()))?;
        
        let client = Client::try_from(config)
            .map_err(|e| OperatorError::Config(e.to_string()))?;
        
        Ok(Self {
            client,
            namespace: namespace.into(),
        })
    }
    
    /// Create operator with existing client
    pub fn with_client(client: Client, namespace: impl Into<String>) -> Self {
        Self {
            client,
            namespace: namespace.into(),
        }
    }
    
    /// Run the operator
    pub async fn run(self) -> Result<(), OperatorError> {
        let agents: Api<SentientAgent> = Api::namespaced(
            self.client.clone(),
            &self.namespace,
        );
        
        // Setup controller
        let controller = Controller::new(agents, WatcherConfig::default())
            .run(
                Self::reconcile,
                Self::error_policy,
                std::sync::Arc::new(self.client),
            );
        
        // Run until interrupted
        controller
            .for_each(|res| async {
                match res {
                    Ok((agent, _)) => {
                        log::info!("Reconciled agent: {:?}", agent.name);
                    }
                    Err(e) => {
                        log::error!("Reconcile error: {}", e);
                    }
                }
            })
            .await;
        
        Ok(())
    }
    
    /// Reconcile agent state
    async fn reconcile(
        agent: SentientAgent,
        client: std::sync::Arc<Client>,
    ) -> Result<Action, OperatorError> {
        let name = agent.metadata.name.as_deref()
            .ok_or_else(|| OperatorError::Reconcile("Missing name".into()))?;
        
        let namespace = agent.metadata.namespace.as_deref().unwrap_or("default");
        
        log::info!("Reconciling agent {} in {}", name, namespace);
        
        // Check current state
        let spec = &agent.spec;
        let status = agent.status.clone().unwrap_or_default();
        
        // Determine desired state
        let desired_replicas = spec.replicas;
        let current_replicas = status.current_replicas;
        
        if current_replicas != desired_replicas {
            // Scale deployment
            let api: Api<k8s_openapi::api::apps::v1::Deployment> = 
                Api::namespaced((*client).clone(), namespace);
            
            let deployment_name = format!("sentient-agent-{}", name);
            
            // Create or update deployment
            if let Some(_dep) = api.get_opt(&deployment_name).await? {
                // Update existing deployment
                Self::scale_deployment(&api, &deployment_name, desired_replicas).await?;
            } else {
                // Create new deployment
                Self::create_deployment(&api, name, spec).await?;
            }
            
            // Update status
            let agents: Api<SentientAgent> = Api::namespaced((*client).clone(), namespace);
            
            let new_status = SentientAgentStatus {
                current_replicas: desired_replicas,
                ready_replicas: 0,
                phase: AgentPhase::Pending,
                conditions: vec![],
                last_update: Some(chrono::Utc::now().to_rfc3339()),
            };
            
            // Patch status
            let _ = agents.patch_status(
                name,
                &kube::api::PatchParams::default(),
                &kube::api::Patch::Merge(serde_json::json!({
                    "status": new_status
                })),
            ).await;
        }
        
        // Requeue after 30 seconds
        Ok(Action::requeue(Duration::from_secs(30)))
    }
    
    /// Error policy
    fn error_policy(_agent: &SentientAgent, error: &OperatorError, _ctx: &std::sync::Arc<Client>) -> Action {
        log::error!("Agent error: {}", error);
        Action::requeue(Duration::from_secs(5))
    }
    
    /// Create deployment
    async fn create_deployment(
        api: &Api<k8s_openapi::api::apps::v1::Deployment>,
        name: &str,
        spec: &crate::crds::SentientAgentSpec,
    ) -> Result<(), OperatorError> {
        use k8s_openapi::api::apps::v1::{Deployment, DeploymentSpec};
        use k8s_openapi::api::core::v1::{Container, PodSpec, PodTemplateSpec, ResourceRequirements};
        
        let deployment = Deployment {
            metadata: kube::api::ObjectMeta {
                name: Some(format!("sentient-agent-{}", name)),
                labels: Some(std::collections::BTreeMap::from([
                    ("app".into(), "sentient-agent".into()),
                    ("agent".into(), name.into()),
                ])),
                ..Default::default()
            },
            spec: Some(DeploymentSpec {
                replicas: Some(spec.replicas),
                selector: kube::api::LabelSelector {
                    match_labels: Some(std::collections::BTreeMap::from([
                        ("app".into(), "sentient-agent".into()),
                        ("agent".into(), name.into()),
                    ])),
                    ..Default::default()
                },
                template: PodTemplateSpec {
                    metadata: Some(kube::api::ObjectMeta {
                        labels: Some(std::collections::BTreeMap::from([
                            ("app".into(), "sentient-agent".into()),
                            ("agent".into(), name.into()),
                        ])),
                        ..Default::default()
                    }),
                    spec: Some(PodSpec {
                        containers: vec![Container {
                            name: "sentient".into(),
                            image: Some("sentientai/agent:latest".into()),
                            ports: Some(vec![k8s_openapi::api::core::v1::ContainerPort {
                                container_port: 8080,
                                ..Default::default()
                            }]),
                            resources: Some(ResourceRequirements {
                                limits: Some(spec.resources.memory.as_ref().map(|m| {
                                    ("memory".into(), k8s_openapi::apimachinery::pkg::api::resource::Quantity(m.clone()))
                                }).into_iter().chain(
                                    spec.resources.cpu.as_ref().map(|c| {
                                        ("cpu".into(), k8s_openapi::apimachinery::pkg::api::resource::Quantity(c.clone()))
                                    })
                                ).collect()),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                },
                ..Default::default()
            }),
            ..Default::default()
        };
        
        api.create(&kube::api::PostParams::default(), &deployment).await?;
        
        Ok(())
    }
    
    /// Scale deployment
    async fn scale_deployment(
        api: &Api<k8s_openapi::api::apps::v1::Deployment>,
        name: &str,
        replicas: i32,
    ) -> Result<(), OperatorError> {
        api.patch(
            name,
            &kube::api::PatchParams::default(),
            &kube::api::Patch::Merge(serde_json::json!({
                "spec": {
                    "replicas": replicas
                }
            })),
        ).await?;
        
        Ok(())
    }
}

/// Operator error
#[derive(Debug, thiserror::Error)]
pub enum OperatorError {
    #[error("Config error: {0}")]
    Config(String),
    
    #[error("Reconcile error: {0}")]
    Reconcile(String),
    
    #[error("Kubernetes error: {0}")]
    Kube(#[from] kube::Error),
}
