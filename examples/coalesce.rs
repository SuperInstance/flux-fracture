//! Coalesce example — fracture-coalesce with block diagonal structure,
//! verifying zero false negatives across all blocks.

use flux_fracture::{Coalescer, DependencyGraph, Fracturer};

fn main() {
    println!("=== flux-fracture: Coalesce Example ===\n");

    // Build a block-diagonal graph: 2 blocks of 4 constraints each
    let mut graph = DependencyGraph::identity(8);
    graph.fill_block(0, 4, 0, 4);
    graph.fill_block(4, 8, 4, 8);

    let fracturer = Fracturer::new();
    let result = fracturer.fracture(&graph);
    println!("Fractured into {} blocks", result.n_blocks);

    // Simulate block-level error masks
    // Block 0 (constraints 0-3): constraints 1 and 3 violated
    // Block 1 (constraints 4-7): constraint 5 violated
    let block_masks: Vec<u64> = vec![
        0b1010, // block 0: bits 1,3 set → constraints 1,3 violated
        0b0010, // block 1: bit 1 set → constraint 5 violated
    ];

    let block_indices: Vec<Vec<usize>> = result
        .blocks
        .iter()
        .map(|b| b.constraint_indices.clone())
        .collect();

    // Coalesce
    let coalescer = Coalescer::new();
    let coalesced = coalescer.coalesce_masks(&block_masks);
    println!("\nBlock masks: {:?}", block_masks);
    println!("Coalesced:   0b{:08b}", coalesced);

    // Build expected monolithic mask manually:
    // Block 0 bit 1 → global constraint 1
    // Block 0 bit 3 → global constraint 3
    // Block 1 bit 1 → global constraint 5
    let monolithic: u64 = (1 << 1) | (1 << 3) | (1 << 5); // 0b00101010 = 42
    println!("Monolithic:  0b{:08b}", monolithic);

    // Verify correctness
    let (ok, msg) = coalescer.verify_coalescence(&block_masks, &block_indices, monolithic);
    println!("\nVerification: {} — {}", if ok { "✓ PASS" } else { "✗ FAIL" }, msg);

    // Also demonstrate array coalescence
    println!("\n--- Array coalescence ---");
    let a = vec![1, 0, 1, 0];
    let b = vec![0, 1, 0, 0];
    let c = vec![0, 0, 1, 1];
    let merged = coalescer.coalesce_arrays(&[a.clone(), b.clone(), c.clone()]);
    println!("  {:?} OR {:?} OR {:?} = {:?}", a, b, c, merged);
}
