//! Snapshot Comparator Service - Compares snapshots to detect changes
//!
//! Single Responsibility: Compare two snapshots and identify differences.

use super::{CodebaseSnapshot, SnapshotChanges};

/// Service for comparing snapshots
pub struct SnapshotComparator;

impl Default for SnapshotComparator {
    fn default() -> Self {
        Self::new()
    }
}

impl SnapshotComparator {
    /// Create a new comparator
    pub fn new() -> Self {
        Self
    }

    /// Compare two snapshots and return changes
    pub fn compare(
        &self,
        old_snapshot: &CodebaseSnapshot,
        new_snapshot: &CodebaseSnapshot,
    ) -> SnapshotChanges {
        let mut added = Vec::new();
        let mut modified = Vec::new();
        let mut removed = Vec::new();
        let mut unchanged = Vec::new();

        // Check each file in new snapshot
        for (path, new_file) in &new_snapshot.files {
            match old_snapshot.files.get(path) {
                Some(old_file) => {
                    // File exists in both - check if modified
                    if old_file.hash != new_file.hash {
                        modified.push(path.clone());
                    } else {
                        unchanged.push(path.clone());
                    }
                }
                None => {
                    // File is new
                    added.push(path.clone());
                }
            }
        }

        // Find removed files (in old but not in new)
        for path in old_snapshot.files.keys() {
            if !new_snapshot.files.contains_key(path) {
                removed.push(path.clone());
            }
        }

        SnapshotChanges {
            added,
            modified,
            removed,
            unchanged,
        }
    }

    /// Check if there are any changes
    pub fn has_changes(&self, changes: &SnapshotChanges) -> bool {
        !changes.added.is_empty() || !changes.modified.is_empty() || !changes.removed.is_empty()
    }

    /// Get total number of changes
    pub fn change_count(&self, changes: &SnapshotChanges) -> usize {
        changes.added.len() + changes.modified.len() + changes.removed.len()
    }
}
