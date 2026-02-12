# PRD: vectorcade-shared

## Product Overview

`vectorcade-shared` provides the foundational API contracts for VectorCade, a retro vector-arcade game platform targeting browser (WASM) and potentially native/mobile deployments.

## Goals

1. **Stable API Surface** - Enable parallel development across 4+ dependent repos
2. **Pure Portability** - No platform-specific code; compile anywhere Rust compiles
3. **Minimal Footprint** - Keep dependencies minimal for fast compilation and small WASM bundles
4. **Classic Arcade Fidelity** - Support the visual characteristics of vector displays (phosphor glow, stroke width, coordinate systems)

## Non-Goals

- Implementing rendering (that's `vectorcade-render-wgpu`)
- Implementing font glyphs (that's `vectorcade-fonts`)
- Game logic (that's `vectorcade-games`)
- Web integration (that's `vectorcade-web-yew`)

## Users

### Primary: Downstream Crates

- `vectorcade-fonts` - Needs font traits and glyph path types
- `vectorcade-render-wgpu` - Needs DrawCmd, Stroke, color types
- `vectorcade-games` - Needs Game trait, GameCtx, input abstractions
- `vectorcade-web-yew` - Needs all public types for integration

### Secondary: Game Developers

Developers implementing new games for the platform will use these traits and types directly.

## Requirements

### Functional Requirements

| ID | Requirement | Status |
|----|-------------|--------|
| F1 | Define `DrawCmd` enum covering: clear, line, polyline, text, transforms | Done |
| F2 | Define `Game` trait with update/render lifecycle | Done |
| F3 | Define `InputState` trait abstracting keyboard, axes, pointer | Done |
| F4 | Define `VectorFont` trait for stroke-based font rendering | Done |
| F5 | Provide 2D math helpers (rotation, translation matrices) | Done |
| F6 | Provide 3D->2D perspective projection for 2.5D games | Done |
| F7 | Define `Rgba` color type with common constants | Done |
| F8 | Define `Stroke` type with width and glow parameters | Done |

### Non-Functional Requirements

| ID | Requirement | Status |
|----|-------------|--------|
| N1 | Zero platform-specific dependencies | Done |
| N2 | Compile with stable Rust | Done |
| N3 | All public types documented | Partial |
| N4 | Unit tests for math functions | Done |
| N5 | No unsafe code | Done |

## Success Metrics

- All 4 downstream repos compile against this crate
- No breaking changes without semver major bump
- Test coverage > 80% for math module
- Documentation coverage 100% for public API

## Future Considerations

- Audio event types (currently just `AudioOut::beep` stub)
- Replay/determinism helpers (seeded RNG integration)
- Asset loading trait for fonts/sounds
