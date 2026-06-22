# llm-cli

Quick AI answers from the terminal, using any OpenAI-compatible Chat Completions
endpoint.

```
$ llm -s "What command shall I use to see all files and folders including hidden ones"
ls -la
```

## Install

```sh
cargo install --path .
```

The binary is installed as `llm`.

## Usage

**Single shot** (`-s` / `--single`): one request, print the answer, exit.

```sh
llm -s "Explain the difference between TCP and UDP"
```

**Interactive REPL** (default): opens a multi-turn conversation. Pass an
optional initial prompt to seed it, then keep typing at the `>> ` prompt.

```sh
llm "What command lists all files"
ls
>> And hidden ones
ls -la
>> exit
```

REPL commands:

- `exit` / `quit` (case-insensitive) — leave the REPL
- `Ctrl-D` — leave the REPL
- `Ctrl-C` — leave the REPL
- empty line — skipped, no request sent

## First run

If no config exists, `llm` prompts for the base URL, model id, and API key,
writes `~/.config/llm-cli/config.toml` (mode `0600` on Unix) with a default
system instruction, prints a message, and exits. Re-run `llm` to start.

## Configuration

Config file: `$XDG_CONFIG_HOME/llm-cli/config.toml`
(or `~/.config/llm-cli/config.toml`)

```toml
base_url = "https://api.openai.com/v1"
api_key = "sk-..."
model = "gpt-4o-mini"
system_instruction = "You are an AI CLI assistant. Your answers should be short and concise. If the user asks for command, output just command without any comments."
```

### Environment overrides

Non-empty values for any of these environment variables override the file value:

- `LLM_CLI_API_KEY`
- `LLM_CLI_BASE_URL`
- `LLM_CLI_MODEL`

```sh
LLM_CLI_MODEL=claude-opus-4 llm -s "hi"
```

## Errors

Provider errors (non-2xx, network failures, malformed responses) are printed to
stderr. In single mode they exit with code 1; in the REPL they print and the
session continues.

## License

MIT
