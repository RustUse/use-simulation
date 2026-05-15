# use-simulation

Composable facade crate for RustUse simulation primitives.

## Install

```toml
[dependencies]
use-simulation = "0.0.1"
```

## What it reexports

`use-simulation` reexports `use-state-step`, `use-simulation-clock`,
`use-seed`, `use-monte-carlo`, `use-random-walk`, `use-event-simulation`, and
`use-simulation-result`.

## When to use it

Use this crate when you want one dependency for the full simulation workspace.
Depend on the focused crates directly when you only need a smaller slice.

## Status

This crate is experimental and may evolve while the workspace remains below
`0.3.0`.
