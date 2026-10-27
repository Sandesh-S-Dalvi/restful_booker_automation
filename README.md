# Restful Booker Automation

Production-grade Rust API automation framework for the [Restful Booker](https://restful-booker.herokuapp.com/) sample API.

## Features

- Typed async HTTP client with middleware-ready architecture
- Serde models matching OpenAPI schema
- Fluent test data builders
- Configurable environments
- JSON and interactive HTML test reporters
- Comprehensive integration tests (CRUD, auth, security, negative)
- CLI for health checks and manual runs
- Observability with tracing

## Prerequisites

- Rust 1.75+ (edition 2021)
- Cargo

## Setup

```bash
git clone https://github.com/Sandesh-S-Dalvi/restful_booker_automation.git
cd restful_booker_automation
cargo build
```

Default base URL: `https://restful-booker.herokuapp.com`

Override via environment:

```bash
export BOOKER_BASE_URL=https://restful-booker.herokuapp.com
export BOOKER_USERNAME=admin
export BOOKER_PASSWORD=password123
```

## Running Tests

```bash
# All integration tests
cargo test --test '*'

# Specific suite
cargo test --test crud_tests

# With output
cargo test -- --nocapture
```

Reports are written to `target/test_report.json` and `target/test_report.html` when the reporting helpers are invoked from tests or CLI.

## CLI

```bash
cargo run -- ping
cargo run -- create-booking
cargo run -- report
```

## Architecture

See [PROJECT_STRUCTURE.md](PROJECT_STRUCTURE.md) for the full layout and design decisions.

## License

MIT
