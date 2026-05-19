//! # flux-fracture
//!
//! FRACTURE-COALESCE: Disjoint Linear Algebra for Constraint Systems.
//!
//! If constraints are independent, the partition function factorizes Z = Π(Zᵢ).
//! This crate makes that **operational** by fracturing constraint systems into
//! independent blocks and coalescing results provably correct.
//!
//! ## Theorem
//!
//! If fracture correctly identifies connected components of the
//! constraint–dimension dependency graph, coalescence via bitwise OR
//! preserves zero false negatives.
//!
//! **Proof:** Each constraint violation is a Boolean event. For independent blocks,
//! the event spaces are disjoint (no shared dimensions). The union of all
//! violations = OR of block error masks. QED.

pub mod graph;
pub mod coalesce;

pub use graph::{DependencyGraph, FractureResult, Block, Fracturer, AdaptiveFracturer, FractureDelta};
pub use coalesce::Coalescer;
