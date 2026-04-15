//! ═══════════════════════════════════════════════════════════════════════════════
//!  Skill Dependency System
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//! Manages dependencies between skills:
//! - Dependency declaration
//! - Version constraints
//! - Circular dependency detection
//! - Dependency resolution order
//! - Auto-installation of dependencies

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use chrono::{DateTime, Utc};

// ═══════════════════════════════════════════════════════════════════════════════
//  VERSION CONSTRAINTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Semantic version
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub pre: Option<String>,
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch, pre: None }
    }
    
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim_start_matches('v');
        let parts: Vec<&str> = s.split('.').collect();
        
        if parts.len() < 3 {
            return None;
        }
        
        let major = parts[0].parse().ok()?;
        let minor = parts[1].parse().ok()?;
        let patch_str = parts[2];
        
        let (patch, pre) = if let Some(idx) = patch_str.find('-') {
            (patch_str[..idx].parse().ok()?, Some(patch_str[idx+1..].to_string()))
        } else {
            (patch_str.parse().ok()?, None)
        };
        
        Some(Self { major, minor, patch, pre })
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if let Some(ref pre) = self.pre {
            write!(f, "-{}", pre)?;
        }
        Ok(())
    }
}

/// Version constraint (semver range)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VersionConstraint {
    /// Exact version
    Exact(Version),
    /// Major version (e.g., ^1.0.0)
    Caret(Version),
    /// Compatible version (e.g., ~1.2.0)
    Tilde(Version),
    /// Range (>=1.0.0, <2.0.0)
    Range { min: Option<Version>, max: Option<Version> },
    /// Any version
    Any,
}

impl VersionConstraint {
    /// Parse a constraint string
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim();
        
        if s == "*" {
            return Some(Self::Any);
        }
        
        if s.starts_with('^') {
            let version = Version::parse(&s[1..])?;
            return Some(Self::Caret(version));
        }
        
        if s.starts_with('~') {
            let version = Version::parse(&s[1..])?;
            return Some(Self::Tilde(version));
        }
        
        if s.starts_with(">=<") || s.contains(',') {
            return Self::parse_range(s);
        }
        
        let version = Version::parse(s)?;
        Some(Self::Exact(version))
    }
    
    fn parse_range(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split(',').map(|p| p.trim()).collect();
        let mut min = None;
        let mut max = None;
        
        for part in parts {
            if part.starts_with(">=") {
                min = Some(Version::parse(&part[2..])?);
            } else if part.starts_with("<=") {
                max = Some(Version::parse(&part[2..])?);
            } else if part.starts_with('>') {
                // > means >= next patch
                let v = Version::parse(&part[1..])?;
                min = Some(Version::new(v.major, v.minor, v.patch + 1));
            } else if part.starts_with('<') {
                max = Some(Version::parse(&part[1..])?);
            }
        }
        
        Some(Self::Range { min, max })
    }
    
    /// Check if version satisfies constraint
    pub fn satisfies(&self, version: &Version) -> bool {
        match self {
            Self::Exact(v) => v == version,
            
            Self::Caret(v) => {
                // ^1.2.3 := >=1.2.3, <2.0.0 (for major > 0)
                // ^0.2.3 := >=0.2.3, <0.3.0 (for major = 0)
                version >= v && (
                    v.major > 0 && version.major == v.major ||
                    v.major == 0 && version.major == 0 && version.minor == v.minor
                )
            }
            
            Self::Tilde(v) => {
                // ~1.2.3 := >=1.2.3, <1.3.0
                version >= v && version.major == v.major && version.minor == v.minor
            }
            
            Self::Range { min, max } => {
                let min_ok = min.as_ref().map_or(true, |m| version >= m);
                let max_ok = max.as_ref().map_or(true, |m| version < m);
                min_ok && max_ok
            }
            
            Self::Any => true,
        }
    }
}

