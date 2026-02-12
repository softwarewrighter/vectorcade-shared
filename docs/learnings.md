# Learnings

This document captures issues encountered and their solutions during development.

## 2026-02-12: Initial Implementation

### Issue: Clippy manual_range_contains

**What went wrong:** Used manual range checks in tests like `v >= -1.0 && v < 1.0` instead of `(-1.0..1.0).contains(&v)`.

**Why wasn't it caught sooner:** Tests pass regardless of style; clippy only caught it during strict `-D warnings` check.

**Prevention:** Always run `cargo clippy --all-targets --all-features -- -D warnings` before committing.

### Issue: Dyn-incompatible trait

**What went wrong:** Added a generic method `pick<T>(&mut self, slice: &[T])` to `GameRng` trait, making it incompatible with `dyn GameRng`.

**Root cause:** Generic methods prevent trait object creation because the compiler can't know the size at runtime.

**Solution:** Moved generic `pick` method to a separate extension trait `GameRngExt` and added `pick_index` (non-generic) to the main trait.

**Prevention:** When designing traits intended for dynamic dispatch, avoid generic methods in the trait itself.

### Issue: Markdown ASCII compliance

**What went wrong:** Used Unicode box-drawing characters in architecture diagrams and Unicode arrows.

**Root cause:** Copy-pasting from editor that auto-formats, or using fancy characters for visual appeal.

**Solution:** Replace Unicode with ASCII equivalents:
- Use pipe `|` instead of box-drawing vertical lines
- Use `+` instead of box-drawing corners and junctions
- Use `-` instead of box-drawing horizontal lines
- Use `->` instead of fancy arrows
- Use `v` instead of down-arrow symbols

**Prevention:** Run `markdown-checker -f "**/*.md"` before every commit.

### Issue: sw-checklist module/function limits

**What went wrong:** Initial single-crate design exceeded sw-checklist limits:
- 9 modules per crate (max 7)
- Several modules had >7 functions

**Solution:** Refactored into a 3-crate workspace:
- `vectorcade-core`: Basic types (Rgba, RNG) - 5 modules
- `vectorcade-math`: Math utilities and collision - 7 modules
- `vectorcade-shared`: API contracts (Game, Input, Draw, Font) - 7 modules

Additionally:
- Split `game.rs` into `game/mod.rs`, `game/ctx.rs`, `game/coords.rs`
- Split `collision.rs` into `collision/mod.rs`, `collision/aabb.rs`, `collision/circle.rs`
- Moved `GameRngExt` to separate module to keep `GameRng` trait at 7 methods

**Prevention:** Check sw-checklist early and design with module limits in mind.
