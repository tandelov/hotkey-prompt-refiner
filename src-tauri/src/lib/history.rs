use crate::models::HistoryEntry;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

/// Maximum number of history entries to keep in memory
const MAX_HISTORY_SIZE: usize = 100;

/// In-memory history storage
pub struct HistoryStore {
    entries: Arc<Mutex<VecDeque<HistoryEntry>>>,
}

impl HistoryStore {
    /// Create a new history store
    pub fn new() -> Self {
        HistoryStore {
            entries: Arc::new(Mutex::new(VecDeque::with_capacity(MAX_HISTORY_SIZE))),
        }
    }

    /// Add a new history entry
    pub fn add_entry(&self, entry: HistoryEntry) {
        let mut entries = self.entries.lock().unwrap();

        // Add to front (most recent first)
        entries.push_front(entry);

        // Trim if exceeds max size
        if entries.len() > MAX_HISTORY_SIZE {
            entries.pop_back();
        }
    }

    /// Get all history entries
    pub fn get_all(&self) -> Vec<HistoryEntry> {
        let entries = self.entries.lock().unwrap();
        entries.iter().cloned().collect()
    }

    /// Get history entries with pagination
    pub fn get_page(&self, offset: usize, limit: usize) -> Vec<HistoryEntry> {
        let entries = self.entries.lock().unwrap();
        entries
            .iter()
            .skip(offset)
            .take(limit)
            .cloned()
            .collect()
    }

    /// Clear all history
    pub fn clear(&self) {
        let mut entries = self.entries.lock().unwrap();
        entries.clear();
    }

    /// Get total count of history entries
    pub fn count(&self) -> usize {
        let entries = self.entries.lock().unwrap();
        entries.len()
    }

    /// Search history by template name or content
    pub fn search(&self, query: &str) -> Vec<HistoryEntry> {
        let entries = self.entries.lock().unwrap();
        let query_lower = query.to_lowercase();

        entries
            .iter()
            .filter(|entry| {
                entry.template_name.to_lowercase().contains(&query_lower)
                    || entry.source_preview.to_lowercase().contains(&query_lower)
                    || entry.result_preview.to_lowercase().contains(&query_lower)
            })
            .cloned()
            .collect()
    }

    /// Get a clone of the Arc for sharing across threads
    pub fn clone_arc(&self) -> Arc<Mutex<VecDeque<HistoryEntry>>> {
        Arc::clone(&self.entries)
    }
}

impl Default for HistoryStore {
    fn default() -> Self {
        Self::new()
    }
}