impl std::fmt::Display for VersionConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Exact(v) => write!(f, "{}", v),
            Self::Caret(v) => write!(f, "^{}", v),
            Self::Tilde(v) => write!(f, "~{}", v),
            Self::Range { min, max } => {
                if let Some(min) = min {
                    write!(f, ">={}", min)?;
                }
                if let Some(max) = max {
                    if min.is_some() { write!(f, ", ")?; }
                    write!(f, "<{}", max)?;
                }
                Ok(())
            }
            Self::Any => write!(f, "*"),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  DEPENDENCY DECLARATION
// ═══════════════════════════════════════════════════════════════════════════════

/// A single dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    /// Skill ID or name
    pub skill: String,
    /// Version constraint
    pub version: VersionConstraint,
    /// Is this an optional dependency?
    pub optional: bool,
    /// Features to enable
    pub features: Vec<String>,
    /// Alias for the dependency
    pub alias: Option<String>,
}

impl Dependency {
    pub fn new(skill: impl Into<String>) -> Self {
        Self {
            skill: skill.into(),
            version: VersionConstraint::Any,
            optional: false,
            features: Vec::new(),
            alias: None,
        }
    }
    
    pub fn with_version(mut self, constraint: &str) -> Self {
        if let Some(v) = VersionConstraint::parse(constraint) {
            self.version = v;
        }
        self
    }
    
    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }
    
    pub fn with_features(mut self, features: Vec<String>) -> Self {
        self.features = features;
        self
    }
    
    pub fn with_alias(mut self, alias: impl Into<String>) -> Self {
        self.alias = Some(alias.into());
        self
    }
    
    /// Check if version satisfies this dependency
    pub fn satisfies(&self, version: &Version) -> bool {
        self.version.satisfies(version)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  DEPENDENCY GRAPH
// ═══════════════════════════════════════════════════════════════════════════════

/// Dependency error
#[derive(Debug, thiserror::Error)]
pub enum DependencyError {
    #[error("Circular dependency detected: {0}")]
    CircularDependency(String),
    
    #[error("Missing dependency: {0}")]
    MissingDependency(String),
    
    #[error("Version conflict: {skill} requires {required} but {found} is installed")]
    VersionConflict { skill: String, required: String, found: String },
    
    #[error("Dependency resolution failed: {0}")]
    ResolutionFailed(String),
}

/// Dependency node in graph
#[derive(Debug, Clone)]
pub struct DependencyNode {
    pub skill_id: String,
    pub version: Version,
    pub dependencies: Vec<Dependency>,
    pub depth: usize,
}

/// Dependency graph for resolution
pub struct DependencyGraph {
    nodes: HashMap<String, DependencyNode>,
    edges: HashMap<String, Vec<String>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }
    
    /// Add a skill to the graph
    pub fn add_skill(
        &mut self, 
        skill_id: String, 
        version: Version, 
        dependencies: Vec<Dependency>
    ) {
        let node = DependencyNode {
            skill_id: skill_id.clone(),
            version,
            dependencies,
            depth: 0,
        };
        
        self.nodes.insert(skill_id.clone(), node);
        self.edges.insert(skill_id, Vec::new());
    }
    
    /// Build edges from dependencies
    pub fn build_edges(&mut self) {
        let skill_ids: Vec<String> = self.nodes.keys().cloned().collect();
        
        for skill_id in skill_ids {
            if let Some(node) = self.nodes.get(&skill_id) {
                let deps: Vec<String> = node.dependencies.iter()
                    .map(|d| d.skill.clone())
                    .collect();
                
                for dep_id in deps {
                    if self.nodes.contains_key(&dep_id) {
                        self.edges.entry(skill_id.clone())
                            .or_insert_with(Vec::new)
                            .push(dep_id);
                    }
                }
            }
        }
    }
    
    /// Detect circular dependencies
    pub fn detect_cycles(&self) -> Result<(), DependencyError> {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        
        for skill_id in self.nodes.keys() {
            if !visited.contains(skill_id) {
                self.detect_cycle_dfs(skill_id, &mut visited, &mut rec_stack)?;
            }
        }
        
        Ok(())
    }
    
    fn detect_cycle_dfs(
        &self,
        skill_id: &str,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
    ) -> Result<(), DependencyError> {
        visited.insert(skill_id.to_string());
        rec_stack.insert(skill_id.to_string());
        
        if let Some(deps) = self.edges.get(skill_id) {
            for dep in deps {
                if !visited.contains(dep) {
                    self.detect_cycle_dfs(dep, visited, rec_stack)?;
                } else if rec_stack.contains(dep) {
                    return Err(DependencyError::CircularDependency(
                        format!("{} -> {}", skill_id, dep)
                    ));
                }
            }
        }
        
        rec_stack.remove(skill_id);
        Ok(())
    }
    
    /// Topological sort - returns skills in dependency order
    pub fn topological_sort(&self) -> Result<Vec<String>, DependencyError> {
        self.detect_cycles()?;
        
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        
        // Initialize in-degrees
        for skill_id in self.nodes.keys() {
            in_degree.insert(skill_id.clone(), 0);
        }
        
        // Calculate in-degrees
        for deps in self.edges.values() {
            for dep in deps {
                *in_degree.entry(dep.clone()).or_insert(0) += 1;
            }
        }
        
        // Find nodes with no dependencies
        for (skill_id, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(skill_id.clone());
            }
        }
        
        // Process queue
        while let Some(skill_id) = queue.pop_front() {
            result.push(skill_id.clone());
            
            if let Some(deps) = self.edges.get(&skill_id) {
                for dep in deps {
                    let degree = in_degree.get_mut(dep).unwrap();
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(dep.clone());
                    }
                }
            }
        }
        
        Ok(result)
    }
    
    /// Get all dependencies of a skill (transitive)
    pub fn get_all_dependencies(&self, skill_id: &str) -> HashSet<String> {
        let mut deps = HashSet::new();
        self.collect_dependencies(skill_id, &mut deps);
        deps
    }
    
    fn collect_dependencies(&self, skill_id: &str, deps: &mut HashSet<String>) {
        if let Some(node) = self.nodes.get(skill_id) {
            for dep in &node.dependencies {
                if !deps.contains(&dep.skill) {
                    deps.insert(dep.skill.clone());
                    self.collect_dependencies(&dep.skill, deps);
                }
            }
        }
    }
    
    /// Get dependents of a skill (who depends on it)
    pub fn get_dependents(&self, skill_id: &str) -> Vec<String> {
        let mut dependents = Vec::new();
        
        for (id, deps) in &self.edges {
            if deps.iter().any(|d| d == skill_id) {
                dependents.push(id.clone());
            }
        }
        
        dependents
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  DEPENDENCY RESOLVER
// ═══════════════════════════════════════════════════════════════════════════════

/// Resolved dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedDependency {
    pub skill_id: String,
    pub version: Version,
    pub source: String,
    pub required_by: Vec<String>,
}

