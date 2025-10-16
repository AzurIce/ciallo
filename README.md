# Ciallo

A Rust CLI tool for executing commands with webhook notifications.

## Features

- Execute commands defined in project configuration
- Capture stdout/stderr with configurable output handling
- Send execution results to webhooks (currently supports Feishu)
- Separate global and project-level configurations
- Track command execution time

## Installation

```bash
cargo install --path .
```

## Configuration

### Global Config (`~/.config/ciallo/config.toml`)

Define reusable hooks that can be used across all projects:

```toml
[hook.feishu.feishu]
webhook_url = "https://open.feishu.cn/open-apis/bot/v2/hook/your-webhook-url"

[hook.another_hook.feishu]
webhook_url = "https://open.feishu.cn/open-apis/bot/v2/hook/another-webhook"
```

### Project Config (`ciallo.toml`)

Define commands for your project:

```toml
[cmd.check]
command = "cargo check"
hooks = ["feishu"]

[cmd.bench]
command = "cargo bench"
stdout = "piped"  # Options: null, piped, captured (default: piped)
stderr = "piped"  # Options: null, piped, captured (default: piped)
hooks = ["feishu", "another_hook"]
```

## Usage

Execute a command defined in `ciallo.toml`:

```bash
# Run 'cmd.check' defined in `ciallo.toml`
ciallo check

# Run 'cmd.bench' defined in `ciallo.toml`
ciallo bench

# Use a custom config file
ciallo -c /path/to/config.toml check
```
## Webhook payload json format

```json
{"msg":"xxx"}
```
