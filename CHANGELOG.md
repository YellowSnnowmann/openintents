# Changelog

All three SDKs share one version. Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- Spec: browser profiles (`/profiles`), browser sessions (`/sessions`, with
  commands, screenshots and release, and a `sessionEvent` webhook), and live
  event streams for intents and sessions. Marked preview: not yet available.
- Spec: what the API already serves but the spec left out: customers
  (`/customers`), top-ups (`/topups`), `customer` on intents, and `held` and
  `customer` on the balance.

## [0.1.0]

### Added

- OpenAPI spec for the v1 preview API (`spec/openapi.yaml`).
- TypeScript SDK and `openintents` CLI (`@tinyhumansai/openintents`).
- Python SDK with sync and async clients (`openintents`).
- Rust SDK with an optional CLI (`openintents`).
- Webhook signature verification in every SDK, with a shared test vector.
- Agent skill and Claude Code plugin.
