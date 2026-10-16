# Changelog

## [0.2.1] - 2026-10-16

- Fixed: authenticated commands now send `x-api-key`. The spec declared auth as `apiKey AND subject`, but `x-subject-id` is a global header rather than an auth scheme, so per-endpoint auth routing found no `subject` provider and attached no API key (only `x-subject-id` went out and every call returned 401). The auth requirement is now the API key alone.
- `brains runBrief` is typed as nullable: the API returns `null` when the last 7 days had no activity. The SDK's `run_brief` now returns `Option<BriefRunResponse>`.
- README documents the required `LIVING_BRAIN_SUBJECT_ID` (a UUID) alongside the API key.
- Added `tests/auth_headers.rs`, which runs `livingbrain brains list` against a mock server and asserts both `x-api-key` and `x-subject-id` are sent, via env vars and via flags.

## [0.2.0] - 2026-10-14

- Regenerated from the Oct 14 OpenAPI export.
- `Page` no longer has an `embedding` field (the API stopped returning it).
- `SearchResult` has a new required `matchedOn` field: `semantic`, `keyword`, or `both`.
- Brain deletion is now documented as a soft delete.

## [0.1.0] - 2026-09-29

