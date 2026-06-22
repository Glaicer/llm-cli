# llm-cli — SDD Progress Ledger

> Durable progress log for the llm-cli implementation plan. Updated after each task's review comes back clean.

## Plan
`docs/superpowers/plans/2026-06-22-llm-cli.md` — 9 tasks, Rust CLI with interactive REPL + single-shot modes, OpenAI-compatible chat completions.

## Global Constraints (binding)
- Binary name `llm` (package `llm-cli`).
- Config at `$XDG_CONFIG_HOME/llm-cli/config.toml` via `dirs::config_dir()`.
- Default system instruction: "You are an AI CLI assistant. Your answers should be short and concise. If the user asks for command, output just command without any comments."
- Modes: default = interactive REPL; `-s`/`--single` = one request and exit. `-s` is single-only.
- Non-streaming responses.
- First run with no config: interactive prompts for base URL/model/API key, write config with default system instruction, exit so user restarts.
- Env overrides: `LLM_CLI_API_KEY`, `LLM_CLI_BASE_URL`, `LLM_CLI_MODEL` (non-empty replaces file value).
- In-memory history per process.
- Provider errors to stderr only (no notify-send); single-mode errors exit 1, REPL errors print and continue.
- Config file written with mode 0600 on Unix.
- DRY, YAGNI, TDD, frequent commits.

## Task Status

- Task 1: complete (commits 3f74e81..1b6660f, review clean)
- Task 2: complete (commits 1b6660f..5ff55f6, review clean). Plan command note: brief uses `cargo test --lib` but the crate is binary-only (no src/lib.rs). Accepted substitution: use `cargo test <module>` (e.g. `cargo test config`) for all tasks. Applied to future dispatches.
- Task 3: complete (commits 5ff55f6..b27905a, review clean).
- Task 4: complete (commits b27905a..1a164b7, review clean).
- Task 5: complete (commits 1a164b7..9ab9e38, review clean).
- Task 6: complete (commits 9ab9e38..32edb7c, review clean).
- Task 7: complete (commits 32edb7c..f5eac54, review clean).
- Task 8: complete (commits f5eac54..93e74de, review clean).
- Task 9: complete (commits 93e74de..ed06830, review clean). Includes `&PathBuf → &Path` sweep and `to_toml` clippy fix in `src/config.rs` to pass `cargo clippy -- -D warnings`.
- Final whole-branch review: Ready to merge. No Critical/Important issues; 7 Minor polish items (use-statement consolidation, HTTP timeout, empty-input prompts, case-insensitive exit, history error handling, AppError enum, optional README/zeroize) all outside spec scope.
- Polish pass: complete (commits ed06830..6075c3d, 4 commits). Items 1-6 implemented: consolidated `use` block + `prompt_required` with TDD test (config), 60s HTTP timeout with `unwrap_or_else` fallback (client), case-insensitive `exit`/`quit` with TDD assertions + non-fatal history errors (repl), `AppError::MissingPrompt` (main). 17/17 tests passing, clippy clean. Item 7 (README/zeroize/wiremock) explicitly skipped per user scope.
