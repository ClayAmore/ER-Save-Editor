Rust Skills Loaded
        _~^~^~_
    \) /  o o  \ (/
      '_   -   _'
      / '-----' \

Formatted per the skill-creator rules: YAML frontmatter with name + a pushy description (the trigger mechanism), imperative body under 500 lines, "why" explanations instead of bare MUSTs, and an exact report template. Save it as .claude/skills/rust-code-review/SKILL.md (folder name must match name).

---
name: rust-code-review
description: Reviews Rust code for correctness, idiomatic style, and project-convention violations before a commit or on a pull request. Use this skill whenever the user asks to review code, check a diff, look over changes, sanity-check a branch, or asks "is this ready to commit/merge" — even without the word "review". Also use it proactively right after finishing a multi-file Rust implementation, before handing the changes back to the user.
---

# Rust Code Review

Review changed Rust code in one of two modes — **pre-commit** (uncommitted working-tree
changes) or **PR** (a branch against its base). The job is to surface what actu
nics, wrong data, broken invariants, and real idiom violations — each with a
`file:line` reference and a concrete failure scenario. Findings without a failure
scenario are opinions; either ground them or drop them.

Report findings; do not rewrite the code unless asked.

## Step 1 — Load project context before reading the diff

Read `CLAUDE.md` (fall back to `README.md` if absent) first. A reviewer that flags
deliberate, documented decisions wastes the author's time and teaches them to ignore
findings, so project conventions override generic Rust style throughout this skill.
Note especially any documented invariants — they are exactly where regressions hide,
and Step 5 checks new code against them explicitly.

## Step 2 — Determine the scope

| Mode | When | Commands |
|------|------|----------|
| Pre-commit | uncommitted changes | `git status`, then `git diff HEAD` (staged
| PR / branch | reviewing a branch | `git merge-base HEAD master` (or `main`), then `git diff <base>...HEAD` and `git log --oneline <base>..HEAD` |

The commit list is the change's *intent*; read it first so findings are judged
what the author was trying to do, not what you assume. Then read the diff — and for each
hunk, read the full surrounding function in the file. A hunk without its function
routinely hides the bug: a changed branch of a `match`, a caller whose assumpti
shifted, an invariant only visible ten lines up.

## Step 3 — Run verification before reading anything

Mechanical findings cost the tools nothing; let them take the first pass so pro
findings can focus on what clippy cannot see.

```
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test
cargo fmt --check
```

Report the outcome of each in the final report. Two rules:

- If clippy already caught it, do not repeat it as a prose finding — one mentio
- A green test run can be vacuous. Suites whose tests self-skip when fixtures are
  missing (they print `skipping: ...` and pass) prove nothing on a machine without the
  fixtures — say so explicitly next to the test line in the report rather than
  the behaviour is verified.

## Step 4 — Checklist

Ordered by how much the findings matter. Not every section applies to every diff;
skip freely, but never skip correctness.

### Correctness and panics (highest priority)
- `unwrap()`, `expect()`, indexing, slicing, or arithmetic that can panic on
  user-controlled input (file bytes, save fields, parsed ids). Ask: what input reaches
  this line, and what happens on the worst legal one?                                                                                                                                     - Off-by-one, inverted conditions, and swapped arguments of the same type — the
  `write(offset, len)` transposition class.
- `match` arms with an over-broad `_` that silently swallows future variants, or a
  missing arm that only compiles by luck of exhaustiveness.
- Mutation paths that leave state half-updated when an early return or `?` fires
  mid-way (partial writes are worse than failed writes).
- Every `unsafe` block: the invariant it relies on, stated and actually upheld by the
  surrounding code.

### Error handling
- `unwrap()` outside tests and `#[cfg(test)]` — at minimum `expect("why")` so the
  panic says something; prefer `?` when callers can do something about it.
- Swallowed errors: `let _ = expr_that_returns_result`, ignored `send`/`write` results.
- Consistency with the codebase's existing channel (some projects deliberately use
  panics as the parser's error channel — that is a Step 1 decision, not a finding).

### Naming and API conventions                                                                                                                                                            - No `get_` prefix (`name()`, not `get_name()`); conversion names match cost —
  `as_` cheap borrowed, `to_` expensive/copying, `into_` consumes.
- `iter()` / `iter_mut()` / `into_iter()` follow the standard convention.                                                                                                                 - Newtypes where domain semantics matter (`struct ItemId(u32)` beats bare `u32`
  call sites and prevents id-space mix-ups).
                                                                                                                                                                                          ### Types and data modeling
- Fixed-size data in arrays, not `Vec`; `&str` parameters instead of `String`;                                                                                                              `Vec::with_capacity` where the size is known and the loop is hot.
- `bool` parameters that are unreadable at call sites (`update(id, true, false)`)
  want a small enum.                                                                                                                                                                      
### Strings
- `bytes()` instead of `chars()` when the data is ASCII (offset math, byte protocols).                                                                                                    - `format!` / `push_str` instead of `+` concatenation, especially in loops.
- `contains()` on strings inside iteration — O(n·m); usually a set or prefix check.
                                                                                                                                                                                          ### Ownership, borrows, memory
- `.clone()` on large data or hot paths without a stated reason; borrow instead.                                                                                                          - `RefCell::borrow` that can panic at runtime where `try_borrow` is cheap.
- Non-trivial lifetimes named for meaning (`'src`, `'ctx`), not `'a`/`'b` soup.                                                                                                           
### Concurrency and async
- Consistent lock ordering wherever two locks can be held together (deadlock review).                                                                                                     - `AtomicBool`/`AtomicUsize` instead of `Mutex<bool>`/`Mutex<usize>`, with a me
  ordering chosen deliberately rather than defaulting to `SeqCst` blindly.
- A `std::sync` lock (non-`Send` guard) held across an `.await`, or blocking calls
  (file IO, channel `recv`) inside async contexts.
                                                                                                                                                                                          ### Macros
- A declarative macro where a function or generic would do the same job. Macros hide                                                                                                        logic from rustfmt, clippy, and readers; they must earn their place.
                                                                                                                                                                                          ### Deprecated patterns
Only flag when the crate's MSRV permits the replacement.
                                                                                                                                                                                          | Seen | Prefer | Since |
|------|--------|-------|
| `lazy_static!` | `std::sync::OnceLock` | 1.70 |                                                                                                                                         | `once_cell::Lazy` | `std::sync::LazyLock` | 1.80 |
| `try!()` | `?` | 2018 |
| `failure` / `error-chain` | `thiserror` / `anyhow` | — |                                                                                                                                
## Step 5 — Project conventions win
                                                                                                                                                                                          Documented decisions are not findings. The kind of thing that looks non-idiomat
is deliberate when the docs say so: parsers that `assert!` their way through a layout
because panics are the error channel behind a `catch_unwind`; write paths gated by a                                                                                                      `changed` flag so untouched sections keep their original bytes; near-duplicated
`match` arms per platform kept on purpose instead of abstracted. Flag *violations of
the documented invariant*, never the pattern itself. If CLAUDE.md documents an                                                                                                            invariant (byte-identical round trip, id-space separation, param-map freshness)
the diff against those rules line by line — that is where this review earns its keep.

## Step 6 — Report                                                                                                                                                                        
Use this template exactly. Findings are ranked most-severe first within each section;
every finding carries `file:line`, the concrete failure, and a fix direction in one or                                                                                                    two lines — not a rewritten implementation.

```markdown                                                                                                                                                                               # Code Review — <pre-commit | PR>: <one line describing the change>

## Verification                                                                                                                                                                           - `cargo check --all-targets` — <pass | fail: summary>
- `cargo clippy --all-targets` — <pass | fail: summary>
- `cargo test` — <N passed, M skipped — note if skips are vacuous>                                                                                                                        - `cargo fmt --check` — <pass | fail>

## Findings                                                                                                                                                                               
### 🔴 Blockers                                                                                                                                                                           **`src/path/file.rs:123` — one-line title**
Breaks when: <concrete input/state → wrong output, panic, or corruption>
Fix: <direction>                                                                                                                                                                          
### 🟡 Should fix
(same format)                                                                                                                                                                             
### 🔵 Nits
(same format, one line each)

## Summary                                                                                                                                                                                2–4 sentences: verdict (ready to commit / needs changes), the single most impor
finding, and what was checked and came back clean.                                                                                                                                        ```
                                                                                                                                                                                          Verdict rule: any 🔴 → needs changes. Otherwise ready, with 🟡s listed as follo
                                                                                                                                                                                          Only report on code the change touches. Pre-existing issues outside the diff ge
most one line under Summary, and only if they are serious.

An empty findings section is a valid result — say "clean" and stop. A padded report                                                                                                       with invented nits is worse than a short one; the author should be able to trus
every bullet is worth reading.                                                                                                                                                            
Notes on the choices, so you can adjust before saving:                                                                                                                                    
- Two modes in one skill — the scope table in Step 2 is the only thing that differs between PR and pre-commit, so one file covers both instead of duplicating the checklist.
- Step 5 exists because of this repo — a generic Rust reviewer would flag this codebase's deliberate assert!-heavy parsers, Box::leaked statics, and changed-gated writes as defects. The skill forces it to read CLAUDE.md first and check invariants instead of style.
- Vacuous-test warning in the report template — this repo's save-shape tests pass silently without saves/ fixtures; the reviewer is told to surface that rather than imply coverage.
