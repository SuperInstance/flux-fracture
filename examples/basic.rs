//! Basic fracture example — create a dependency graph, fracture it, print results.

use flux_fracture::{DependencyGraph, Fracturer};

fn main() {
    println!("=== flux-fracture: Basic Example ===\n");

    // Build an identity graph: 8 constraints, each on its own dimension
    let graph = DependencyGraph::identity(8);
    println!("Graph: {} constraints × {} dimensions", graph.n_constraints, graph.n_dimensions);

    // Fracture into independent blocks
    let fracturer = Fracturer::new();
    let result = fracturer.fracture(&graph);

    println!("\nFracture result:");
    println!("  Blocks:              {}", result.n_blocks);
    println!("  Largest block size:  {}", result.largest_block_size);
    println!("  Speedup potential:   {:.1}×", result.speedup_potential);

    println!("\nBlocks:");
    for (i, block) in result.blocks.iter().enumerate() {
        println!(
            "  Block {}: constraints {:?}, dimensions {:?} (size {})",
            i, block.constraint_indices, block.dimension_indices, block.size
        );
    }

    // Try a denser graph
    println!("\n--- Block diagonal (2×4) ---");
    let mut bd = DependencyGraph::identity(8);
    bd.fill_block(0, 4, 0, 4);
    bd.fill_block(4, 8, 4, 8);

    let result2 = fracturer.fracture(&bd);
    println!("  Blocks: {}", result2.n_blocks);
    println!("  Speedup: {:.1}×", result2.speedup_potential);
}
