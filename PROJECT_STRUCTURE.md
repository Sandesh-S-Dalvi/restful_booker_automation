# Project Structure

```
restful_booker_automation/
├── Cargo.toml                  # Project metadata and dependencies
├── Cargo.lock                  # Dependency lock file
├── README.md                   # Project documentation and execution instructions
├── PROJECT_STRUCTURE.md        # Architecture and directory layout guide
├── .gitignore                  # Git ignore rules
│
├── src/                        # Source code root
│   ├── main.rs                 # Binary entry point & CLI orchestration
│   ├── lib.rs                  # Library entry point (exposes modules for integration tests)
│   │
│   ├── api/                    # API Client Abstractions
│   │   ├── mod.rs              # API module declaration & re-exports
│   │   ├── client.rs           # Core HTTP Client setup & middleware
│   │   ├── auth.rs             # Auth endpoints (/auth)
│   │   └── booking.rs          # Booking endpoints (/booking)
│   │
│   ├── models/                 # Data Transfer Objects (DTOs) & Serde Models
│   │   ├── mod.rs              # Models module declaration
│   │   ├── auth.rs             # Auth request/response structs
│   │   └── booking.rs          # Booking request/response/patch structs
│   │
│   ├── builders/               # Test Data Builders & Fixtures
│   │   ├── mod.rs              # Builders module declaration
│   │   └── booking_builder.rs  # Fluent builder pattern for test data creation
│   │
│   ├── reporting/              # Test Execution & Report Generators
│   │   ├── mod.rs              # Reporting module declaration
│   │   ├── metrics.rs          # Test result & suite execution data structures
│   │   ├── json_reporter.rs    # Automated test_report.json exporter
│   │   └── html_reporter.rs    # Standalone test_report.html generator
│   │
│   └── utils/                  # Utility Functions & Configuration
│       ├── mod.rs              # Utilities module declaration
│       ├── config.rs           # Environment configuration & URL handlers
│       └── helpers.rs          # Miscellaneous helper utilities
│
├── tests/                      # Integration Test Suite (Cargo Test integration)
│   ├── common/                 # Shared test setup logic
│   │   └── mod.rs
│   ├── auth_tests.rs           # Authentication test scenarios
│   ├── crud_tests.rs           # E2E Booking CRUD integration tests
│   ├── security_tests.rs       # Authorization & security checks
│   └── negative_tests.rs       # Negative & boundary validation tests
│
└── target/                     # Build artifacts & test outputs (Generated at runtime)
    ├── test_report.json        # Output: Machine-readable JSON test execution report
    └── test_report.html        # Output: Interactive single-file HTML execution dashboard
```

## Design Principles

- **Separation of concerns**: API client, models, builders, and reporting are isolated.
- **Async-first**: All HTTP operations use Tokio + Reqwest.
- **Type safety**: Serde models with proper optional fields and date handling.
- **Testability**: Integration tests live in `tests/` and reuse the library crate.
- **Observability**: Structured logging via tracing.
- **Extensibility**: Client is middleware-ready; reporting is pluggable.
