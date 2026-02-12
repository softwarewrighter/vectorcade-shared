# Design: vectorcade-shared

## Design Decisions

### 1. Display List Architecture

**Decision:** Games emit a `Vec<DrawCmd>` each frame rather than calling a render API directly.

**Rationale:**
- Decouples game logic from rendering implementation
- Enables multiple backends (wgpu, canvas2d, native) without game changes
- Allows render-side batching and optimization
- Simplifies testing (assert on draw commands, not pixels)

**Trade-offs:**
- Per-frame allocation of command vectors (mitigated by reusing/clearing)
- Indirection adds minor complexity

### 2. Coordinate System

**Decision:** World space uses normalized device coordinates (-1 to 1) with Y-up.

**Rationale:**
- Matches OpenGL/WebGPU NDC conventions
- Games don't need to know screen resolution
- Renderer handles mapping to actual pixels

**Trade-offs:**
- Games must use `px_to_ndc()` helper for pixel-based positioning
- Aspect ratio handling delegated to renderer

### 3. Transform Stack

**Decision:** `PushTransform(Mat3)` / `PopTransform` commands use `glam::Mat3` for 2D affine transforms.

**Rationale:**
- `glam` is a well-maintained, minimal math library with WASM support
- 3x3 matrices handle rotation, scaling, translation, and shear
- Stack model familiar from OpenGL/canvas2D

### 4. Font Style IDs

**Decision:** Fonts identified by `FontStyleId(u32)` rather than string names.

**Rationale:**
- Faster comparison and hashing
- No string allocation in render paths
- Predefined constants for common styles (ATARI, MIDWAY, etc.)

**Trade-offs:**
- Less self-documenting than strings
- Requires coordination between this crate and `vectorcade-fonts`

### 5. Input Abstraction

**Decision:** Dual model with discrete keys (`Key` enum) and continuous axes (`Axis` enum).

**Rationale:**
- Keys map naturally to keyboard
- Axes map to gamepad sticks and touch virtual joysticks
- Same game code works across input methods

**Trade-offs:**
- Games must handle both models or choose one
- `Axis` values are normalized (-1 to 1), losing raw precision

### 6. 3D Projection Helper

**Decision:** Include a simple perspective projection function for 2.5D games (Battlezone, Tempest).

**Rationale:**
- These games don't need a full 3D engine
- Project 3D line segments to 2D, then render as normal polylines
- Keeps complexity in games that need it, not in the shared API

**Trade-offs:**
- Single projection model may not fit all 3D-ish games
- Depth-based effects (fog, brightness) not included here

## API Boundaries

### What Goes Here

- Type definitions (structs, enums)
- Trait definitions (Game, InputState, VectorFont, AudioOut, Assets)
- Pure functions (math helpers, coordinate conversion)
- Constants (colors, font style IDs)

### What Doesn't Go Here

- Rendering implementation → `vectorcade-render-wgpu`
- Font glyph data → `vectorcade-fonts`
- Game implementations → `vectorcade-games`
- Platform integration → `vectorcade-web-yew`

## Error Handling

This crate generally uses `Option` for fallible operations (e.g., `project_persp` returns `None` for points behind camera). No custom error types are defined since most operations are infallible.

## Testing Strategy

- Unit tests for all math functions with edge cases
- Property tests for coordinate transformations (round-trip, associativity)
- Compile tests ensuring no platform-specific code sneaks in
