# use-simulation

Composable primitive simulation utilities for Rust.

`use-simulation` is part of RustUse, alongside sibling repositories such as
`use-math`, `use-stats`, `use-optimization`, `use-color`, `use-text`,
`use-time`, and `use-units`. It groups small, focused crates for state
stepping, deterministic simulation clocks, repeatable seeds, Monte Carlo
helpers, random walks, event-driven simulation, and reusable simulation
results.

The RustUse approach in this workspace stays intentionally narrow:

- crates stay small and independently useful
- APIs stay explicit, documented, tested, and composable
- implementations favor practical `f64` and `usize` helpers over framework-style abstractions
- dependencies stay minimal so each crate is easy to audit and adopt

## Workspace crates

- `use-simulation`: thin facade crate that reexports the full simulation workspace
- `use-state-step`: explicit state transition helpers over `f64`
- `use-simulation-clock`: deterministic tick and elapsed-time helpers
- `use-seed`: repeatable seed utilities and simple deterministic sampling
- `use-monte-carlo`: small Monte Carlo mean and variance helpers
- `use-random-walk`: repeatable one-dimensional random walk helpers
- `use-event-simulation`: simple event scheduling and event-state application
- `use-simulation-result`: reusable summaries for finite simulation series

## Facade crate

If you want one dependency for the whole workspace, use `use-simulation`. It
reexports each focused crate and exposes the focused APIs directly so this
works:

```rust
use use_simulation::*;

let seed = SimulationSeed::new(7);
let path = random_walk(RandomWalkConfig {
    start: 0.0,
    step_size: 1.0,
    steps: 4,
    seed,
})
.unwrap();
let summary = summarize(&path).unwrap();

assert_eq!(summary.steps, 4);
assert_eq!(summary.final_value, *path.last().unwrap());
```

## Status

This workspace is experimental while it remains below `0.3.0`. Expect the
public API to stay small and practical, but still evolve as the RustUse
simulation surface becomes clearer.

## Development

Run the standard workspace checks from the repository root:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo doc --workspace --no-deps
```
