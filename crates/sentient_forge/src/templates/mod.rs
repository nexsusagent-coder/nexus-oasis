//! ─── SABLON KUTUPHANESI ───

use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Sablon kutuphanesi
#[derive(Debug, Clone)]
pub struct TemplateLibrary {
    templates: HashMap<String, Template>,
    handlebars: Handlebars<'static>,
}

/// Sablon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    /// Sablon adi
    pub name: String,
    /// Sablon tipi
    pub template_type: TemplateType,
    /// Sablon icerigi
    pub content: String,
    /// Varsayilan parametreler
    pub default_params: HashMap<String, String>,
    /// Aciklama
    pub description: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemplateType {
    N8nWorkflow,
    PythonScript,
    NodeModule,
    ShellScript,
    GitHubAction,
    DockerCompose,
}

impl Default for TemplateLibrary {
    fn default() -> Self {
        let mut lib = Self {
            templates: HashMap::new(),
            handlebars: Handlebars::new(),
        };
        
        // Varsayilan sablonlari yukle
        lib.load_defaults();
        lib
    }
}

impl TemplateLibrary {
    /// Sablon yukle
    pub fn load(&mut self, template: Template) {
        self.handlebars.register_template_string(&template.name, &template.content).ok();
        self.templates.insert(template.name.clone(), template);
    }
    
    /// Sablonu al
    pub fn get(&self, name: &str) -> Option<&Template> {
        self.templates.get(name)
    }
    
    /// Sablonu render et
    pub fn render(&self, name: &str, params: &HashMap<String, String>) -> Option<String> {
        self.handlebars.render(name, params).ok()
    }
    
    /// Varsayilan sablonlari yukle
    fn load_defaults(&mut self) {
        // n8n HTTP Request sablonu
        self.load(Template {
            name: "n8n_http_request".into(),
            template_type: TemplateType::N8nWorkflow,
            content: include_str!("../../templates/n8n_http_request.json.hbs").to_string(),
            default_params: [
                ("url".into(), "https://api.example.com".into()),
                ("method".into(), "GET".into()),
            ].into(),
            description: "HTTP request yapan n8n workflow'u".into(),
        });
        
        // Python API scraper sablonu
        self.load(Template {
            name: "python_api_scraper".into(),
            template_type: TemplateType::PythonScript,
            content: include_str!("../../templates/python_api_scraper.py.hbs").to_string(),
            default_params: [
                ("base_url".into(), "https://api.example.com".into()),
            ].into(),
            description: "API scraper Python scripti".into(),
        });
        
        // GitHub Action sablonu
        self.load(Template {
            name: "github_action".into(),
            template_type: TemplateType::GitHubAction,
            content: include_str!("../../templates/github_action.yml.hbs").to_string(),
            default_params: [].into(),
            description: "GitHub Actions workflow'u".into(),
        });
    }
    
    /// Sablon tiplerine gore listele
    pub fn list_by_type(&self, template_type: TemplateType) -> Vec<&Template> {
        self.templates.values()
            .filter(|t| t.template_type == template_type)
            .collect()
    }
}

// Inline default templates when files don't exist
const DEFAULT_N8N_TEMPLATE: &str = r#"{
  "name": "{{name}}",
  "nodes": [
    {
      "name": "Start",
      "type": "n8n-nodes-base.start",
      "position": [250, 300]
    },
    {
      "name": "HTTP Request",
      "type": "n8n-nodes-base.httpRequest",
      "position": [450, 300],
      "parameters": {
        "url": "{{url}}",
        "method": "{{method}}"
      }
    }
  ],
  "connections": {
    "Start": {
      "main": [[{"node": "HTTP Request", "type": "main", "index": 0}]]
    }
  }
}"#;

const DEFAULT_PYTHON_TEMPLATE: &str = r#"#!/usr/bin/env python3
"""
{{name}} - Otomatik uretilen SENTIENT Forge araci
Uretim zamani: {{generated_at}}
"""

import requests
import json
from typing import Dict, Any

BASE_URL = "{{base_url}}"

def main():
    """Ana fonksiyon"""
    print(f"[{{name}}] Baslatiliyor...")
    
    # API istegi
    response = requests.get(BASE_URL)
    
    if response.ok:
        data = response.json()
        print(f"Veri alindi: {len(str(data))} karakter")
        return data
    else:
        print(f"Hata: {response.status_code}")
        return None

if __name__ == "__main__":
    main()
"#;

const DEFAULT_GITHUB_ACTION: &str = r#"name: {{name}}
on:
  push:
    branches: [main]
  pull_request:
    branches: [main]
  schedule:
    - cron: '0 * * * *'

jobs:
  run:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Python
        uses: actions/setup-python@v5
        with:
          python-version: '3.11'
      
      - name: Run script
        run: python scripts/{{script_name}}.py
"#;
