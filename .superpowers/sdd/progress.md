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

(Each entry: `Task N: complete (commits <base7>..<head7>, review clean)` — appended as tasks finish.)