/// Dependency resolver
pub struct DependencyResolver {
    installed: HashMap<String, Version>,
    graph: DependencyGraph,
}

impl DependencyResolver {
    pub fn new() -> Self {
        Self {
            installed: HashMap::new(),
            graph: DependencyGraph::new(),
        }
    }
    
    /// Add an installed skill
    pub fn add_installed(&mut self, skill_id: String, version: Version) {
        self.installed.insert(skill_id, version);
    }
    
    /// Resolve dependencies for a skill
    pub fn resolve(
        &mut self,
        skill_id: &str,
        version: &Version,
        dependencies: Vec<Dependency>,
    ) -> Result<Vec<ResolvedDependency>, DependencyError> {
        // Add skill to graph
        self.graph.add_skill(skill_id.to_string(), version.clone(), dependencies.clone());
        
        // Build edges
        self.graph.build_edges();
        
        // Check for cycles
        self.graph.detect_cycles()?;
        
        // Get resolution order
        let order = self.graph.topological_sort()?;
        
        // Resolve each dependency
        let mut resolved = Vec::new();
        
        for dep_skill_id in order {
            if dep_skill_id == skill_id {
                continue; // Skip the root skill
            }
            
            let installed_version = self.installed.get(&dep_skill_id).cloned();
            
            // Find the dependency constraint
            let constraint = dependencies.iter()
                .find(|d| d.skill == dep_skill_id)
                .map(|d| d.version.clone());
            
            if let Some(version) = installed_version {
                // Check if installed version satisfies constraint
                if let Some(ref constraint) = constraint {
                    if !constraint.satisfies(&version) {
                        return Err(DependencyError::VersionConflict {
                            skill: dep_skill_id.clone(),
                            required: constraint.to_string(),
                            found: version.to_string(),
                        });
                    }
                }
                
                let skill_id = dep_skill_id.clone();
                let required_by = self.graph.get_dependents(&dep_skill_id);
                
                resolved.push(ResolvedDependency {
                    skill_id,
                    version,
                    source: "installed".to_string(),
                    required_by,
                });
            } else {
                // Need to install - for now just add as missing
                return Err(DependencyError::MissingDependency(dep_skill_id));
            }
        }
        
        Ok(resolved)
    }
    
