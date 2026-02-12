# vectorcade-shared

Root crate for the VectorCade multi-repo DAG.

This crate is **pure Rust**: no WASM bindings, no wgpu, no web-sys.
It defines the stable API contract that other repos depend on:

- Display-list drawing: `DrawCmd`
- Input abstraction traits
- Game loop traits
- Vector-font traits and glyph path types (interfaces only)
- Small math helpers (2D + optional 3D projection helpers)

## Build

```bash
cargo test
```
