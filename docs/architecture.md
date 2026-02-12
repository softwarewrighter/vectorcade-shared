# Architecture: vectorcade-shared

## Overview

`vectorcade-shared` is a Cargo workspace containing the root crates for VectorCade, a Rust-based vector graphics arcade game platform. The workspace defines the stable API contracts that all other repos depend on.

## Design Principles

1. **Pure Rust** - No WASM bindings, no wgpu, no web-sys dependencies
2. **Platform Agnostic** - Must compile and run anywhere Rust runs
3. **Minimal Dependencies** - Only `glam` for math (with `libm` for no-std compatibility)
4. **Trait-Based Abstraction** - Define interfaces, not implementations
5. **sw-checklist Compliance** - Max 7 modules per crate, max 7 functions per module

## Workspace Structure

```
vectorcade-shared/           # Workspace root
+-- Cargo.toml               # Workspace manifest
+-- vectorcade-core/         # Basic types (Rgba, RNG)
|   +-- src/
|       +-- lib.rs
|       +-- color.rs         # Rgba type with color constants
|       +-- rng/
|           +-- mod.rs       # GameRng trait, Xorshift64
|           +-- ext.rs       # GameRngExt extension trait
|           +-- xorshift.rs  # Xorshift64 implementation
+-- vectorcade-math/         # Math utilities and collision
|   +-- src/
|       +-- lib.rs
|       +-- helpers.rs       # lerp, clamp, remap, wrap_position
|       +-- projection.rs    # 3D projection, angle utilities
|       +-- transform.rs     # 2D transformation matrices
|       +-- collision/
|           +-- mod.rs       # Line intersection functions
|           +-- aabb.rs      # Axis-aligned bounding box
|           +-- circle.rs    # Circle collision primitive
+-- vectorcade-shared/       # API contracts
    +-- src/
    |   +-- lib.rs           # Re-exports core and math crates
    |   +-- draw.rs          # DrawCmd display-list primitives
    |   +-- font.rs          # VectorFont trait, GlyphPath types
    |   +-- input.rs         # InputState trait, Key/Axis enums
    |   +-- game/
    |       +-- mod.rs       # Game trait
    |       +-- ctx.rs       # GameCtx, ScreenInfo, AudioOut
    |       +-- coords.rs    # NDC/pixel coordinate conversion
    +-- tests/
        +-- math_smoke.rs
```

## Crate Dependencies

```
vectorcade-core (basic types)
       |
       v
vectorcade-math (depends on core for potential future use)
       |
       v
vectorcade-shared (re-exports both, adds Game/Draw/Input/Font)
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
