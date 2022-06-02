# Requirements
- Receive a CSV file as a POST body (but not as a multipart upload through a form). Parse, return as an array of JSON objects.
- Create tests.

# Optional requirements
Position-independent CSV headers. A file is accepted regardless of the order of columns, as far as their names match.

Store in Postgres. Discuss schema designs.

# Preferred technologies
Tokio, Axum

# Usage and Scope
## Start the server
- Register at https://www.mezmo.com/sign-up (community tier is fine).
- Visit https://app.logdna.com > Settings > "API Keys" and generate an "ingestion key."
- Put the ingestion key to an environment variable:
  - `export API_KEY=...`.
- `cargo run`

## Submit a request
- `wget --post-file=tests/assets/addresses.csv http://127.0.0.1:8080/addresses`
 or
- `curl -H "Content-Type: text/csv" --data-binary @tests/assets/addresses.csv 127.0.0.1:8080/addresses`
  - Use `--data-binary` instead of `--data`, otherwise newlines are stripped - and those are a part of CSV format.

Only a very simple CSV is accepted: Hheader field names are case sensitive, no special handling of quotes, no escaping.

# Debugging
`curl -w "%{http_code}" -H "Content-Type: text/json" --data-binary @tests/assets/addresses.csv 127.0.0.1:8080/addresses`

# Roadmap
- [x] `Tokio` webservice
- [x] parse CSV with https://docs.rs/csv/latest/csv
- [x] create own CSV parser
- [x] gemerate JSON with `Serde` (unclear how to generate it with https://docs.rs/tokio-serde-json/latest/tokio_serde_json, so skipping `tokio-serde-json`)
- [x] test manually with `curl` and `wget`
- [ ] reproducible with a local Postgres: https://github.com/faokunega/pg-embed (but unmaintained for 10 months! - however, it depends on https://github.com/zonkyio/embedded-postgres-binaries, which has 4 contributors and last commit 11 days ago)
- [ ] write to `Postgres`
- [ ] test with `Docker` image that contains `Postgres`
- [ ] investigate https://docs.rs/axum-test-helper/0.1.0/axum_test_helper (updated 4 weeks ago, 1 contributor)
- [ ] investigate https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs (more bolierplate)
- [ ] local & CI to be reproducible
- [ ] script with `cargo-run-bin` with `Makefile.toml` to make build & CI process independent of locally installed cargo packages

# Tradeoffs and Decisions
- Postgres with https://docs.rs/tokio-postgres/latest/tokio_postgres - chosen because it's a part of Tokio project => reliable.
  - Otherwise, based on https://project-awesome.org/rust-unofficial/awesome-rust:
    - https://github.com/launchbadge/sqlx:  it allows type-safe queries, and several SQL back-ends
    - Postgres only: https://github.com/sfackler/rust-postgres
- OpenAPI generation. It seems useless for CSV. But if we had parameters in an HTTP query/form:
  - https://github.com/softprops/openapi - too low-level for this task
  - https://github.com/glademiller/openapiv3 (last commit 4 months ago, 18 contributors) - but not Axum-specific, and the tests don't involve Axum either!
  - Based on https://github.com/tokio-rs/axum/issues/50, I've considered
    - https://github.com/jsdw/seamless (last commit 5 months ago, tested with Tokio 1.1.0 while the current is 1.18.2!),
    - https://github.com/oxidecomputer/dropshot (last commit a few days ago, using more current Tokio 1.16; it seems heavyweight),
    - https://github.com/jakobhellermann/axum_openapi (10 months old, one maintainer only),
    - https://github.com/juhaku/utoipa (last commit 6 days ago, 4 contributors) --> seems the **most practical** from the above, but not suitable (at least no obviously so) for CSV parsing (despite its example for Axum: https://github.com/juhaku/utoipa/blob/master/examples/todo-axum/src/main.rs).
- Write to Postgres
  - Do we have a defined schema for each client/company/data source, shared across their uploads? If yes:
    - 1. Schema can change over time, so each CSV column info would have to have its applicability period (two timestamps, and/or a client/company-specific version number/string). Or
    - 2. Everytime a client/company changes their schema, we create a new endpoint (and a new DB table).
  - 3. Alternatively, we have a dynamic schema generated from CSV, independent, per-upload.
  - Out of scope: merging with existing data (which would involve flagging conflicting/unmergable entries and human intervention). Hence:
  - Each upload creates a new subset of entries, all associated with the same "upload" entry, and we return a new ID of that upload.
  - This MVP has only one endpoint, hence #3 from above. Treating all values as texts. Four tables (mutli-dimensional flattened). Descriptions are Postgres-agnostic:
    - `uploads: id (generated primary), uploaded (timestamp)`
    - `schema_field: id (generated primary), upload_id (foreign), field_name (text), field_max_length (numeric integer)`
    - `upload_row: id (generated primary), upload_id (foreign)`
    - `upload_field: id (generated primary), upload_row_id (foreign), schema_field_id (foreign), value (text)`. Optionally add (redundant) `upload_id` to simplify queries (if need be).
