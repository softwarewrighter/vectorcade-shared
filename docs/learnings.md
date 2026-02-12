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
- `|` instead of `│`
- `+` instead of `┌ ┐ └ ┘ ├ ┤ ┬ ┴ ┼`
- `-` instead of `─`
- `->` instead of `->`
- `v` instead of `▼`

**Prevention:** Run `markdown-checker -f "**/*.md"` before every commit.

## Open Issues (sw-checklist)

The following sw-checklist requirements need architectural changes:

1. **Module function count exceeded** - Several modules have >7 functions:
   - collision.rs: 16 functions
   - math.rs: 15 functions
   - rng.rs: 16 functions
   - game.rs: 9 functions

2. **Crate module count exceeded** - 9 modules (max 7)

**Options to address:**
- Combine related modules (e.g., color + draw)
- Split large modules into sub-modules
- Move helper functions into impl blocks
- Reduce scope of shared library

These require architectural decisions and will be addressed in a future iteration.
