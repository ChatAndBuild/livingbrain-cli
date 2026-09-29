---
name: livingbrain-custom-commands
description: How to author custom commands for the livingbrain CLI using the co-generated SDK.
---

# Custom Commands for `livingbrain`

## Overview

The `livingbrain` CLI supports user-authored custom commands that are
compiled into the binary alongside the auto-generated API commands.
Custom commands get a fully-wired SDK client that inherits the CLI's
auth, retries, TLS, base URL, and global headers — zero configuration required.

## Architecture

```
cli/livingbrain/custom.rs    ← Your command handlers (protected by .fernignore)
cli/livingbrain/sdk.rs       ← Generated bridge: client() + block_on()
cli/livingbrain/main.rs      ← Generated entrypoint (calls custom::register)
livingbrain-sdk/             ← Co-generated typed SDK crate
livingbrain-types/           ← Co-generated typed model crate
```

## Adding a Custom Command

### 1. Edit `cli/livingbrain/custom.rs`

This file is protected by `.fernignore` — `fern generate` will never
overwrite it. Register commands in the `register()` function:

```rust
use livingbrain_sdk::api::*;

pub fn register(app: CliApp) -> CliApp {
    let app = app.command(
        clap::Command::new("get")
            .about("Get a brain")
            .arg(clap::Arg::new("brainId").required(true))
        ,
        |matches, ctx| {
            let brain_id = matches.get_one::<String>("brainId").unwrap();
            let client = super::sdk::client(ctx);
            let result = super::sdk::block_on(
                client.brains.get(brain_id),
            )?;
            println!("{}", serde_json::to_string_pretty(&result).unwrap());
            Ok(())
        },
    );
    app
}
```

Then build and test:
```bash
cargo build
livingbrain get <brainId>
```

### 2. Available SDK Clients

The `super::sdk::client(ctx)` call returns a `livingbrain_sdk::api::Client`
with the following sub-clients:

| Field | Type | Description |
|-------|------|-------------|
| `client.brains` | `livingbrain_sdk::api::BrainsClient` | brains operations |
| `client.captures` | `livingbrain_sdk::api::CapturesClient` | captures operations |
| `client.workspace` | `livingbrain_sdk::api::WorkspaceClient` | workspace operations |
| `client.webhooks` | `livingbrain_sdk::api::WebhooksClient` | webhooks operations |
| `client.telegram` | `livingbrain_sdk::api::TelegramClient` | telegram operations |
| `client.slack` | `livingbrain_sdk::api::SlackClient` | slack operations |

### 3. Key Patterns

**Get the SDK client** (execution-sharing, fully authenticated):
```rust
let client = super::sdk::client(ctx);
```

**Run an async SDK call from a sync handler:**
```rust
let result = super::sdk::block_on(
    client.some_resource.some_method(args),
)?;
```

**Use typed models for request/response serialization:**
```rust
use livingbrain_sdk::api::*;
```

### 4. Authentication

Custom commands automatically inherit the CLI's authentication.
The following auth schemes are configured:

- **apiKey** (header): env `LIVING_BRAIN_API_KEY`

No manual auth wiring is needed in custom command handlers.

## Regeneration Safety

| File | Regenerated? | Notes |
|------|-------------|-------|
| `cli/livingbrain/custom.rs` | **No** | Protected by `.fernignore` |
| `cli/livingbrain/sdk.rs` | Yes | Bridges AppContext → SDK client |
| `cli/livingbrain/main.rs` | Yes | Calls `custom::register(app)` |
| `livingbrain-sdk/` | Yes | Co-generated typed SDK crate |
| `livingbrain-types/` | Yes | Co-generated typed models |

After running `fern generate`, your `custom.rs` is preserved. All
generated code (SDK, types, glue, main.rs) is updated to match the
latest API spec. If the SDK surface changes (renamed methods, new
sub-clients), update your `custom.rs` to match.

## Build & Test

```bash
# Build the CLI (includes custom commands)
cargo build

# Run your custom command
livingbrain <your-command> [args]

# Run with verbose output for debugging
RUST_LOG=debug livingbrain <your-command> [args]
```
