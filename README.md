# llm

A blazing fast, single-binary CLI for chatting with any OpenAI-compatible LLM from your terminal. No runtime dependencies. No Docker. No Node. Just a **10 MB** binary that works on any Linux machine.

```
$ llm -s "What command shall I use to see all files and folders including hidden ones"
ls -la
```

> **macOS and Windows releases coming soon.**

## Install

```sh
wget ... | bash
```

That's it. The `llm` binary is dropped into `~/.local/bin`. Make sure it's in your `PATH`:

```sh
export PATH="$HOME/.local/bin:$PATH"
```

## Usage

**Single shot** — one question, one answer, done:

```sh
llm -s "Explain the difference between TCP and UDP"
```

**Interactive mode** — multi-turn conversation with full context memory:

```sh
$ llm
>> What command lists all files
ls
>> And hidden ones
ls -la
>> exit
```

You can also seed the conversation with an initial prompt:

```sh
llm "What is the capital of France"
```

**REPL commands:** `exit`, `quit`, `Ctrl-D`, or `Ctrl-C` to leave.

## Config

On first run, `llm` walks you through setup interactively — base URL, model, and API key. The config is saved to:

```
~/.config/llm-cli/config.toml
```

Edit it directly to change settings:

```toml
base_url = "https://api.openai.com/v1"
api_key = "sk-..."
model = "gpt-5.4-mini"
system_instruction = "You are an AI CLI assistant. Your answers should be short and concise. If the user asks for command, output just command without any comments."
```

| Option | Description |
|---|---|
| `base_url` | Any OpenAI-compatible API endpoint (OpenAI, Ollama, LM Studio, vLLM, etc.) |
| `api_key` | Bearer token for authentication |
| `model` | Model ID accepted by your endpoint (e.g. `gpt-4o-mini`, `claude-opus-4`) |
| `system_instruction` | System prompt prepended to every conversation |

**Environment overrides** — set these to override the config file without editing it:

```sh
LLM_CLI_API_KEY=sk-...
LLM_CLI_BASE_URL=
LLM_CLI_MODEL=
```

## Uninstall

Remove the binary:

```sh
rm ~/.local/bin/llm
```

Remove the config:

```sh
rm -r ~/.config/llm-cli
```

## Development

```sh
git clone repo
cd llm-cli
cargo build
cargo test
```

The binary is built at `target/debug/llm`. For a release build:

```sh
cargo build --release
```

## License

MIT
