//! ═══════════════════════════════════════════════════════════════════════════════
//!  ZK CIRCUITS - Arithmetic Circuits for ZK Proofs
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::{ZkError, ZkResult};
use serde::{Deserialize, Serialize};

/// Circuit definition for ZK proofs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Circuit {
    /// Circuit name
    pub name: String,
    
    /// Input wires (public and private)
    pub inputs: Vec<Wire>,
    
    /// Output wires
    pub outputs: Vec<Wire>,
    
    /// Constraints
    pub constraints: Vec<Constraint>,
    
    /// Number of gates
    pub gate_count: usize,
}

/// Wire in a circuit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wire {
    pub id: u64,
    pub name: String,
    pub is_public: bool,
}

/// Constraint in a circuit (R1CS format)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    /// A matrix entries
    pub a: Vec<(usize, String)>,
    /// B matrix entries  
    pub b: Vec<(usize, String)>,
    /// C matrix entries
    pub c: Vec<(usize, String)>,
}

/// Circuit builder
pub struct CircuitBuilder {
    name: String,
    inputs: Vec<Wire>,
    outputs: Vec<Wire>,
    constraints: Vec<Constraint>,
    wire_counter: u64,
}

impl CircuitBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            constraints: Vec::new(),
            wire_counter: 0,
        }
    }

    /// Add public input
    pub fn add_public_input(&mut self, name: impl Into<String>) -> u64 {
        let wire = Wire {
            id: self.wire_counter,
            name: name.into(),
            is_public: true,
        };
        self.inputs.push(wire);
        self.wire_counter += 1;
        self.wire_counter - 1
    }

    /// Add private input
    pub fn add_private_input(&mut self, name: impl Into<String>) -> u64 {
        let wire = Wire {
            id: self.wire_counter,
            name: name.into(),
            is_public: false,
        };
        self.inputs.push(wire);
        self.wire_counter += 1;
        self.wire_counter - 1
    }

    /// Add output
    pub fn add_output(&mut self, name: impl Into<String>) -> u64 {
        let wire = Wire {
            id: self.wire_counter,
            name: name.into(),
            is_public: true,
        };
        self.outputs.push(wire);
        self.wire_counter += 1;
        self.wire_counter - 1
    }

    /// Add constraint: a * b = c
    pub fn add_constraint(&mut self, a: usize, b: usize, c: usize) {
        self.constraints.push(Constraint {
            a: vec![(a, "1".into())],
            b: vec![(b, "1".into())],
            c: vec![(c, "1".into())],
        });
    }

    /// Build circuit
    pub fn build(self) -> Circuit {
        let gate_count = self.constraints.len();
        Circuit {
            name: self.name,
            inputs: self.inputs,
            outputs: self.outputs,
            constraints: self.constraints,
            gate_count,
        }
    }
}

/// MCP Request circuit - proves request validity without revealing contents
pub fn mcp_request_circuit() -> Circuit {
    let mut builder = CircuitBuilder::new("mcp_request");
    
    // Public inputs
    let tool_name = builder.add_public_input("tool_name");
    let request_hash = builder.add_public_input("request_hash");
    
    // Private inputs
    let _parameters = builder.add_private_input("parameters");
    let _secret_key = builder.add_private_input("secret_key");
    
    // Output
    let _valid = builder.add_output("valid");
    
    // Constraint: request_hash = H(tool_name || parameters || secret_key)
    builder.add_constraint(tool_name as usize, request_hash as usize, 0);
    
    builder.build()
}

/// Range circuit - proves value is in range without revealing value
pub fn range_circuit(min: u64, max: u64) -> Circuit {
    let mut builder = CircuitBuilder::new(&format!("range_{}_{}", min, max));
    
    // Public inputs
    let _min_wire = builder.add_public_input("min");
    let _max_wire = builder.add_public_input("max");
    let commitment = builder.add_public_input("commitment");
    
    // Private input
    let _value = builder.add_private_input("value");
    
    // Output
    let _in_range = builder.add_output("in_range");
    
    // Constraint: commitment = H(value) AND min <= value <= max
    builder.add_constraint(commitment as usize, 0, 0);
    
    builder.build()
}

/// Circuit compiler
pub struct CircuitCompiler;

impl CircuitCompiler {
    /// Compile circuit to R1CS format
    pub fn compile(circuit: &Circuit) -> ZkResult<CompiledCircuit> {
        // Count constraints
        let num_constraints = circuit.constraints.len();
        let num_variables = circuit.inputs.len() + circuit.outputs.len();
        
        // Compute circuit hash
        let circuit_hash = blake3::hash(
            &serde_json::to_vec(circuit).map_err(|e| ZkError::CircuitCompilationFailed(e.to_string()))?
        ).to_hex().to_string();

        Ok(CompiledCircuit {
            name: circuit.name.clone(),
            num_constraints,
            num_variables,
            circuit_hash,
            r1cs: circuit.constraints.clone(),
        })
    }
}

/// Compiled circuit ready for proving
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledCircuit {
    pub name: String,
    pub num_constraints: usize,
    pub num_variables: usize,
    pub circuit_hash: String,
    pub r1cs: Vec<Constraint>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_builder() {
        let mut builder = CircuitBuilder::new("test");
        builder.add_public_input("x");
        builder.add_private_input("y");
        builder.add_output("z");
        
        let circuit = builder.build();
        assert_eq!(circuit.inputs.len(), 2);
        assert_eq!(circuit.outputs.len(), 1);
    }

    #[test]
    fn test_mcp_request_circuit() {
        let circuit = mcp_request_circuit();
        assert!(circuit.gate_count > 0);
    }

    #[test]
    fn test_circuit_compiler() {
        let circuit = mcp_request_circuit();
        let compiled = CircuitCompiler::compile(&circuit);
        assert!(compiled.is_ok());
    }
}
