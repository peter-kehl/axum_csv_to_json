# Requirements
- Receive a CSV file as a POST body. Parse, return as an array of JSON objects.
- Document the API.
- How to test?

# Optional requirements
Store in Postgres.

# Preferred technologies
Tokio, Axum

# Roadmap
- Tokio webservice
- optional: OpenAPI/Swagger with Tokio, so that we & potentially clients can test this with Postman
- parse CSV with Serde-based https://docs.rs/csv/latest/csv
- generate JSON with https://docs.rs/tokio-serde-json/latest/tokio_serde_json
- Tests
-- manual with a generated OpenAPI schema and Postman
-- reproducible with a local Postgres: https://github.com/faokunega/pg-embed (but unmaintained for
   10 months! - however, it depends on https://github.com/zonkyio/embedded-postgres-binaries, which
   has 4 contributors and last commit 11 days ago)
-- reproducible with a Docker image that contains Postgres
-- with https://docs.rs/axum-test-helper/0.1.0/axum_test_helper (updated 4 weeks ago, 1 contributor), or 
-- with https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs (more bolierplate)
- consider replacing Serde-based CSV parser with a custom parser later (may be worthwhile if no other dependancy needs Serde, then avoiding Serde speeds up build times, hence local dev & CI times)
- consider generating JSON on our own later (again, more worthwhile if we don't need Serde for anything else)
- local & CI to be reproducible
-- `cargo-run-bin` with `Makefile.toml` to make build & CI process independent of locally installed cargo packages
-- auto-generate OpenAPI and compare it to what is in GIT (with `git diff`), fail if different

# Tradeoffs and Decisions
- Postgres with https://docs.rs/tokio-postgres/latest/tokio_postgres - chosen because it's a part of Tokio project => reliable.
-- Otherwise, based on https://project-awesome.org/rust-unofficial/awesome-rust:
--- https://github.com/launchbadge/sqlx:  it allows type-safe queries, and several SQL back-ends
--- Postgres only: https://github.com/sfackler/rust-postgres
- OpenAPI (good for manual/client testing). Considered:
-- https://github.com/softprops/openapi - too low-level for this task
-- https://github.com/glademiller/openapiv3 (last commit 4 months ago, 18 contributors) - but not Axum-specific, and the tests don't involve Axum either
-- Based on https://github.com/tokio-rs/axum/issues/50, I've considered
--- https://github.com/jsdw/seamless (last commit 5 months ago, tested with Tokio 1.1.0 while the current is 1.18.2!),
--- https://github.com/oxidecomputer/dropshot (last commit a few days ago, using more current Tokio 1.16; it seems heavyweight),
--- https://github.com/jakobhellermann/axum_openapi (10 months old, one maintainer only),
--- https://github.com/juhaku/utoipa (last commit 6 days ago, 4 contributors) --> chosen
- Write to Postgres
-- Do we have a defined schema for each client/company/data source, shared across their uploads? If yes:
--- 1. Schema can change over time, so each CSV column info would have to have its applicability period (two timestamps, and/or a client/company-specific version number/string). Or
--- 2. Everytime a client/company changes their schema, we create a new endpoint (and a new DB table).
-- 3. Alternatively, we have a dynamic schema generated from CSV, independent, per-upload.
-- Out of scope: merging with existing data (which would involve flagging conflicting/unmergable entries and human intervention). Hence:
-- Each upload creates a new subset of entries, all associated with the same "upload" entry, and we return a new ID of that upload.
-- This MVP has only one endpoint, hence #3 from above. Treating all values as texts. Four tables (mutli-dimensional flattened). Descriptions are Postgres-agnostic:
--- `uploads: id (generated primary), uploaded (timestamp)`
--- `schema_field: id (generated primary), upload_id (foreign), field_name (text), field_max_length (numeric integer)`
--- `upload_row: id (generated primary), upload_id (foreign)`
--- `upload_field: id (generated primary), upload_row_id (foreign), schema_field_id (foreign), value (text)`. Optionally add (redundant) `upload_id` to simplify queries (if need be).

# Generate OpenAPI
-----


-----
Re-install Postgres
Postman Protobuf/JSON/REST request tester
Tests with Embedded Postgres
-----
study 20min
-----
lpalmieri.com
- #[tokio::main] async fn main() {...}

