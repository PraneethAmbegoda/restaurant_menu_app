#![deny(warnings)]
#![deny(clippy::all)]

use crate::server::data_model::models::OrderStore;
use crate::server::utils::error::RestaurantError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// In-memory implementation of the `OrderStore` trait.
///
/// This store maintains orders for tables in the restaurant. Each order is represented
/// as a mapping from table IDs to a list of menu item IDs. The store is thread-safe, using
/// a `Mutex` to protect access to the underlying data.
pub struct InMemoryOrderStore {
    orders: Arc<Mutex<HashMap<u32, Vec<u32>>>>, // Stores table_id -> Vec<item_id>
}

impl InMemoryOrderStore {
    /// Creates a new instance of `InMemoryOrderStore`.
    ///
    /// # Returns
    ///
    /// A new instance of `InMemoryOrderStore` with an empty set of orders.
    pub fn new() -> Self {
        InMemoryOrderStore {
            orders: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryOrderStore {
    /// Provides a default implementation using the `new` method.
    fn default() -> Self {
        Self::new()
    }
}

impl OrderStore for InMemoryOrderStore {
    /// Adds a single item to the specified table's order by its menu item ID.
    ///
    /// # Arguments
    ///
    /// * `table_id` - The ID of the table to which the item should be added.
    /// * `item_id` - The ID of the menu item to add to the order.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the item was successfully added.
    /// * `Err(RestaurantError)` if there was an error accessing the order store.
    fn add_item(&self, table_id: u32, item_id: u32) -> Result<(), RestaurantError> {
        let mut orders = self
            .orders
            .lock()
            .map_err(|e| RestaurantError::LockError(e.to_string()))?;
        orders.entry(table_id).or_default().push(item_id);
        Ok(())
    }

    /// Removes a specific item from the specified table's order by its menu item ID.
    ///
    /// # Arguments
    ///
    /// * `table_id` - The ID of the table from which the item should be removed.
    /// * `item_id` - The ID of the menu item to remove.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the item was successfully removed.
    /// * `Err(RestaurantError)` if the table or item was not found, or if there was an error accessing the order store.
    fn remove_item(&self, table_id: u32, item_id: u32) -> Result<(), RestaurantError> {
        let mut orders = self
            .orders
            .lock()
            .map_err(|e| RestaurantError::LockError(e.to_string()))?;
        if let Some(items) = orders.get_mut(&table_id) {
            if let Some(pos) = items.iter().position(|&id| id == item_id) {
                items.remove(pos);
                Ok(())
            } else {
                Err(RestaurantError::NoMenuForTable(table_id, item_id))
            }
        } else {
            Err(RestaurantError::NoMenusForTable(table_id))
        }
    }

    /// Retrieves all item IDs from the specified table's order.
    ///
    /// # Arguments
    ///
    /// * `table_id` - The ID of the table whose items should be retrieved.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<u32>)` containing all item IDs if the table is found.
    /// * `Err(RestaurantError)` if the table is not found or if there was an error accessing the order store.
    fn get_item_ids(&self, table_id: u32) -> Result<Vec<u32>, RestaurantError> {
        let orders = self
            .orders
            .lock()
            .map_err(|e| RestaurantError::LockError(e.to_string()))?;
        orders
            .get(&table_id)
            .cloned()
            .ok_or(RestaurantError::NoMenusForTable(table_id))
    }

    /// Retrieves a specific item ID from the specified table's order.
    ///
    /// # Arguments
    ///
    /// * `table_id` - The ID of the table whose item should be retrieved.
    /// * `item_id` - The ID of the item to retrieve.
    ///
    /// # Returns
    ///
    /// * `Ok(u32)` if the item is found.
    /// * `Err(RestaurantError)` if the table or item is not found, or if there was an error accessing the order store.
    fn get_item_id(&self, table_id: u32, item_id: u32) -> Result<u32, RestaurantError> {
        let orders = self
            .orders
            .lock()
            .map_err(|e| RestaurantError::LockError(e.to_string()))?;
        orders
            .get(&table_id)
            .and_then(|items| {
                if items.contains(&item_id) {
                    Some(item_id)
                } else {
                    None
                }
            })
            .ok_or(RestaurantError::NoMenuForTable(table_id, item_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_item_success() {
        let store = InMemoryOrderStore::new();
        let table_id = 1;
        let item_id = 42;

        let result = store.add_item(table_id, item_id);
        assert!(result.is_ok());
        let item_ids = store.get_item_ids(table_id).unwrap();
        assert_eq!(item_ids.len(), 1);
        assert_eq!(item_ids[0], item_id);
    }

    #[test]
    fn test_remove_item_success() {
        let store = InMemoryOrderStore::new();
        let table_id = 1;
        let item_id = 42;

        store.add_item(table_id, item_id).unwrap();
        let result = store.remove_item(table_id, item_id);
        assert!(result.is_ok());
        let item_ids = store.get_item_ids(table_id).unwrap();
        assert!(item_ids.is_empty());
    }

    #[test]
    fn test_remove_item_not_found() {
        let store = InMemoryOrderStore::new();
        let table_id = 1;
        let item_id = 42;

        store.add_item(table_id, item_id).unwrap();
        let result = store.remove_item(table_id, 99);
        assert!(matches!(
            result,
            Err(RestaurantError::NoMenuForTable(1, 99))
        ));
    }

    #[test]
    fn test_remove_item_table_not_found() {
        let store = InMemoryOrderStore::new();
        let result = store.remove_item(99, 1);
        assert!(matches!(result, Err(RestaurantError::NoMenusForTable(99))));
    }

    #[test]
    fn test_get_item_ids_success() {
        let store = InMemoryOrderStore::new();
        let table_id = 1;
        let item_id1 = 42;
        let item_id2 = 43;

        store.add_item(table_id, item_id1).unwrap();
        store.add_item(table_id, item_id2).unwrap();
        let item_ids = store.get_item_ids(table_id).unwrap();
        assert_eq!(item_ids.len(), 2);
        assert_eq!(item_ids, vec![item_id1, item_id2]);
    }

    #[test]
    fn test_get_item_ids_table_not_found() {
        let store = InMemoryOrderStore::new();
        let result = store.get_item_ids(99);
        assert!(matches!(result, Err(RestaurantError::NoMenusForTable(99))));
    }

    #[test]
    fn test_get_item_id_success() {
        let store = InMemoryOrderStore::new();
        let table_id = 1;
        let item_id = 42;

        store.add_item(table_id, item_id).unwrap();
        let retrieved_item_id = store.get_item_id(table_id, item_id).unwrap();
        assert_eq!(retrieved_item_id, item_id);
    }

    #[test]
    fn test_get_item_id_not_found() {
        let store = InMemoryOrderStore::new();
        let table_id = 1;

        let result = store.get_item_id(table_id, 99);
        assert!(matches!(
            result,
            Err(RestaurantError::NoMenuForTable(1, 99))
        ));
    }
}
