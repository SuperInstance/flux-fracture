//! Fracture-coalesce test suite — mirrors the Python experiment framework.

use flux_fracture::*;

fn make_identity_graph(n: usize) -> DependencyGraph {
    DependencyGraph::identity(n)
}

fn check_constraint(value: f64, lo: f64, hi: f64) -> bool {
    value.is_nan() || value < lo || value > hi
}

// ---------------------------------------------------------------------------
// Structure A: Fully independent → 8 blocks
// ---------------------------------------------------------------------------

#[test]
fn test_fully_independent_8_blocks() {
    let graph = make_identity_graph(8);
    let fracturer = Fracturer::new();
    let result = fracturer.fracture(&graph);

    assert_eq!(result.n_blocks, 8, "identity graph must have 8 blocks");
    assert_eq!(result.largest_block_size, 1, "each block has 1 constraint");
    assert!((result.speedup_potential - 8.0).abs() < 1e-9, "speedup = 8x");
}

// ---------------------------------------------------------------------------
// Structure B: Block diagonal → 2 blocks
// ---------------------------------------------------------------------------

fn make_block_diagonal_graph() -> DependencyGraph {
    let n = 8usize;
    let mut graph = DependencyGraph::identity(n);
    graph.fill_block(0, 4, 0, 4);
    graph.fill_block(4, 8, 4, 8);
    graph
}

#[test]
fn test_block_diagonal_2_blocks() {
    let graph = make_block_diagonal_graph();
    let fracturer = Fracturer::new();
    let result = fracturer.fracture(&graph);

    assert_eq!(result.n_blocks, 2, "block diagonal must have 2 blocks");
    assert_eq!(result.largest_block_size, 4, "each block has 4 constraints");
    assert!((result.speedup_potential - 2.0).abs() < 1e-9, "speedup = 2x");
}

// ---------------------------------------------------------------------------
// Structure C: Chain (overlapping pairs) → 1 block
// ---------------------------------------------------------------------------

fn make_chain_graph() -> DependencyGraph {
    let n = 8usize;
    let mut graph = DependencyGraph::identity(n);
    for i in 0..n {
        graph.set_edge(i, (i + 1) % n, 1);
    }
    graph
}

#[test]
fn test_chain_1_block() {
    let graph = make_chain_graph();
    let fracturer = Fracturer::new();
    let result = fracturer.fracture(&graph);

    assert_eq!(result.n_blocks, 1, "cyclic chain is fully connected → 1 block");
    assert_eq!(result.largest_block_size, 8);
}

// ---------------------------------------------------------------------------
// Structure D: Fully connected → 1 block
// ---------------------------------------------------------------------------

fn make_fully_connected_graph() -> DependencyGraph {
    let n = 8usize;
    let adj = vec![1u8; n * n];
    DependencyGraph::from_adjacency(adj, n, n)
}

#[test]
fn test_fully_connected_1_block() {
    let graph = make_fully_connected_graph();
    let fracturer = Fracturer::new();
    let result = fracturer.fracture(&graph);

    assert_eq!(result.n_blocks, 1, "fully connected → 1 block");
    assert_eq!(result.largest_block_size, 8);
    assert!((result.speedup_potential - 1.0).abs() < 1e-9, "no speedup");
}

// ---------------------------------------------------------------------------
// Coalescence correctness across all structures
// ---------------------------------------------------------------------------

const BOUNDS: [(f64, f64); 8] = [
    (0.0, 8000.0),    // d0
    (0.0, 300.0),     // d1
    (-40.0, 150.0),   // d2
    (0.0, 100.0),     // d3
    (0.0, 200.0),     // d4
    (-720.0, 720.0),  // d5
    (9.0, 16.0),      // d6
    (0.0, 100.0),     // d7
];

fn verify_structure(graph: &DependencyGraph, frac: &FractureResult, test_values: &[[f64; 8]]) -> usize {
    let coalescer = Coalescer::new();
    let mut mismatches = 0usize;

    for row in test_values {
        // Monolithic check
        let mut mono_mask: u64 = 0;
        for i in 0..graph.n_constraints {
            let dims = graph.constraint_dims(i);
            for &d in &dims {
                let (lo, hi) = BOUNDS[d];
                if check_constraint(row[d], lo, hi) {
                    mono_mask |= 1u64 << i;
                    break;
                }
            }
        }

        // Block check
        let mut block_masks: Vec<u64> = Vec::new();
        let mut block_indices: Vec<Vec<usize>> = Vec::new();
        for block in &frac.blocks {
            let mut block_mask: u64 = 0;
            for (bit, &c_idx) in block.constraint_indices.iter().enumerate() {
                let dims = graph.constraint_dims(c_idx);
                for &d in &dims {
                    let (lo, hi) = BOUNDS[d];
                    if check_constraint(row[d], lo, hi) {
                        block_mask |= 1u64 << bit;
                        break;
                    }
                }
            }
            block_masks.push(block_mask);
            block_indices.push(block.constraint_indices.clone());
        }

        let (correct, _) = coalescer.verify_coalescence(&block_masks, &block_indices, mono_mask);
        if !correct { mismatches += 1; }
    }

    mismatches
}

