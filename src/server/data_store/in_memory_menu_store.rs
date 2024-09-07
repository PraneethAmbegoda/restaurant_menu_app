#![deny(warnings)]
#![deny(clippy::all)]

use crate::server::data_model::models::{MenuItem, MenuStore};
use crate::server::utils::error::RestaurantError;
use std::sync::{Arc, Mutex};

/// In-memory implementation of the `MenuStore` trait.
///
/// This store maintains a list of menu items that can be accessed concurrently.
/// The store is thread-safe, using a `Mutex` to protect access to the underlying data.
pub struct InMemoryMenuStore {
    menus: Arc<Mutex<Vec<MenuItem>>>,
}

impl InMemoryMenuStore {
    /// Creates a new instance of `InMemoryMenuStore` with the provided list of menu items.
    ///
    /// # Arguments
    ///
    /// * `menus` - A vector of `MenuItem` instances representing the initial menu.
    ///
    /// # Returns
    ///
    /// A new instance of `InMemoryMenuStore`.
    pub fn new(menus: Vec<MenuItem>) -> Self {
        InMemoryMenuStore {
            menus: Arc::new(Mutex::new(menus)),
        }
    }

    /// Creates a new instance of `InMemoryMenuStore` with a predefined list of menu items.
    ///
    /// # Returns
    ///
    /// A new instance of `InMemoryMenuStore` containing 20 predefined recipes with cooking times ranging from 1 to 15 minutes.
    pub fn with_predefined_recipes() -> Self {
        let predefined_menus = vec![
            MenuItem {
                id: 1,
                name: "Salad".to_string(),
                cooking_time: 1,
            },
            MenuItem {
                id: 2,
                name: "Soup".to_string(),
                cooking_time: 5,
            },
            MenuItem {
                id: 3,
                name: "Sandwich".to_string(),
                cooking_time: 7,
            },
            MenuItem {
                id: 4,
                name: "Pasta".to_string(),
                cooking_time: 12,
            },
            MenuItem {
                id: 5,
                name: "Steak".to_string(),
                cooking_time: 15,
            },
            MenuItem {
                id: 6,
                name: "Burger".to_string(),
                cooking_time: 10,
            },
            MenuItem {
                id: 7,
                name: "Pizza".to_string(),
                cooking_time: 14,
            },
            MenuItem {
                id: 8,
                name: "Tacos".to_string(),
                cooking_time: 8,
            },
            MenuItem {
                id: 9,
                name: "Fries".to_string(),
                cooking_time: 3,
            },
            MenuItem {
                id: 10,
                name: "Stir Fry".to_string(),
                cooking_time: 10,
            },
            MenuItem {
                id: 11,
                name: "Omelette".to_string(),
                cooking_time: 4,
            },
            MenuItem {
                id: 12,
                name: "Pancakes".to_string(),
                cooking_time: 6,
            },
            MenuItem {
                id: 13,
                name: "Sushi".to_string(),
                cooking_time: 12,
            },
            MenuItem {
                id: 14,
                name: "Curry".to_string(),
                cooking_time: 15,
            },
            MenuItem {
                id: 15,
                name: "Fish & Chips".to_string(),
                cooking_time: 13,
            },
            MenuItem {
                id: 16,
                name: "Fried Rice".to_string(),
                cooking_time: 9,
            },
            MenuItem {
                id: 17,
                name: "Ramen".to_string(),
                cooking_time: 14,
            },
            MenuItem {
                id: 18,
                name: "Burrito".to_string(),
                cooking_time: 8,
            },
            MenuItem {
                id: 19,
                name: "Waffles".to_string(),
                cooking_time: 5,
            },
            MenuItem {
                id: 20,
                name: "Salmon".to_string(),
                cooking_time: 13,
            },
        ];
        Self::new(predefined_menus)
    }
}

impl Default for InMemoryMenuStore {
    /// Provides a default implementation that initializes the store with predefined recipes.
    fn default() -> Self {
        Self::with_predefined_recipes()
    }
}

impl MenuStore for InMemoryMenuStore {
    /// Retrieves all menu items stored in the `InMemoryMenuStore`.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `MenuItem`s if successful, or a `RestaurantError` if an error occurs.
    fn get_all_menus(&self) -> Result<Vec<MenuItem>, RestaurantError> {
        let menus = self
            .menus
            .lock()
            .map_err(|_| RestaurantError::MenusRetrieveError)?;
        Ok(menus.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::data_model::models::MenuItem;

    #[test]
    fn test_get_all_menus_success() {
        let store = InMemoryMenuStore::with_predefined_recipes();
        let menus = store.get_all_menus().unwrap();

        assert_eq!(menus.len(), 20);
        assert!(menus.iter().any(|item| item.name == "Burger"));
        assert!(menus.iter().any(|item| item.name == "Sushi"));
    }

    #[test]
    fn test_get_all_menus_custom_items_success() {
        let custom_items = vec![
            MenuItem {
                id: 1,
                name: "Custom Item 1".to_string(),
                cooking_time: 5,
            },
            MenuItem {
                id: 2,
                name: "Custom Item 2".to_string(),
                cooking_time: 10,
            },
        ];
        let store = InMemoryMenuStore::new(custom_items.clone());
        let menus = store.get_all_menus().unwrap();

        assert_eq!(menus.len(), 2);
        assert_eq!(menus, custom_items);
    }

    #[test]
    fn test_get_all_menus_error() {
        // Create a store with an empty list.
        let store = InMemoryMenuStore {
            menus: Arc::new(Mutex::new(vec![])),
        };

        // Simulate a panic that causes the mutex to be poisoned.
        let result = std::panic::catch_unwind(|| {
            let _lock = store.menus.lock().unwrap();
            panic!("Simulating panic to poison mutex");
        });
        assert!(result.is_err()); // Ensure the panic occurred.

        // Try to get all menus, which should now result in a MenusRetrieveError due to the poisoned mutex.
        let result = store.get_all_menus();

        assert!(result.is_err());
        if let Err(RestaurantError::MenusRetrieveError) = result {
            // Test passes as we expect a MenusRetrieveError
        } else {
            panic!("Expected MenusRetrieveError");
        }
    }
}
