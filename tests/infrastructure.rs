//! Infrastructure layer tests
//!
//! Tests for infrastructure components organized by subdirectory.

// Config tests (moved from tests/config/)
#[path = "infrastructure/config/config_loading.rs"]
mod config_loading;

#[path = "infrastructure/config/config_logic.rs"]
mod config_logic;

#[path = "infrastructure/config/config_tests.rs"]
mod config_tests;

#[path = "infrastructure/config/config_unit.rs"]
mod config_unit;

// DI tests (moved from tests/shaku_compatibility_test.rs)
#[path = "infrastructure/di/shaku_test.rs"]
mod shaku_test;

// Metrics tests (moved from tests/metrics/)
#[path = "infrastructure/metrics/metrics.rs"]
mod metrics;

// Snapshot tests (moved from tests/snapshot/)
#[path = "infrastructure/snapshot/snapshot_tests.rs"]
mod snapshot_tests;

// Sync tests (moved from tests/sync/)
#[path = "infrastructure/sync/sync_manager.rs"]
mod sync_manager;
