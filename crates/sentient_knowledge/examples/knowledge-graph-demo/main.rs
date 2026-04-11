//! ─── Knowledge Graph Demo ───
//!
//! Demonstrates Knowledge Graph functionality.

use sentient_knowledge::{
    Entity, EntityType, KnowledgeGraph, Relation,
    RELATES_TO, PART_OF, DEPENDS_ON, KNOWS, WORKS_FOR,
};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║       SENTIENT OS - Knowledge Graph Demo                 ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!();

    // Create in-memory knowledge graph
    let kg = KnowledgeGraph::in_memory("demo");
    println!("✅ Created Knowledge Graph: '{}'", kg.name());
    println!();

    // ─── Create Entities ───
    println!("📝 Creating entities...");
    println!();

    // People
    let alice = Entity::new("Alice")
        .with_type(EntityType::Person)
        .with_description("Senior Software Engineer")
        .with_tag("rust")
        .with_tag("python");

    let bob = Entity::new("Bob")
        .with_type(EntityType::Person)
        .with_description("ML Engineer")
        .with_tag("python")
        .with_tag("tensorflow");

    let charlie = Entity::new("Charlie")
        .with_type(EntityType::Person)
        .with_description("Product Manager");

    // Organizations
    let acme = Entity::new("Acme Corp")
        .with_type(EntityType::Organization)
        .with_description("Technology company");

    // Tools
    let rust = Entity::new("Rust")
        .with_type(EntityType::Tool)
        .with_description("Systems programming language")
        .with_tag("programming");

    let python = Entity::new("Python")
        .with_type(EntityType::Tool)
        .with_description("High-level programming language")
        .with_tag("programming");

    let tensorflow = Entity::new("TensorFlow")
        .with_type(EntityType::Tool)
        .with_description("Machine learning framework")
        .with_tag("ml");

    // Topics
    let ml = Entity::new("Machine Learning")
        .with_type(EntityType::Topic)
        .with_description("AI and ML techniques");

    // Add entities
    let alice_id = kg.add_entity(alice).await.expect("Failed to add Alice");
    let bob_id = kg.add_entity(bob).await.expect("Failed to add Bob");
    let charlie_id = kg.add_entity(charlie).await.expect("Failed to add Charlie");
    let acme_id = kg.add_entity(acme).await.expect("Failed to add Acme");
    let rust_id = kg.add_entity(rust).await.expect("Failed to add Rust");
    let python_id = kg.add_entity(python).await.expect("Failed to add Python");
    let tensorflow_id = kg.add_entity(tensorflow).await.expect("Failed to add TensorFlow");
    let ml_id = kg.add_entity(ml).await.expect("Failed to add ML");

    println!("   Added {} entities", kg.stats().total_entities);
    println!();

    // ─── Create Relations ───
    println!("🔗 Creating relations...");
    println!();

    // Person -> Organization
    kg.add_relation(Relation::new(alice_id, acme_id, WORKS_FOR)).await.expect("Failed");
    kg.add_relation(Relation::new(bob_id, acme_id, WORKS_FOR)).await.expect("Failed");
    kg.add_relation(Relation::new(charlie_id, acme_id, WORKS_FOR)).await.expect("Failed");

    // Person -> Person
    kg.add_relation(Relation::new(alice_id, bob_id, KNOWS)).await.expect("Failed");
    kg.add_relation(Relation::new(bob_id, charlie_id, KNOWS)).await.expect("Failed");

    // Person -> Tool
    kg.add_relation(Relation::new(alice_id, rust_id, "uses")).await.expect("Failed");
    kg.add_relation(Relation::new(alice_id, python_id, "uses")).await.expect("Failed");
    kg.add_relation(Relation::new(bob_id, python_id, "uses")).await.expect("Failed");
    kg.add_relation(Relation::new(bob_id, tensorflow_id, "uses")).await.expect("Failed");

    // Tool -> Topic
    kg.add_relation(Relation::new(tensorflow_id, ml_id, PART_OF)).await.expect("Failed");
    kg.add_relation(Relation::new(python_id, ml_id, RELATES_TO)).await.expect("Failed");

    // Tool -> Tool
    kg.add_relation(Relation::new(tensorflow_id, python_id, DEPENDS_ON)).await.expect("Failed");

    println!("   Added {} relations", kg.stats().total_relations);
    println!();

    // ─── Query: Find Path ───
    println!("🔍 Finding path: Alice → TensorFlow");
    let path = kg.find_path(alice_id, tensorflow_id, 5).await.expect("Path query failed");
    
    match path {
        Some(path) => {
            println!("   Path found ({} hops):", path.len() - 1);
            for (i, id) in path.iter().enumerate() {
                if let Some(entity) = kg.get_entity(*id).await.expect("Get failed") {
                    println!("   {} {}", i, entity.name);
                }
            }
        }
        None => println!("   No path found"),
    }
    println!();

    // ─── Query: Subgraph ───
    println!("🔍 Getting subgraph around Bob (depth=2)...");
    let subgraph = kg.subgraph(bob_id, 2).await.expect("Subgraph failed");
    
    println!("   Entities:");
    for entity in &subgraph.entities {
        println!("   - {} ({:?})", entity.name, entity.entity_type);
    }
    
    println!();
    println!("   Relations:");
    for rel in &subgraph.relations {
        let from = kg.get_entity(rel.from_id).await.unwrap().map(|e| e.name).unwrap_or_default();
        let to = kg.get_entity(rel.to_id).await.unwrap().map(|e| e.name).unwrap_or_default();
        println!("   - {} → {} → {}", from, rel.relation_type, to);
    }
    println!();

    // ─── Graph RAG ───
    println!("🧠 Graph RAG Context:");
    let rag = kg.rag();
    let context = rag.retrieve_from_entities(&[alice_id, bob_id]).await.expect("RAG failed");
    
    println!("{}", context.text);

    // ─── Stats ───
    println!("📊 Statistics:");
    let stats = kg.stats();
    println!("   Entities: {}", stats.total_entities);
    println!("   Relations: {}", stats.total_relations);
    println!("   Queries: {}", stats.queries_executed);
    println!();

    println!("✅ Demo complete!");
}
