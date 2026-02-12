# Plan: vectorcade-shared

## Current Phase: Initial API Stabilization

The core API is implemented. Focus now shifts to documentation, testing, and preparing for downstream integration.

## Completed Work

- [x] Module structure established
- [x] `DrawCmd` enum with all primitive variants
- [x] `Stroke` type with color, width, glow
- [x] `Game` trait with lifecycle methods
- [x] `GameCtx` with input, audio, screen info
- [x] `InputState` trait and supporting types
- [x] `VectorFont` trait and `GlyphPath` types
- [x] Math helpers (clamp, wrap, rot2, translate2)
- [x] 3D→2D perspective projection
- [x] `Rgba` color type with constants
- [x] Basic math smoke tests

## In Progress

- [ ] Documentation coverage for all public items
- [ ] Additional unit tests for edge cases

## Upcoming Work

### Phase 1: API Polish
- Add rustdoc comments to all public types and functions
- Ensure `#[must_use]` on appropriate functions
- Add `Default` implementations where sensible
- Consider `#[non_exhaustive]` for enums that may grow

### Phase 2: Test Coverage
- Property-based tests for math functions
- Round-trip tests for coordinate conversions
- Compile-time checks for Send+Sync

### Phase 3: Downstream Integration
- Coordinate with `vectorcade-fonts` on FontStyleId constants
- Validate API against Pong implementation in `vectorcade-games`
- Ensure `vectorcade-render-wgpu` can consume all DrawCmd variants

### Phase 4: Iteration
- Gather feedback from downstream repos
- Add any missing types or traits discovered during integration
- Consider adding replay/determinism helpers

## Milestones

| Milestone | Description | Target |
|-----------|-------------|--------|
| M1 | API v0.1.0 published, all downstream repos compile | Week 1 |
| M2 | Full rustdoc coverage | Week 2 |
| M3 | Test coverage > 80% | Week 3 |
| M4 | Pong playable end-to-end | Week 4 |

## Risks

| Risk | Mitigation |
|------|------------|
| Breaking API changes discovered late | Early integration testing with downstream repos |
| Math precision issues on WASM | Use `libm` feature of glam, test in browser |
| Font trait too rigid | Design for extension (non_exhaustive, additional methods) |

## Dependencies

- `glam 0.27` (minimal math library, no-std compatible)
- `approx 0.5` (dev only, for float comparisons in tests)

## Notes for Agents

- Keep this crate **pure** - no wasm-bindgen, no web-sys, no wgpu
- Prefer `f32` over `f64` for all game-related math
- All public items need rustdoc comments before v1.0
