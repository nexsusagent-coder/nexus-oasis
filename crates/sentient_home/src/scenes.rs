//! ─── Scene Management ───

use serde::{Deserialize, Serialize};

/// Scene definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub actions: Vec<SceneAction>,
    pub is_default: bool,
}

impl Scene {
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            icon: None,
            actions: vec![],
            is_default: false,
        }
    }
    
    pub fn with_icon(mut self, icon: &str) -> Self {
        self.icon = Some(icon.to_string());
        self
    }
    
    pub fn add_action(mut self, action: SceneAction) -> Self {
        self.actions.push(action);
        self
    }
}

/// Scene action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneAction {
    pub entity_id: String,
    pub service: String,
    pub data: serde_json::Value,
}

impl SceneAction {
    pub fn turn_on(entity_id: &str) -> Self {
        Self {
            entity_id: entity_id.to_string(),
            service: "turn_on".to_string(),
            data: serde_json::json!({}),
        }
    }
    
    pub fn turn_off(entity_id: &str) -> Self {
        Self {
            entity_id: entity_id.to_string(),
            service: "turn_off".to_string(),
            data: serde_json::json!({}),
        }
    }
    
    pub fn set_brightness(entity_id: &str, brightness: u8) -> Self {
        Self {
            entity_id: entity_id.to_string(),
            service: "turn_on".to_string(),
            data: serde_json::json!({"brightness_pct": brightness}),
        }
    }
    
    pub fn set_temperature(entity_id: &str, temp: f32) -> Self {
        Self {
            entity_id: entity_id.to_string(),
            service: "set_temperature".to_string(),
            data: serde_json::json!({"temperature": temp}),
        }
    }
}

/// Scene manager
pub struct SceneManager {
    scenes: Vec<Scene>,
}

impl SceneManager {
    pub fn new() -> Self {
        Self {
            scenes: default_scenes(),
        }
    }
    
    /// Get all scenes
    pub fn get_scenes(&self) -> &[Scene] {
        &self.scenes
    }
    
    /// Get scene by ID
    pub fn get_scene(&self, id: &str) -> Option<&Scene> {
        self.scenes.iter().find(|s| s.id == id)
    }
    
    /// Find scene by name (fuzzy match)
    pub fn find_scene(&self, name: &str) -> Option<&Scene> {
        let name_lower = name.to_lowercase();
        
        self.scenes.iter().find(|s| {
            s.name.to_lowercase() == name_lower ||
            s.id.to_lowercase().contains(&name_lower) ||
            name_lower.contains(&s.id.to_lowercase())
        })
    }
    
    /// Add custom scene
    pub fn add_scene(&mut self, scene: Scene) {
        self.scenes.push(scene);
    }
}

impl Default for SceneManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Default scenes
fn default_scenes() -> Vec<Scene> {
    vec![
        // Good Morning
        Scene::new("good_morning", "Good Morning")
            .with_icon("🌅")
            .add_action(SceneAction::turn_on("light.living_room"))
            .add_action(SceneAction::set_brightness("light.living_room", 80))
            .add_action(SceneAction::set_temperature("climate.home", 22.0)),
        
        // Good Night
        Scene::new("good_night", "Good Night")
            .with_icon("🌙")
            .add_action(SceneAction::turn_off("light.living_room"))
            .add_action(SceneAction::turn_off("light.bedroom"))
            .add_action(SceneAction::set_temperature("climate.home", 19.0)),
        
        // Movie Mode
        Scene::new("movie", "Movie Mode")
            .with_icon("🎬")
            .add_action(SceneAction::set_brightness("light.living_room", 20))
            .add_action(SceneAction::turn_off("light.kitchen")),
        
        // Focus Mode
        Scene::new("focus", "Focus Mode")
            .with_icon("🎯")
            .add_action(SceneAction::set_brightness("light.office", 100))
            .add_action(SceneAction::turn_off("light.living_room")),
        
        // Away Mode
        Scene::new("away", "Away Mode")
            .with_icon("🚪")
            .add_action(SceneAction::turn_off("light.living_room"))
            .add_action(SceneAction::turn_off("light.bedroom"))
            .add_action(SceneAction::turn_off("light.kitchen"))
            .add_action(SceneAction::set_temperature("climate.home", 18.0)),
        
        // Party Mode
        Scene::new("party", "Party Mode")
            .with_icon("🎉")
            .add_action(SceneAction::set_brightness("light.living_room", 100))
            .add_action(SceneAction::turn_on("light.led_strip")),
        
        // Reading Mode
        Scene::new("reading", "Reading Mode")
            .with_icon("📖")
            .add_action(SceneAction::set_brightness("light.bedroom", 70)),
        
        // Relax Mode
        Scene::new("relax", "Relax Mode")
            .with_icon("🧘")
            .add_action(SceneAction::set_brightness("light.living_room", 40))
            .add_action(SceneAction::set_temperature("climate.home", 21.0)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_find_scene() {
        let manager = SceneManager::new();
        
        let scene = manager.find_scene("movie");
        assert!(scene.is_some());
        
        let scene = manager.find_scene("Movie Mode");
        assert!(scene.is_some());
    }
    
    #[test]
    fn test_scene_actions() {
        let action = SceneAction::set_brightness("light.test", 50);
        assert_eq!(action.entity_id, "light.test");
    }
}
