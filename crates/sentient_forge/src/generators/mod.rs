//! ─── KOD URETICILERI ───

use crate::{ForgeRequest, ToolType};
use sentient_common::error::{SENTIENTError, SENTIENTResult};
use std::collections::HashMap;

/// Generator trait
pub trait Generator: Send + Sync {
    /// Uretim yap
    fn generate(&self, request: &ForgeRequest) -> SENTIENTResult<String>;
    
    /// Desteklenen arac tipi
    fn tool_type(&self) -> ToolType;
}

/// n8n Workflow Generator
pub struct N8nWorkflowGenerator {
    base_templates: HashMap<String, String>,
}

impl N8nWorkflowGenerator {
    pub fn new() -> Self {
        let mut templates = HashMap::new();
        
        // Temel HTTP istek sablonu
        templates.insert("http_request".into(), r#"{
  "name": "{{name}}",
  "nodes": [
    {
      "parameters": {},
      "name": "Start",
      "type": "n8n-nodes-base.start",
      "typeVersion": 1,
      "position": [250, 300]
    },
    {
      "parameters": {
        "url": "{{url}}",
        "method": "{{method}}",
        "authentication": "{{auth_type}}",
        "options": {}
      },
      "name": "HTTP Request",
      "type": "n8n-nodes-base.httpRequest",
      "typeVersion": 4,
      "position": [450, 300]
    }
  ],
  "connections": {
    "Start": {
      "main": [[{"node": "HTTP Request", "type": "main", "index": 0}]]
    }
  },
  "settings": {},
  "staticData": null
}"#.into());
        
        // Webhook sablonu
        templates.insert("webhook".into(), r#"{
  "name": "{{name}}",
  "nodes": [
    {
      "parameters": {
        "httpMethod": "POST",
        "path": "webhook"
      },
      "name": "Webhook",
      "type": "n8n-nodes-base.webhook",
      "typeVersion": 1,
      "position": [250, 300],
      "webhookId": "{{webhook_id}}"
    },
    {
      "parameters": {
        "functionCode": "return items;"
      },
      "name": "Process",
      "type": "n8n-nodes-base.code",
      "typeVersion": 2,
      "position": [450, 300]
    }
  ],
  "connections": {
    "Webhook": {
      "main": [[{"node": "Process", "type": "main", "index": 0}]]
    }
  }
}"#.into());
        
        // Scheduled job sablonu
        templates.insert("schedule".into(), r#"{
  "name": "{{name}}",
  "nodes": [
    {
      "parameters": {
        "rule": {
          "interval": [{"field": "hours", "hoursInterval": {{interval_hours}}}]
        }
      },
      "name": "Schedule",
      "type": "n8n-nodes-base.scheduleTrigger",
      "typeVersion": 1,
      "position": [250, 300]
    }
  ],
  "connections": {}
}"#.into());
        
        Self { base_templates: templates }
    }
}

impl Generator for N8nWorkflowGenerator {
    fn generate(&self, request: &ForgeRequest) -> SENTIENTResult<String> {
        let template_type = request.parameters.get("template_type")
            .map(|s| s.as_str())
            .unwrap_or("http_request");
        
        let template = self.base_templates.get(template_type)
            .ok_or_else(|| SENTIENTError::General(format!("Bilinmeyen sablon tipi: {}", template_type)))?;
        
        // Parametreleri cikar
        let url = request.parameters.get("url").map(|s| s.as_str()).unwrap_or("");
        let method = request.parameters.get("method").map(|s| s.as_str()).unwrap_or("GET");
        let auth_type = request.parameters.get("auth_type").map(|s| s.as_str()).unwrap_or("none");
        let webhook_id = request.parameters.get("webhook_id").map(|s| s.as_str()).unwrap_or("");
        let interval_hours = request.parameters.get("interval_hours").map(|s| s.as_str()).unwrap_or("1");
        
        // Sablonu doldur
        let mut result = template.clone();
        result = result.replace("{{name}}", &request.name);
        result = result.replace("{{url}}", url);
        result = result.replace("{{method}}", method);
        result = result.replace("{{auth_type}}", auth_type);
        result = result.replace("{{webhook_id}}", webhook_id);
        result = result.replace("{{interval_hours}}", interval_hours);
        
        // JSON validasyonu
        serde_json::from_str::<serde_json::Value>(&result)
            .map_err(|e| SENTIENTError::General(format!("Gecersiz JSON: {}", e)))?;
        
        Ok(result)
    }
    
