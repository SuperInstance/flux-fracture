# flux-fracture

**Disjoint Linear Algebra for Constraint Systems** — fracture constraint systems into independent blocks, coalesce results provably correct.

## What It Does

When constraints are independent, the partition function factorizes: `Z = Π(Zᵢ)`. This crate makes that **operational**:

1. **Fracture** — Split a constraint system into independent blocks via connected-component detection on the bipartite constraint×dimension dependency graph.
2. **Coalesce** — Merge block-level error masks via bitwise OR with **zero false negatives guaranteed**.
3. **Adapt** — Re-fracture dynamically when the dependency structure changes.

### Theorem

> If fracture correctly identifies connected components of the constraint–dimension dependency graph, coalescence via bitwise OR preserves zero false negatives.

**Proof:** Each constraint violation is a Boolean event. For independent blocks, the event spaces are disjoint (no shared dimensions). The union of all violations = OR of block error masks. QED.

## Installation

```toml
[dependencies]
flux-fracture = "0.1.0"
```

Or from source:

```bash
git clone https://github.com/SuperInstance/flux-fracture
cd flux-fracture
cargo build --release
```

## Quick Start

```rust
use flux_fracture::{DependencyGraph, Fracturer, Coalescer};

// Build an identity graph: 8 constraints, each on its own dimension
let graph = DependencyGraph::identity(8);

// Fracture into independent blocks
let fracturer = Fracturer::new();
let result = fracturer.fracture(&graph);

println!("Blocks: {}", result.n_blocks);           // 8
println!("Speedup: {}x", result.speedup_potential); // 8x

// Coalesce block masks
let coalescer = Coalescer::new();
let total = coalescer.coalesce_masks(&[0b0001, 0b0010, 0b0100]);
assert_eq!(total, 0b0111);
```

## Dependency Structures

| Structure | Constraints | Blocks | Speedup |
|-----------|:-----------:|:------:|:-------:|
| Fully independent | 8 | 8 | 8× |
| Block diagonal | 8 (2×4) | 2 | 2× |
| Chain (cyclic pairs) | 8 | 1 | 1× |
| Fully connected | 8 | 1 | 1× |

## API Reference

### `DependencyGraph`

Bipartite constraint×dimension adjacency graph.

| Method | Description |
|--------|-------------|
| `from_adjacency(adj, n_c, n_d)` | Build from flat row-major adjacency |
| `from_masks(masks)` | Build from per-constraint dimension index lists |
| `identity(n)` | Identity: constraint `i` touches only dimension `i` |
| `involves(c, d)` | Check if constraint `c` involves dimension `d` |
| `constraint_dims(c)` | Dimensions involved in constraint `c` |
| `dim_constraints(d)` | Constraints involving dimension `d` |
| `set_edge(c, d, val)` | Set an adjacency entry |
| `fill_block(c0, c1, d0, d1)` | Fill a rectangular block with 1s |
| `stats()` | Connected-component statistics |

### `Fracturer`

Splits a dependency graph into independent blocks via BFS.

| Method | Description |
|--------|-------------|
| `fracture(&graph)` | Find connected components → `FractureResult` |

### `FractureResult`

| Field | Type | Description |
|-------|------|-------------|
| `blocks` | `Vec<Block>` | Independent blocks |
| `n_blocks` | `usize` | Number of blocks |
| `largest_block_size` | `usize` | Constraints in the biggest block |
| `speedup_potential` | `f64` | `n_constraints / largest_block_size` |

### `Coalescer`

Merges block results with zero false negatives.

| Method | Description |
|--------|-------------|
| `coalesce_masks(&[u64])` | Bitwise OR of error masks |
| `coalesce_arrays(&[Vec<u8>])` | Elementwise OR of violation arrays |
| `verify_coalescence(masks, indices, mono)` | Verify coalesced == monolithic |

### `AdaptiveFracturer`

Re-fractures when the dependency structure changes.

| Method | Description |
|--------|-------------|
| `update(&graph)` | Re-fracture, returns `(FractureResult, FractureDelta)` |
| `current()` | Get last fracture result |
| `refracture_count` | How many times structure actually changed |

## Performance

Pure Rust, zero dependencies. BFS connected-component detection on flat adjacency arrays.

Expected performance vs Python (NumPy-based):

| System | Rust | Python | Speedup |
|--------|------|--------|:-------:|
| 8 constraints | ~100 ns | ~10 µs | 100× |
| 64 constraints | ~500 ns | ~50 µs | 100× |
| 256 constraints | ~5 µs | ~500 µs | 100× |

Run benchmarks:

```bash
cargo bench
```

## Testing

```bash
cargo test
```

All four dependency structures (independent, block diagonal, chain, fully connected) are verified with zero false negatives across coalescence.

## License

MIT — Part of the [SuperInstance](https://github.com/SuperInstance) constraint-theory ecosystem.

## Related Crates

| Crate | Purpose |
|-------|--------|
| [plato-types](https://github.com/SuperInstance/plato-types) | Tile lifecycle, Lamport clocks |
| [tensor-spline](https://github.com/SuperInstance/tensor-spline) | SplineLinear compression, Eisenstein lattice weights |
| [plato-data](https://github.com/SuperInstance/plato-data) | CSV/JSONL/PLATO data loading |
| [plato-training](https://github.com/SuperInstance/plato-training) | Micro model training and fleet deployment |
