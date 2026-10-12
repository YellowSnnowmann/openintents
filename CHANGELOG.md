# Changelog

All three SDKs share one version. Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- Spec, marked preview (designed, not yet served): browser profiles
  (`/profiles`), browser sessions (`/sessions`, with commands, screenshots and
  release, and a `sessionEvent` webhook), event streams for intents and
  sessions, and `lease` on the error body for `profile_busy`.
- Spec: what the API already serves but the spec left out: customers
  (`/customers`, with the `external_id` filter), top-ups (`/topups`),
  `customer` on intents (and the `customer` filter on `GET /intents`), `held`
  and `customer` on the balance (and `?customer=`), and the server's limits
  and idempotency rules.
- Spec conventions: test and live keys share customers, intents, balances
  and profiles; unknown request fields are refused; new enum values may
  appear.

## [0.1.0]

### Added

- OpenAPI spec for the v1 preview API (`spec/openapi.yaml`).
- TypeScript SDK and `openintents` CLI (`@tinyhumansai/openintents`).
- Python SDK with sync and async clients (`openintents`).
- Rust SDK with an optional CLI (`openintents`).
- Webhook signature verification in every SDK, with a shared test vector.
- Agent skill and Claude Code plugin.
