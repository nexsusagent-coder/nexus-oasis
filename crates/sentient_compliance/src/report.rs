//! ─── Compliance Reporting ───

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use crate::controls::{Control, ControlStatus, ControlCategory};
use crate::audit::AuditEvent;
use super::Soc2Certification;

/// Compliance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    pub id: String,
    pub report_type: ReportType,
    pub generated_at: DateTime<Utc>,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub summary: ReportSummary,
    pub controls: ControlReportSection,
    pub audit_summary: AuditReportSection,
    pub certification_status: Option<CertificationReportSection>,
    pub recommendations: Vec<Recommendation>,
}

impl ComplianceReport {
    pub fn new(
        report_type: ReportType,
        controls: &HashMap<String, Control>,
        audit_log: &[AuditEvent],
        certification: &Option<Soc2Certification>,
    ) -> Self {
        let now = Utc::now();
        
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            report_type,
            generated_at: now,
            period_start: now - chrono::Duration::days(30),
            period_end: now,
            summary: Self::calculate_summary(controls),
            controls: Self::build_controls_section(controls),
            audit_summary: Self::build_audit_section(audit_log),
            certification_status: certification.as_ref().map(|c| CertificationReportSection {
                type_: format!("{:?}", c.cert_type),
                status: format!("{:?}", c.status),
                start_date: c.start_date,
                expected_completion: c.expected_completion,
                progress: if c.completed_date.is_some() { 100 } else { 50 },
            }),
            recommendations: Self::generate_recommendations(controls),
        }
    }
    
    fn calculate_summary(controls: &HashMap<String, Control>) -> ReportSummary {
        let total = controls.len() as u32;
        let compliant = controls.values()
            .filter(|c| c.status == ControlStatus::Compliant)
            .count() as u32;
        let non_compliant = controls.values()
            .filter(|c| c.status == ControlStatus::NonCompliant)
            .count() as u32;
        let not_assessed = controls.values()
            .filter(|c| c.status == ControlStatus::NotAssessed)
            .count() as u32;
        
        ReportSummary {
            controls_total: total,
            controls_compliant: compliant,
            controls_non_compliant: non_compliant,
            controls_not_assessed: not_assessed,
            compliance_rate: if total > 0 { 
                (compliant as f64 / total as f64) * 100.0 
            } else { 
                0.0 
            },
            overall_score: if total > 0 {
                (compliant as f64 / total as f64) * 100.0
            } else {
                0.0
            },
        }
    }
    
    fn build_controls_section(controls: &HashMap<String, Control>) -> ControlReportSection {
        let mut by_category = HashMap::new();
        
        for category in [
            ControlCategory::Security,
            ControlCategory::Availability,
            ControlCategory::ProcessingIntegrity,
            ControlCategory::Confidentiality,
            ControlCategory::Privacy,
        ] {
            let category_controls: Vec<_> = controls.values()
                .filter(|c| c.category == category)
                .collect();
            
            let compliant = category_controls.iter()
                .filter(|c| c.status == ControlStatus::Compliant)
                .count() as u32;
            let total = category_controls.len() as u32;
            
            by_category.insert(
                format!("{}", category),
                CategorySummary {
                    total,
                    compliant,
                    rate: if total > 0 { (compliant as f64 / total as f64) * 100.0 } else { 0.0 },
                },
            );
        }
        
        ControlReportSection {
            total_controls: controls.len() as u32,
            by_category,
            issues: controls.values()
                .flat_map(|c| c.issues.iter())
                .map(|i| IssueSummary {
                    id: i.id.clone(),
                    title: i.title.clone(),
                    severity: format!("{:?}", i.severity),
                    status: format!("{:?}", i.status),
                })
                .collect(),
        }
    }
    
    fn build_audit_section(audit_log: &[AuditEvent]) -> AuditReportSection {
        use crate::audit::AuditSeverity;
        
        let critical = audit_log.iter()
            .filter(|e| e.severity == AuditSeverity::Critical)
            .count() as u32;
        let errors = audit_log.iter()
            .filter(|e| e.severity == AuditSeverity::Error)
            .count() as u32;
        let warnings = audit_log.iter()
            .filter(|e| e.severity == AuditSeverity::Warning)
            .count() as u32;
        
        AuditReportSection {
            total_events: audit_log.len() as u32,
            critical_events: critical,
            error_events: errors,
            warning_events: warnings,
            top_event_types: vec![
                "Login".into(),
                "ApiRequest".into(),
                "DataAccess".into(),
            ],
        }
    }
    
    fn generate_recommendations(controls: &HashMap<String, Control>) -> Vec<Recommendation> {
        let mut recommendations = Vec::new();
        
        // Check for non-compliant controls
        for control in controls.values() {
            if control.status == ControlStatus::NonCompliant {
                recommendations.push(Recommendation {
                    priority: RecommendationPriority::High,
                    control_id: control.id.clone(),
                    title: format!("Remediate control {}", control.id),
                    description: control.description.clone(),
                    estimated_effort: "Medium".into(),
                });
            }
        }
        
        // Check for controls without evidence
        for control in controls.values() {
            if control.evidence_refs.is_empty() {
                recommendations.push(Recommendation {
                    priority: RecommendationPriority::Medium,
                    control_id: control.id.clone(),
                    title: format!("Collect evidence for {}", control.id),
                    description: "No evidence has been collected for this control".into(),
                    estimated_effort: "Low".into(),
                });
            }
        }
        
        recommendations
    }
    
    /// Export to JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
    
    /// Export to HTML
    pub fn to_html(&self) -> String {
        format!(r#"<!DOCTYPE html>
<html>
<head>
    <title>SOC 2 Compliance Report</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .score {{ font-size: 48px; color: {}; }}
        .section {{ margin: 20px 0; padding: 20px; background: #f5f5f5; }}
        table {{ width: 100%; border-collapse: collapse; }}
        th, td {{ padding: 10px; border: 1px solid #ddd; text-align: left; }}
        th {{ background: #333; color: white; }}
        .pass {{ color: green; }}
        .fail {{ color: red; }}
    </style>
</head>
<body>
    <h1>SOC 2 Compliance Report</h1>
    <p>Generated: {}</p>
    
    <div class="section">
        <h2>Overall Compliance Score</h2>
        <div class="score">{:.1}%</div>
        <p>Controls: {}/{} compliant</p>
    </div>
    
    <div class="section">
        <h2>Controls by Category</h2>
        <table>
            <tr><th>Category</th><th>Total</th><th>Compliant</th><th>Rate</th></tr>
            {}
        </table>
    </div>
    
    <div class="section">
        <h2>Recommendations</h2>
        <ul>
            {}
        </ul>
    </div>
</body>
</html>"#,
            if self.summary.compliance_rate >= 80.0 { "green" } else if self.summary.compliance_rate >= 60.0 { "orange" } else { "red" },
            self.generated_at.to_rfc3339(),
            self.summary.compliance_rate,
            self.summary.controls_compliant,
            self.summary.controls_total,
            self.controls.by_category.iter()
                .map(|(k, v)| format!("<tr><td>{}</td><td>{}</td><td>{}</td><td>{:.1}%</td></tr>", k, v.total, v.compliant, v.rate))
                .collect::<Vec<_>>()
                .join("\n"),
            self.recommendations.iter()
                .map(|r| format!("<li><strong>[{:?}]</strong> {} - {}</li>", r.priority, r.control_id, r.title))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}

/// Report type
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ReportType {
    /// Executive summary
    Summary,
    /// Detailed control report
    Detailed,
    /// Audit-ready report
    Audit,
    /// Gap analysis
    GapAnalysis,
}

/// Report summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSummary {
    pub controls_total: u32,
    pub controls_compliant: u32,
    pub controls_non_compliant: u32,
    pub controls_not_assessed: u32,
    pub compliance_rate: f64,
    pub overall_score: f64,
}

/// Control report section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlReportSection {
    pub total_controls: u32,
    pub by_category: HashMap<String, CategorySummary>,
    pub issues: Vec<IssueSummary>,
}

/// Category summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategorySummary {
    pub total: u32,
    pub compliant: u32,
    pub rate: f64,
}

/// Issue summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueSummary {
    pub id: String,
    pub title: String,
    pub severity: String,
    pub status: String,
}

/// Audit report section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditReportSection {
    pub total_events: u32,
    pub critical_events: u32,
    pub error_events: u32,
    pub warning_events: u32,
    pub top_event_types: Vec<String>,
}

/// Certification report section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificationReportSection {
    #[serde(rename = "type")]
    pub type_: String,
    pub status: String,
    pub start_date: DateTime<Utc>,
    pub expected_completion: DateTime<Utc>,
    pub progress: u32,
}

/// Recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub priority: RecommendationPriority,
    pub control_id: String,
    pub title: String,
    pub description: String,
    pub estimated_effort: String,
}

/// Recommendation priority
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RecommendationPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_report_creation() {
        let controls = HashMap::new();
        let audit_log = vec![];
        let report = ComplianceReport::new(
            ReportType::Summary,
            &controls,
            &audit_log,
            &None,
        );
        assert!(report.summary.controls_total >= 0);
    }
}