    fn tool_type(&self) -> ToolType {
        ToolType::N8nWorkflow
    }
}

impl Default for N8nWorkflowGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Python Script Generator
pub struct PythonScriptGenerator;

impl PythonScriptGenerator {
    pub fn new() -> Self {
        Self
    }
}

impl Generator for PythonScriptGenerator {
    fn generate(&self, request: &ForgeRequest) -> SENTIENTResult<String> {
        let script_type = request.parameters.get("script_type")
            .map(|s| s.as_str())
            .unwrap_or("api_client");
        
        let code = match script_type {
            "api_client" => generate_api_client(request),
            "scraper" => generate_scraper(request),
            "processor" => generate_processor(request),
            _ => generate_api_client(request),
        };
        
        Ok(code)
    }
    
    fn tool_type(&self) -> ToolType {
        ToolType::PythonScript
    }
}

impl Default for PythonScriptGenerator {
    fn default() -> Self {
        Self::new()
    }
}

fn generate_api_client(request: &ForgeRequest) -> String {
    let base_url = request.parameters.get("base_url").map(|s| s.as_str()).unwrap_or("");
    let api_key = request.parameters.get("api_key").map(|s| s.as_str()).unwrap_or("");
    
    format!(r#"#!/usr/bin/env python3
"""
{} - SENTIENT Forge tarafindan uretilmistir
Kaynak: {}
"""

import requests
import json
from typing import Dict, Any, Optional

class {}APIClient:
    """API istemci sinifi"""
    
    def __init__(self, base_url: str = "{}", api_key: str = "{}"):
        self.base_url = base_url.rstrip('/')
        self.api_key = api_key
        self.session = requests.Session()
        
        if api_key:
            self.session.headers['Authorization'] = f'Bearer {{api_key}}'
    
    def get(self, endpoint: str, params: Optional[Dict] = None) -> Dict[str, Any]:
        """GET istegi"""
        response = self.session.get(
            f'{{self.base_url}}{{endpoint}}',
            params=params
        )
        response.raise_for_status()
        return response.json()
    
    def post(self, endpoint: str, data: Dict[str, Any]) -> Dict[str, Any]:
        """POST istegi"""
        response = self.session.post(
            f'{{self.base_url}}{{endpoint}}',
            json=data
        )
        response.raise_for_status()
        return response.json()

def main():
    """Ana fonksiyon"""
    client = {}APIClient()
    
    # Ornek kullanim
    try:
        result = client.get('/endpoint')
        print(json.dumps(result, indent=2))
    except requests.RequestException as e:
        print(f"API hatasi: {{e}}")

if __name__ == "__main__":
    main()
"#, request.name, request.description, request.name, base_url, api_key, request.name)
}

fn generate_scraper(request: &ForgeRequest) -> String {
    format!(r#"#!/usr/bin/env python3
"""
{} - Web Scraper
SENTIENT Forge tarafindan uretilmistir
"""

import requests
from bs4 import BeautifulSoup
from typing import List, Dict
import time
import random

class {}Scraper:
    """Web scraper sinifi"""
    
    def __init__(self):
        self.session = requests.Session()
        self.session.headers.update({{
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36'
        }})
    
    def scrape(self, url: str) -> Dict:
        """Sayfa cekme"""
        response = self.session.get(url)
        response.raise_for_status()
        
        soup = BeautifulSoup(response.text, 'html.parser')
        
        return {{
            'title': soup.title.string if soup.title else '',
            'text': soup.get_text(strip=True),
            'links': [a.get('href') for a in soup.find_all('a') if a.get('href')]
        }}
    
    def scrape_multiple(self, urls: List[str], delay: float = 1.0) -> List[Dict]:
        """Coklu sayfa cekme"""
        results = []
        
        for url in urls:
            try:
                data = self.scrape(url)
                results.append(data)
                time.sleep(delay + random.uniform(0, 0.5))
            except requests.RequestException as e:
                print(f"Hata: {{url}} - {{e}}")
        
        return results

def main():
    scraper = {}Scraper()
    # Ornek kullanim
    data = scraper.scrape('https://example.com')
    print(f"Baslik: {{data['title']}}")

if __name__ == "__main__":
    main()
"#, request.name, request.name, request.name)
}

fn generate_processor(request: &ForgeRequest) -> String {
    format!(r#"#!/usr/bin/env python3
"""
{} - Veri Isleci
SENTIENT Forge tarafindan uretilmistir
"""

import json
from typing import List, Dict, Any
from datetime import datetime

class {}Processor:
    """Veri isleme sinifi"""
    
    def __init__(self):
        self.processed_count = 0
    
    def process(self, data: List[Dict]) -> List[Dict]:
        """Veri isleme"""
        processed = []
        
        for item in data:
            processed_item = self._process_item(item)
            processed.append(processed_item)
            self.processed_count += 1
        
        return processed
    
    def _process_item(self, item: Dict) -> Dict:
        """Tek oge isleme"""
        return {{
            'original': item,
            'processed_at': datetime.utcnow().isoformat(),
            'metadata': {{
                'processor': '{}',
                'version': '1.0'
            }}
        }}
    
    def save(self, data: List[Dict], filepath: str):
        """Sonuc kaydetme"""
        with open(filepath, 'w') as f:
            json.dump(data, f, indent=2, default=str)
        print(f"Kaydedildi: {{filepath}} ({{len(data)}} oge)")

def main():
    processor = {}Processor()
    
    # Ornek veri
    sample_data = [
        {{'id': 1, 'name': 'Test'}},
        {{'id': 2, 'name': 'Ornek'}}
    ]
    
    processed = processor.process(sample_data)
    print(f"Islenen: {{len(processed)}} oge")

if __name__ == "__main__":
    main()
"#, request.name, request.name, request.name, request.name)
}

// Factory functions
pub fn create_n8n_generator() -> Box<dyn Generator> {
    Box::new(N8nWorkflowGenerator::new())
}

pub fn create_python_generator() -> Box<dyn Generator> {
    Box::new(PythonScriptGenerator::new())
}

pub fn create_node_generator() -> Box<dyn Generator> {
    Box::new(NodeGenerator)
}

pub fn create_shell_generator() -> Box<dyn Generator> {
    Box::new(ShellGenerator)
}

pub fn create_github_action_generator() -> Box<dyn Generator> {
    Box::new(GitHubActionGenerator)
}

pub fn create_docker_generator() -> Box<dyn Generator> {
    Box::new(DockerGenerator)
}

pub struct NodeGenerator;
pub struct ShellGenerator;
pub struct GitHubActionGenerator;
pub struct DockerGenerator;

impl Generator for NodeGenerator {
    fn generate(&self, request: &ForgeRequest) -> SENTIENTResult<String> {
        Ok(format!("// Node.js module: {}\nmodule.exports = {{}};", request.name))
    }
    fn tool_type(&self) -> ToolType { ToolType::NodeModule }
}

impl Generator for ShellGenerator {
    fn generate(&self, request: &ForgeRequest) -> SENTIENTResult<String> {
        Ok(format!("#!/bin/bash\n# {}\necho 'Hello from {}'", request.name, request.name))
    }
    fn tool_type(&self) -> ToolType { ToolType::ShellScript }
}

impl Generator for GitHubActionGenerator {
    fn generate(&self, request: &ForgeRequest) -> SENTIENTResult<String> {
        Ok(format!(r#"name: {}
on: push
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
"#, request.name))
    }
    fn tool_type(&self) -> ToolType { ToolType::GitHubAction }
}

impl Generator for DockerGenerator {
    fn generate(&self, request: &ForgeRequest) -> SENTIENTResult<String> {
        Ok(format!(r#"version: '3.8'
services:
  {}:
    image: python:3.11
    volumes:
      - ./:/app
    working_dir: /app
"#, request.name.to_lowercase()))
    }
    fn tool_type(&self) -> ToolType { ToolType::DockerCompose }
}
