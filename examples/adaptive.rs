//! Adaptive example — AdaptiveFracturer detecting structure changes in real time.

use flux_fracture::{AdaptiveFracturer, DependencyGraph};

fn main() {
    println!("=== flux-fracture: Adaptive Example ===\n");

    let mut adaptive = AdaptiveFracturer::new();

    // Phase 1: Start with fully independent system
    println!("Phase 1: Fully independent (8 isolated constraints)");
    let g1 = DependencyGraph::identity(8);
    let (res, delta) = adaptive.update(&g1);
    println!(
        "  Blocks: {} | Changed: {} | Speedup: {:.1}×",
        res.n_blocks, delta.structure_changed, res.speedup_potential
    );

    // Phase 2: Same graph again — no change detected
    println!("\nPhase 2: Same graph (no change expected)");
    let (_res, delta) = adaptive.update(&g1);
    println!("  Changed: {} | Refracture count: {}", delta.structure_changed, adaptive.refracture_count);

    // Phase 3: Collapse to block diagonal — structure changes
    println!("\nPhase 3: Collapse to block diagonal (2×4)");
    let mut g2 = DependencyGraph::identity(8);
    g2.fill_block(0, 4, 0, 4);
    g2.fill_block(4, 8, 4, 8);
    let (res, delta) = adaptive.update(&g2);
    println!(
        "  Blocks: {} | Changed: {} | Largest: {} | Speedup: {:.1}×",
        res.n_blocks, delta.structure_changed, res.largest_block_size, res.speedup_potential
    );
    println!(
        "  Block size delta: {:+} | Speedup delta: {:+.1}",
        delta.max_block_size_delta, delta.speedup_delta
    );

    // Phase 4: Fully connected — complete collapse
    println!("\nPhase 4: Fully connected (complete collapse)");
    let n = 8usize;
    let g3 = DependencyGraph::from_adjacency(vec![1u8; n * n], n, n);
    let (res, delta) = adaptive.update(&g3);
    println!(
        "  Blocks: {} | Changed: {} | Speedup: {:.1}×",
        res.n_blocks, delta.structure_changed, res.speedup_potential
    );

    // Summary
    println!("\n--- Summary ---");
    println!("Total re-fractures: {}", adaptive.refracture_count);
    println!("(Only {} out of 4 updates triggered actual re-computation)", adaptive.refracture_count);
}
