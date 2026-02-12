# Architecture: vectorcade-shared

## Overview

`vectorcade-shared` is the root crate in a multi-repo DAG for VectorCade, a Rust-based vector graphics arcade game platform. This crate defines the stable API contracts that all other repos depend on.

## Design Principles

1. **Pure Rust** - No WASM bindings, no wgpu, no web-sys dependencies
2. **Platform Agnostic** - Must compile and run anywhere Rust runs
3. **Minimal Dependencies** - Only `glam` for math (with `libm` for no-std compatibility)
4. **Trait-Based Abstraction** - Define interfaces, not implementations

## Module Structure

```
vectorcade-shared/
+-- src/
|   +-- lib.rs      # Crate root, re-exports all modules
|   +-- collision.rs # AABB, Circle, line intersection
|   +-- color.rs    # Rgba color type with common constants
|   +-- draw.rs     # DrawCmd display-list primitives
|   +-- font.rs     # VectorFont trait and GlyphPath types
|   +-- game.rs     # Game trait, GameCtx, ScreenInfo, AudioOut
|   +-- input.rs    # InputState trait, Key/Axis enums
|   +-- math.rs     # 2D/3D math helpers
|   +-- rng.rs      # GameRng trait, Xorshift64
+-- tests/
    +-- math_smoke.rs
```

## Dependency Graph (Multi-Repo DAG)

```
                  vectorcade-shared (this repo)
                           |
         +-----------------+----------------+
         |                 |                |
         v                 v                |
  vectorcade-fonts  vectorcade-render-wgpu  |
         |                 |                |
         +--------+--------+                |
                  |                         |
                  v                         |
           vectorcade-games <---------------+
                  |
                  v
           vectorcade-web-yew
```

## Key Types

### Display List Model (`draw.rs`)

Games render via a display list of `DrawCmd` variants:
- `Clear` - Fill screen with color
- `Line` / `Polyline` - Vector strokes with configurable width and glow
- `Text` - Font-rendered text with style selection
- `PushTransform` / `PopTransform` - Affine transformation stack
- `BeginLayer` / `EndLayer` - Optional render grouping

### Game Lifecycle (`game.rs`)

The `Game` trait defines:
- `metadata()` - Name, preferred aspect ratio
- `reset()` - Initialize/restart game state
- `update(ctx, dt)` - Fixed-timestep logic
- `render(ctx, out)` - Append draw commands

### Input Abstraction (`input.rs`)

`InputState` trait abstracts:
- Keyboard via `Key` enum and `Button` state
- Virtual axes (for gamepad/touch)
- Pointer (mouse/touch) position and state

### Vector Fonts (`font.rs`)

`VectorFont` trait allows multiple font styles (Atari, Midway, etc.) with stroke-based glyph paths.

## Thread Safety

All types are designed to be `Send + Sync` where applicable. No interior mutability or global state.

## Versioning Strategy

This crate follows semver. Breaking changes to public traits require a major version bump. Downstream repos pin to compatible versions.