    /// Check if a skill can be safely removed
    pub fn can_remove(&self, skill_id: &str) -> Result<(), DependencyError> {
        let dependents = self.graph.get_dependents(skill_id);
        
        if !dependents.is_empty() {
            return Err(DependencyError::ResolutionFailed(
                format!("Cannot remove: required by {}", dependents.join(", "))
            ));
        }
        
        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_version_parsing() {
        let v = Version::parse("1.2.3").unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
        
        let v2 = Version::parse("2.0.0-beta").unwrap();
        assert_eq!(v2.pre, Some("beta".to_string()));
    }
    
    #[test]
    fn test_version_constraint_caret() {
        let constraint = VersionConstraint::parse("^1.2.3").unwrap();
        
        assert!(constraint.satisfies(&Version::new(1, 2, 3)));
        assert!(constraint.satisfies(&Version::new(1, 2, 4)));
        assert!(constraint.satisfies(&Version::new(1, 9, 9)));
        assert!(!constraint.satisfies(&Version::new(2, 0, 0)));
        assert!(!constraint.satisfies(&Version::new(1, 2, 2)));
    }
    
    #[test]
    fn test_version_constraint_tilde() {
        let constraint = VersionConstraint::parse("~1.2.3").unwrap();
        
        assert!(constraint.satisfies(&Version::new(1, 2, 3)));
        assert!(constraint.satisfies(&Version::new(1, 2, 9)));
        assert!(!constraint.satisfies(&Version::new(1, 3, 0)));
    }
    
    #[test]
    fn test_dependency_satisfies() {
        let dep = Dependency::new("test-skill")
            .with_version("^1.0.0");
        
        assert!(dep.satisfies(&Version::new(1, 5, 0)));
        assert!(!dep.satisfies(&Version::new(2, 0, 0)));
    }
    
    #[test]
    fn test_circular_detection() {
        let mut graph = DependencyGraph::new();
        
        graph.add_skill("a".to_string(), Version::new(1, 0, 0), 
            vec![Dependency::new("b")]);
        graph.add_skill("b".to_string(), Version::new(1, 0, 0),
            vec![Dependency::new("c")]);
        graph.add_skill("c".to_string(), Version::new(1, 0, 0),
            vec![Dependency::new("a")]); // Cycle!
        
        graph.build_edges();
        
        assert!(graph.detect_cycles().is_err());
    }
    
    #[test]
    fn test_topological_sort() {
        let mut graph = DependencyGraph::new();
        
        graph.add_skill("a".to_string(), Version::new(1, 0, 0),
            vec![Dependency::new("b"), Dependency::new("c")]);
        graph.add_skill("b".to_string(), Version::new(1, 0, 0), vec![]);
        graph.add_skill("c".to_string(), Version::new(1, 0, 0),
            vec![Dependency::new("b")]);
        
        graph.build_edges();
        
        let order = graph.topological_sort().unwrap();
        
        // b must come before c and a
        let b_idx = order.iter().position(|s| s == "b").unwrap();
        let c_idx = order.iter().position(|s| s == "c").unwrap();
        
        assert!(b_idx < c_idx);
    }
}
