# Living Brain Service CLI Reference

Full command reference for `livingbrain`.

## Commands

- [`livingbrain brains`](#livingbrain-brains)
- [`livingbrain captures`](#livingbrain-captures)
- [`livingbrain slack`](#livingbrain-slack)
- [`livingbrain telegram`](#livingbrain-telegram)
- [`livingbrain webhooks`](#livingbrain-webhooks)
- [`livingbrain workspace`](#livingbrain-workspace)

---

### `livingbrain brains`

#### `livingbrain brains bulk-stats`

Aggregate counts for many brains in one round trip. Pass a comma-separated `ids` list; brains the caller cannot access are omitted rather than erroring.

`GET /v1/brains/stats`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--ids` | `string` | No | Comma-separated brain ids, e.g. `ids=a,b,c`. Unowned ids are omitted. |

#### `livingbrain brains create`

Create a new brain for the calling subject. A brain is an isolated knowledge vault: every capture, page, and connection belongs to exactly one brain. The `x-subject-id` header determines who owns it.

`POST /v1/brains`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `livingbrain brains get`

Retrieve a single brain by ID, including its name, description, and brief settings.

`GET /v1/brains/{brainId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |

#### `livingbrain brains list`

List every brain owned by the calling subject, most recently updated first.

`GET /v1/brains`

#### `livingbrain brains remove`

Permanently delete a brain along with all of its pages, sources, connections, and change history. This cannot be undone.

`DELETE /v1/brains/{brainId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |

#### `livingbrain brains run-brief`

Generate this brain's daily brief immediately instead of waiting for its scheduled run. Returns the brief content once generation completes.

`POST /v1/brains/{brainId}/brief/run`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |

#### `livingbrain brains stats`

Aggregate counts for a single brain — pages, sources, connections, and unacknowledged changes. Useful for dashboard headline numbers.

`GET /v1/brains/{brainId}/stats`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |

#### `livingbrain brains update`

Update a brain's mutable fields, such as its name or description. Omitted fields are left unchanged.

`PATCH /v1/brains/{brainId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `livingbrain brains update-brief-settings`

Update the daily-brief schedule and delivery settings for a brain, including whether briefs are enabled and what time they are generated.

`PATCH /v1/brains/{brainId}/brief-settings`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `livingbrain captures`

#### `livingbrain captures capture`

Push a single item into a brain for ingestion. Accepts notes, URLs, raw text, chat transcripts, and PDF/DOCX/PPTX documents. Capture is asynchronous: this returns an ingest source whose `status` you can poll or receive via webhook.

`POST /v1/brains/{brainId}/captures`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `livingbrain captures capture-batch`

Push many captures in a single request. Each item is processed independently, so a malformed entry fails on its own without rejecting the whole batch.

`POST /v1/brains/{brainId}/captures/batch`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `livingbrain captures capture-chat-turn`

Submit a single chat turn for worthiness classification. The service decides whether the turn contains durable knowledge and only captures it if so, which keeps conversational filler out of the brain.

`POST /v1/brains/{brainId}/captures/chat-turn`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `livingbrain captures list-sources`

List ingest sources for a brain, optionally filtered by processing status. Use this to surface an ingestion queue or to find sources that failed to compile.

`GET /v1/brains/{brainId}/sources`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--limit` | `integer` | No |  |
| `--offset` | `integer` | No |  |
| `--status` | `pending | compiling | completed | failed` | No |  |
| `--ids` | `string` | No | Comma-separated source ids for a targeted lookup (e.g. resolving a page’s source refs regardless of age) |

#### `livingbrain captures remove-source`

Delete an ingest source. Pages already derived from the source are retained unless you delete them separately.

`DELETE /v1/brains/{brainId}/sources/{sourceId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--source-id` | `string` | Yes | Ingest source id |

#### `livingbrain captures retry-source`

Retry compiling an ingest source that previously failed. Only sources in a failed state can be retried.

`POST /v1/brains/{brainId}/sources/{sourceId}/retry`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--source-id` | `string` | Yes | Ingest source id |

---

### `livingbrain slack`

#### `livingbrain slack bind`

Connect a Slack channel to a brain so it receives brain notifications. Configure which notifications it receives with `updateBinding`.

`POST /v1/brains/{brainId}/slack/bindings`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `livingbrain slack channels`

List the channels visible to the Living Brain bot in a Slack installation. `isMember` tells you whether the bot has already been added to the channel.

`GET /v1/brains/{brainId}/slack/channels`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--installation-id` | `string` | Yes | Slack installation id returned by the status endpoint |

#### `livingbrain slack confirm-oauth`

Complete a Slack installation by exchanging the `code` and `state` Slack returned to your redirect URL. Returns the new installation ID.

`POST /v1/brains/{brainId}/slack/oauth/confirm`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `livingbrain slack disconnect-installation`

Disconnect a Slack workspace from a brain and remove all of its channel bindings.

`DELETE /v1/brains/{brainId}/slack/installations/{installationId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--installation-id` | `string` | Yes | Slack installation id |

#### `livingbrain slack start-oauth`

Create a signed Slack OAuth installation URL for a brain. Redirect the user to `authorizationUrl`; Slack sends them back to your redirect URL with a `code` and `state` to pass to `confirmOauth`. The signed state expires at `expiresAt`.

`POST /v1/brains/{brainId}/slack/oauth/start`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |

#### `livingbrain slack status`

Return every Slack workspace installed for a brain along with its channel bindings. Use the installation IDs returned here to list channels or create bindings.

`GET /v1/brains/{brainId}/slack`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |

#### `livingbrain slack unbind`

Disconnect a single Slack channel from a brain. The Slack installation and any other bindings are unaffected.

`DELETE /v1/brains/{brainId}/slack/bindings/{bindingId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--binding-id` | `string` | Yes | Slack channel binding id |

#### `livingbrain slack update-binding`

Update which notifications a bound Slack channel receives — daily briefs, new connections, and page changes. Omitted fields are left unchanged.

`PATCH /v1/brains/{brainId}/slack/bindings/{bindingId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--binding-id` | `string` | Yes | Slack channel binding id |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `livingbrain telegram`

#### `livingbrain telegram connect`

Connect a Telegram bot to a brain so messages sent to the bot are captured automatically.

`POST /v1/brains/{brainId}/telegram`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `livingbrain telegram disconnect`

Disconnect the Telegram bot from a brain. Already-captured messages are retained.

`DELETE /v1/brains/{brainId}/telegram`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |

#### `livingbrain telegram status`

Return the current Telegram bot connection status for a brain, including the connected bot's username when one is linked.

`GET /v1/brains/{brainId}/telegram`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |

---

### `livingbrain webhooks`

#### `livingbrain webhooks create`

Register a webhook endpoint and receive its signing secret. The secret is returned only on creation — store it immediately, as it cannot be retrieved again.

`POST /v1/webhooks`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `livingbrain webhooks list`

List all webhook subscriptions for the calling subject. Signing secrets are not included.

`GET /v1/webhooks`

#### `livingbrain webhooks remove`

Delete a webhook subscription. Deliveries stop immediately.

`DELETE /v1/webhooks/{id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--id` | `string` | Yes | Webhook subscription id |

#### `livingbrain webhooks update`

Update a webhook subscription's URL or subscribed event types. The signing secret is unchanged.

`PATCH /v1/webhooks/{id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--id` | `string` | Yes | Webhook subscription id |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `livingbrain workspace`

#### `livingbrain workspace ack-all`

Acknowledge every outstanding change event in one call, and return how many were cleared.

`POST /v1/brains/{brainId}/changes/ack-all`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |

#### `livingbrain workspace ack-change`

Acknowledge a single change event, removing it from unacknowledged views.

`POST /v1/brains/{brainId}/changes/{changeId}/ack`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--change-id` | `string` | Yes | Change event id |

#### `livingbrain workspace ack-connection`

Dismiss a connection so it stops appearing in unacknowledged views. The connection itself is preserved.

`POST /v1/brains/{brainId}/connections/{connectionId}/ack`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--connection-id` | `string` | Yes | Connection id |

#### `livingbrain workspace archive`

Archive a page. Archived pages are hidden from lists and search until restored, but their content, versions, and connections are kept. Use `restore` to bring the page back.

`POST /v1/brains/{brainId}/pages/{slug}/archive`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--slug` | `string` | Yes | Page slug |

#### `livingbrain workspace buckets`

Return per-bucket page counts for the brain — the fastest way to render bucket navigation with badges.

`GET /v1/brains/{brainId}/buckets`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |

#### `livingbrain workspace changes`

List change-feed events with offset pagination. This is the primary way to sync an external system: poll with `since`, or filter to `unackedOnly` to build an inbox.

`GET /v1/brains/{brainId}/changes`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--limit` | `integer` | No |  |
| `--offset` | `integer` | No |  |
| `--unacked-only` | `boolean` | No |  |
| `--page-id` | `string` | No |  |
| `--since` | `string` | No | ISO timestamp |

#### `livingbrain workspace delete-permanently`

Permanently delete a page and every record that depends on it, including its versions and connections. This cannot be undone.

`DELETE /v1/brains/{brainId}/pages/{slug}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--slug` | `string` | Yes | Page slug |
| `--permanent` | `true` | Yes | Explicit irreversible-deletion confirmation |

#### `livingbrain workspace edit-content`

Manually overwrite a page's content. This creates a new version, so the previous content stays retrievable through the version history.

`PATCH /v1/brains/{brainId}/pages/{slug}/content`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--slug` | `string` | Yes | Page slug |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `livingbrain workspace export-markdown`

Export the entire brain as a single markdown bundle, suitable for backup or for handing to another tool.

`GET /v1/brains/{brainId}/export/markdown`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--include-deprecated` | `boolean` | No |  |

#### `livingbrain workspace get-page`

Read a single page by slug. This bumps the page's `lastReferencedAt`, which feeds the brain health score — use it when a human or agent actually consumes the content.

`GET /v1/brains/{brainId}/pages/{slug}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--slug` | `string` | Yes | Page slug |
| `--include-deprecated` | `boolean` | No |  |

#### `livingbrain workspace get-profile`

Return the brain profile — the standing directives and context that shape how the brain interprets and organizes new captures.

`GET /v1/brains/{brainId}/profile`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |

#### `livingbrain workspace get-score`

Return the brain's health score with its contributing factors and metrics. The score reflects coverage, freshness, and connectedness.

`GET /v1/brains/{brainId}/score`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |

#### `livingbrain workspace graph`

Return a renderable subgraph of nodes and edges for visualizing how the brain's pages relate. Use `nodeLimit` to keep the payload small enough to draw.

`GET /v1/brains/{brainId}/graph`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--node-limit` | `integer` | No |  |

#### `livingbrain workspace index`

Return a compact index of the brain's top pages, formatted for injection into an LLM prompt. This is the intended way to give an agent working context without sending the whole brain.

`GET /v1/brains/{brainId}/index`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--limit` | `integer` | No |  |

#### `livingbrain workspace list-connections`

List connections across the whole brain with offset pagination. Filter by strength, or restrict to connections the user has not yet dismissed.

`GET /v1/brains/{brainId}/connections`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--limit` | `integer` | No |  |
| `--offset` | `integer` | No |  |
| `--unacked-only` | `boolean` | No |  |
| `--strength` | `weak | medium | strong` | No | Minimum strength (inclusive) |

#### `livingbrain workspace list-pages`

List pages in a brain with offset pagination. Filter by bucket or status, and optionally include deprecated pages that are normally hidden.

`GET /v1/brains/{brainId}/pages`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--limit` | `integer` | No |  |
| `--offset` | `integer` | No |  |
| `--include-deprecated` | `boolean` | No |  |
| `--include-archived` | `boolean` | No | Include archived pages in the results. Archived pages are hidden from lists and search by default. |
| `--bucket` | `inbox | notes | ideas | projects` | No |  |
| `--status` | `active | deprecated | stale | contested | archived` | No |  |

#### `livingbrain workspace move-bucket`

Move a page into a different bucket. Buckets are the brain's top-level organizational grouping.

`PATCH /v1/brains/{brainId}/pages/{slug}/bucket`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--slug` | `string` | Yes | Page slug |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `livingbrain workspace page-connections`

List the connections attached to a single page, with the related page inlined on each connection.

`GET /v1/brains/{brainId}/pages/{slug}/connections`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--slug` | `string` | Yes | Page slug |

#### `livingbrain workspace page-versions`

List a page's version history, newest first. Every content edit and automated recompile produces a version.

`GET /v1/brains/{brainId}/pages/{slug}/versions`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--slug` | `string` | Yes | Page slug |

#### `livingbrain workspace pin`

Pin or unpin a page. Pinned pages are prioritized in the prompt index and are exempt from deprecation.

`POST /v1/brains/{brainId}/pages/{slug}/pin`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--slug` | `string` | Yes | Page slug |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `livingbrain workspace profile-suggestions`

Suggest profile fields inferred from what the brain already contains. Best-effort: returns an empty set when there is not yet enough signal.

`POST /v1/brains/{brainId}/profile/suggestions`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |

#### `livingbrain workspace recover`

Recover a deprecated page and return it to active status.

`POST /v1/brains/{brainId}/pages/{slug}/recover`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--slug` | `string` | Yes | Page slug |

#### `livingbrain workspace rename`

Rename a page's title and slug. Existing connections follow the page, but any external references to the old slug will break.

`POST /v1/brains/{brainId}/pages/{slug}/rename`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--slug` | `string` | Yes | Page slug |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `livingbrain workspace resolve`

Resolve a contested page. A page becomes contested when newly captured material contradicts what the page already asserts; resolving records which version wins.

`POST /v1/brains/{brainId}/pages/{slug}/resolve`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--slug` | `string` | Yes | Page slug |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `livingbrain workspace restore`

Restore an archived page and return it to active status, so it reappears in lists and search.

`POST /v1/brains/{brainId}/pages/{slug}/restore`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--slug` | `string` | Yes | Page slug |

#### `livingbrain workspace reviewed`

Legacy alias for recovering a deprecated page. Prefer `recover` in new code; this endpoint is retained for backwards compatibility.

`POST /v1/brains/{brainId}/pages/{slug}/reviewed`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--slug` | `string` | Yes | Page slug |

#### `livingbrain workspace score-history`

Return the score trend over a time range, including the delta versus the start of the period.

`GET /v1/brains/{brainId}/score/history`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--range` | `7d | 30d | 90d` | No |  |

#### `livingbrain workspace score-suggestions`

Return concrete, ranked suggestions for raising the brain's health score, derived from whichever factors are currently weakest.

`POST /v1/brains/{brainId}/score/suggestions`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |

#### `livingbrain workspace search-pages`

Search pages using combined semantic and keyword matching. Results are ranked by relevance and include a snippet showing why each page matched.

`POST /v1/brains/{brainId}/search`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `livingbrain workspace update-profile`

Update the brain profile's directives. These take effect on subsequent captures, not retroactively.

`PATCH /v1/brains/{brainId}/profile`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--brain-id` | `string` | Yes | Brain id |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

## Global flags

These flags are available on every command:

| Flag | Description |
|------|-------------|
| `--dry-run` | Print the HTTP request without sending it |
| `--json <JSON\|->` | Supply the request body as JSON (or `-` for stdin) |
| `--params <JSON>` | Merge extra parameters as JSON |
| `--format <json\|table\|yaml\|csv>` | Output format (default: `json`) |
| `--output <PATH>` | Write binary responses to a file |
| `--base-url <URL>` | Override the API base URL |
| `--no-extract` | Print the full response body instead of the `x-fern-sdk-return-value` extraction |
| `--no-retry` | Disable retries declared by `x-fern-retries`, including network errors |
| `-q, --quiet` | Suppress stdout on success |
| `-h, --help` | Print help |
| `-V, --version` | Print version |

Operations the spec describes how to page (via `x-fern-pagination` or a root `page_token` parameter) also accept:

| Flag | Description |
|------|-------------|
| `--page-all` | Auto-paginate and stream all results |
| `--page-limit <N>` | Max pages to fetch (default: `10`) |
| `--page-delay <MS>` | Delay between page fetches in milliseconds (default: `100`) |
| `--no-pager` | Disable the pager even on interactive terminals |

Operations the spec marks as streaming (via `x-fern-streaming`) also accept:

| Flag | Description |
|------|-------------|
| `--no-stream` | Buffer the streaming response and print it as a single value once complete |