fn make_test_values() -> Vec<[f64; 8]> {
    vec![
        [4000.0, 150.0, 25.0, 50.0, 100.0, 0.0, 12.0, 50.0],       // all in bounds
        [9000.0, 400.0, -50.0, 200.0, 300.0, 800.0, 5.0, 150.0],    // all out
        [4000.0, 400.0, 25.0, 50.0, 100.0, 0.0, 12.0, 50.0],       // d1 out
        [4000.0, 150.0, -50.0, 50.0, 100.0, 0.0, 12.0, 50.0],      // d2 out
        [4000.0, 150.0, 25.0, 200.0, 100.0, 0.0, 12.0, 50.0],      // d3 out
        [4000.0, 150.0, 25.0, 50.0, 300.0, 0.0, 12.0, 50.0],       // d4 out
        [4000.0, 150.0, 25.0, 50.0, 100.0, 800.0, 12.0, 50.0],     // d5 out
        [4000.0, 150.0, 25.0, 50.0, 100.0, 0.0, 5.0, 50.0],        // d6 out
        [4000.0, 150.0, 25.0, 50.0, 100.0, 0.0, 12.0, 150.0],      // d7 out
        [0.0, 0.0, -40.0, 0.0, 0.0, -720.0, 9.0, 0.0],             // exact lo bounds
        [8000.0, 300.0, 150.0, 100.0, 200.0, 720.0, 16.0, 100.0],  // exact hi bounds
        [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],                  // zeros
        [4000.0, 150.0, 55.0, 50.0, 100.0, 360.0, 12.5, 50.0],     // all valid
    ]
}

#[test]
fn test_coalescence_independent() {
    let graph = make_identity_graph(8);
    let fracturer = Fracturer::new();
    let frac = fracturer.fracture(&graph);
    let mm = verify_structure(&graph, &frac, &make_test_values());
    assert_eq!(mm, 0, "independent: zero mismatches");
}

#[test]
fn test_coalescence_block_diagonal() {
    let graph = make_block_diagonal_graph();
    let fracturer = Fracturer::new();
    let frac = fracturer.fracture(&graph);
    let mm = verify_structure(&graph, &frac, &make_test_values());
    assert_eq!(mm, 0, "block diagonal: zero mismatches");
}

#[test]
fn test_coalescence_chain() {
    let graph = make_chain_graph();
    let fracturer = Fracturer::new();
    let frac = fracturer.fracture(&graph);
    let mm = verify_structure(&graph, &frac, &make_test_values());
    assert_eq!(mm, 0, "chain: zero mismatches");
}

#[test]
fn test_coalescence_fully_connected() {
    let graph = make_fully_connected_graph();
    let fracturer = Fracturer::new();
    let frac = fracturer.fracture(&graph);
    let mm = verify_structure(&graph, &frac, &make_test_values());
    assert_eq!(mm, 0, "fully connected: zero mismatches");
}

// ---------------------------------------------------------------------------
// Coalescer unit tests
// ---------------------------------------------------------------------------

#[test]
fn test_coalesce_empty_masks() {
    let coalescer = Coalescer::new();
    assert_eq!(coalescer.coalesce_masks(&[]), 0);
}

#[test]
fn test_coalesce_simple_or() {
    let coalescer = Coalescer::new();
    assert_eq!(coalescer.coalesce_masks(&[0b0001, 0b0010, 0b0100]), 0b0111);
}

#[test]
fn test_coalesce_overlap() {
    let coalescer = Coalescer::new();
    assert_eq!(coalescer.coalesce_masks(&[0b0011, 0b0110]), 0b0111);
}

#[test]
fn test_verify_perfect_match() {
    let coalescer = Coalescer::new();
    let masks = [0b01, 0b01];
    let indices = [vec![0], vec![1]];
    let (ok, _) = coalescer.verify_coalescence(&masks, &indices, 0b11);
    assert!(ok);
}

#[test]
fn test_coalesce_arrays() {
    let coalescer = Coalescer::new();
    let a = vec![1, 0, 1];
    let b = vec![0, 1, 1];
    let result = coalescer.coalesce_arrays(&[a, b]);
    assert_eq!(result, vec![1, 1, 1]);
}

// ---------------------------------------------------------------------------
// AdaptiveFracturer
// ---------------------------------------------------------------------------

#[test]
fn test_adaptive_refracture() {
    let mut adaptive = AdaptiveFracturer::new();
    let g1 = make_identity_graph(8);
    let (res1, delta1) = adaptive.update(&g1);
    assert!(delta1.structure_changed);
    assert_eq!(res1.n_blocks, 8);

    let (_res2, delta2) = adaptive.update(&g1);
    assert!(!delta2.structure_changed);
    assert_eq!(adaptive.refracture_count, 1);

    let g2 = make_block_diagonal_graph();
    let (res3, delta3) = adaptive.update(&g2);
    assert!(delta3.structure_changed);
    assert_eq!(res3.n_blocks, 2);
    assert_eq!(adaptive.refracture_count, 2);
}

// ---------------------------------------------------------------------------
// Graph from_masks
// ---------------------------------------------------------------------------

#[test]
fn test_graph_from_masks() {
    let masks: Vec<&[usize]> = vec![&[0, 2], &[1], &[0, 1, 3]];
    let graph = DependencyGraph::from_masks(&masks);
    assert_eq!(graph.n_constraints, 3);
    assert_eq!(graph.n_dimensions, 4);
    assert!(graph.involves(0, 0));
    assert!(graph.involves(0, 2));
    assert!(!graph.involves(0, 1));
    assert!(graph.involves(2, 3));
}

#[test]
fn test_graph_stats() {
    let graph = make_identity_graph(8);
    let stats = graph.stats();
    assert_eq!(stats.n_blocks, 8);
    assert_eq!(stats.largest_block, 1);
}
