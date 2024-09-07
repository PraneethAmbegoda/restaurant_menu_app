#![deny(warnings)]
#![deny(clippy::all)]

use crate::server::data_model::models::TableStore;
use crate::server::utils::error::RestaurantError;
use std::sync::{Arc, Mutex};

/// In-memory implementation of the `TableStore` trait.
///
/// This store maintains a list of tables that can be accessed concurrently.
/// The store is thread-safe, using a `Mutex` to protect access to the underlying data.
pub struct InMemoryTableStore {
    tables: Arc<Mutex<Vec<u32>>>, // Stores a list of table IDs
}

impl InMemoryTableStore {
    /// Creates a new instance of `InMemoryTableStore`.
    ///
    /// # Returns
    ///
    /// A new instance of `InMemoryTableStore` with 100 predefined table IDs.
    pub fn new() -> Self {
        let predefined_tables = (1..=100).collect();
        InMemoryTableStore {
            tables: Arc::new(Mutex::new(predefined_tables)),
        }
    }
}

impl Default for InMemoryTableStore {
    /// Provides a default implementation using the `new` method.
    fn default() -> Self {
        Self::new()
    }
}

impl TableStore for InMemoryTableStore {
    /// Retrieves all table IDs stored in the `InMemoryTableStore`.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of table IDs if successful, or a `RestaurantError` if an error occurs.
    fn get_all_tables(&self) -> Result<Vec<u32>, RestaurantError> {
        let tables = self
            .tables
            .lock()
            .map_err(|_| RestaurantError::TablesRetrieveError)?;
        Ok(tables.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_tables_success() {
        let store = InMemoryTableStore::new();
        let tables = store.get_all_tables().unwrap();

        assert_eq!(tables.len(), 100);
        assert_eq!(tables[0], 1);
        assert_eq!(tables[99], 100);
    }

    #[test]
    fn test_get_all_tables_error() {
        let store = InMemoryTableStore {
            tables: Arc::new(Mutex::new(vec![])),
        };

        // Simulate a panic that causes the mutex to be poisoned.
        let result = std::panic::catch_unwind(|| {
            let _lock = store.tables.lock().unwrap();
            panic!("Simulating panic to poison mutex");
        });
        assert!(result.is_err()); // Ensure the panic occurred.

        // Try to get all tables, which should now result in a TablesRetrieveError due to the poisoned mutex.
        let result = store.get_all_tables();

        assert!(result.is_err());
        if let Err(RestaurantError::TablesRetrieveError) = result {
            // Test passes as we expect a TablesRetrieveError
        } else {
            panic!("Expected TablesRetrieveError");
        }
    }
}
