# Repository Guidelines

## Project Structure & Module Organization
- `src/main.rs` is the current entry point. As the client grows, prefer splitting into focused modules such as `src/client/`, `src/models/`, and `src/endpoints/`, with shared exports from `src/lib.rs`.
- `docs/requirements_spec.md` is the frozen baseline for behavior; tie implementation and review notes back to FR/NFR IDs when possible.
- `target/` contains build artifacts and is intentionally ignored.
- Put integration tests in `tests/` (for example, `tests/materials_summary.rs` for one route group).

## Build, Test, and Development Commands
- `cargo check` — fast compile/type validation without building release artifacts.
- `cargo run` — runs the local binary for quick manual checks.
- `cargo test` — executes unit and integration tests.
- `cargo test -- --nocapture` — useful when debugging failing tests with log output.
- `cargo fmt --all` — formats all Rust code.
- `cargo clippy --all-targets --all-features -- -D warnings` — enforces lint cleanliness.

## Coding Style & Naming Conventions
- Use Rust 2024 edition idioms and `rustfmt` defaults (4-space indentation, standard brace/line breaks).
- Naming: `snake_case` for functions/modules/files, `PascalCase` for structs/enums/traits, `UPPER_SNAKE_CASE` for constants.
- Keep the public surface async-first (Tokio-oriented); avoid introducing blocking wrappers.
- Prefer explicit error propagation (`Result<T, E>`) and typed errors over panics in library paths.

## Testing Guidelines
- Keep unit tests close to implementation using `#[cfg(test)]`.
- Use `tests/` for integration and contract tests; one file per endpoint or route group is preferred.
- Baseline requirements call for unit tests (serialization/error mapping), contract tests per OpenAPI operation, and opt-in smoke tests gated by `MP_API_KEY` or `PMG_MAPI_KEY`.
- Before opening a PR, run: `cargo fmt --all && cargo clippy --all-targets --all-features -- -D warnings && cargo test`.

## Commit & Pull Request Guidelines
- The repository currently has no commit history on `master` (checked Feb 9, 2026), so establish `Conventional Commits` now.
- Commit format: `type(scope): imperative summary` (example: `feat(materials): add summary query params`).
- Keep commits focused and include relevant tests/docs with behavior changes.
- PRs should include: a concise problem statement, linked requirement IDs (for example `FR-042`), commands run for validation, and any config/env changes.

## Security & Configuration Tips
- Never commit API keys or secrets.
- Default to HTTPS endpoints; only allow insecure HTTP for explicit local test scenarios.
