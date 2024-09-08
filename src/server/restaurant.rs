#![deny(warnings)]
#![deny(clippy::all)]

use crate::server::data_model::models::{MenuItem, MenuStore, OrderStore, Restaurant, TableStore};
use crate::server::utils::error::RestaurantError;

/// `SimpleRestaurant` is an implementation of the `Restaurant` trait.
/// It interacts with `MenuStore`, `OrderStore`, and `TableStore` to manage
/// restaurant operations such as adding/removing menu items, retrieving
/// available tables, and fetching order information.
pub struct SimpleRestaurant {
    pub menu_store: Box<dyn MenuStore>,
    pub order_store: Box<dyn OrderStore>,
    pub table_store: Box<dyn TableStore>,
}

impl SimpleRestaurant {
    /// Creates a new instance of `SimpleRestaurant`.
    ///
    /// # Arguments
    ///
    /// * `menu_store` - A boxed implementation of `MenuStore`.
    /// * `order_store` - A boxed implementation of `OrderStore`.
    /// * `table_store` - A boxed implementation of `TableStore`.
    ///
    /// # Returns
    ///
    /// A new instance of `SimpleRestaurant`.
    pub fn new(
        menu_store: Box<dyn MenuStore>,
        order_store: Box<dyn OrderStore>,
        table_store: Box<dyn TableStore>,
    ) -> Self {
        SimpleRestaurant {
            menu_store,
            order_store,
            table_store,
        }
    }
}

impl Restaurant for SimpleRestaurant {
    /// Retrieves all available menus in the restaurant.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `MenuItem` if successful,
    /// or `RestaurantError` in case of failure.
    fn get_all_menus(&self) -> Result<Vec<MenuItem>, RestaurantError> {
        self.menu_store.get_all_menus()
    }

    /// Retrieves all available tables in the restaurant.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of table IDs if successful,
    /// or `RestaurantError` in case of failure.
    fn get_all_tables(&self) -> Result<Vec<u32>, RestaurantError> {
        self.table_store.get_all_tables()
    }

    /// Adds an item to a table's order. Checks if the table exists before adding.
    ///
    /// # Arguments
    ///
    /// * `table_id` - ID of the table.
    /// * `item_id` - ID of the menu item to be added.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the item is successfully added.
    /// * `Err(RestaurantError)` if the table or menu item is not found.
    fn add_item(&self, table_id: u32, item_id: u32) -> Result<(), RestaurantError> {
        let tables = self.get_all_tables()?;
        if !tables.contains(&table_id) {
            return Err(RestaurantError::TableNotFound(table_id));
        }

        let all_menus = self.get_all_menus()?;
        if !all_menus.iter().any(|item| item.id == item_id) {
            return Err(RestaurantError::MenuNotFound(item_id));
        }

        self.order_store.add_item(table_id, item_id)
    }

    /// Removes an item from a table's order. Checks if the table exists before removing.
    ///
    /// # Arguments
    ///
    /// * `table_id` - ID of the table.
    /// * `item_id` - ID of the menu item to be removed.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the item is successfully removed.
    /// * `Err(RestaurantError)` if the table or item is not found.
    fn remove_item(&self, table_id: u32, item_id: u32) -> Result<(), RestaurantError> {
        let tables = self.get_all_tables()?;
        if !tables.contains(&table_id) {
            return Err(RestaurantError::TableNotFound(table_id));
        }

        self.order_store.remove_item(table_id, item_id)
    }

    /// Retrieves all items ordered at a specific table.
    ///
    /// # Arguments
    ///
    /// * `table_id` - ID of the table.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `MenuItem` if successful,
    /// or `RestaurantError` in case of failure.
    fn get_items(&self, table_id: u32) -> Result<Vec<MenuItem>, RestaurantError> {
        let tables = self.get_all_tables()?;
        if !tables.contains(&table_id) {
            return Err(RestaurantError::TableNotFound(table_id));
        }

        let item_ids = self.order_store.get_item_ids(table_id)?;
        let all_menus = self.get_all_menus()?;

        let items = item_ids
            .into_iter()
            .filter_map(|id| all_menus.iter().find(|&item| item.id == id).cloned())
            .collect();

        Ok(items)
    }

    /// Retrieves a specific item ordered at a specific table.
    ///
    /// # Arguments
    ///
    /// * `table_id` - ID of the table.
    /// * `item_id` - ID of the item to retrieve.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `MenuItem` if successful,
    /// or `RestaurantError` in case of failure.
    fn get_item(&self, table_id: u32, item_id: u32) -> Result<MenuItem, RestaurantError> {
        let tables = self.get_all_tables()?;
        if !tables.contains(&table_id) {
            return Err(RestaurantError::TableNotFound(table_id));
        }

        self.order_store.get_item_id(table_id, item_id)?;
        let all_menus = self.get_all_menus()?;
        all_menus
            .into_iter()
            .find(|item| item.id == item_id)
            .ok_or(RestaurantError::MenuNotFound(item_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::data_model::models::{MockMenuStore, MockOrderStore, MockTableStore};
    use mockall::predicate::*;

    #[test]
    fn test_add_item_success() {
        let mut mock_menu_store = MockMenuStore::new();
        let mut mock_order_store = MockOrderStore::new();
        let mut mock_table_store = MockTableStore::new();

        let table_id = 1;
        let item_id = 1;

        mock_table_store
            .expect_get_all_tables()
            .returning(move || Ok(vec![table_id]));

        mock_menu_store.expect_get_all_menus().returning(move || {
            Ok(vec![MenuItem {
                id: item_id,
                name: "Burger".to_string(),
                cooking_time_minutes: 10,
            }])
        });

        mock_order_store
            .expect_add_item()
            .with(eq(table_id), eq(item_id))
            .returning(|_, _| Ok(()));

        let restaurant = SimpleRestaurant::new(
            Box::new(mock_menu_store),
            Box::new(mock_order_store),
            Box::new(mock_table_store),
        );

        let result = restaurant.add_item(table_id, item_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_add_item_failure() {
        let mut mock_menu_store = MockMenuStore::new();
        let mut mock_table_store = MockTableStore::new();

        let table_id = 1;
        let item_id = 1;

        mock_table_store
            .expect_get_all_tables()
            .returning(move || Ok(vec![table_id]));

        mock_menu_store
            .expect_get_all_menus()
            .returning(move || Ok(vec![])); // Menu item not found

        let restaurant = SimpleRestaurant::new(
            Box::new(mock_menu_store),
            Box::new(MockOrderStore::new()),
            Box::new(mock_table_store),
        );

        let result = restaurant.add_item(table_id, item_id);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), RestaurantError::MenuNotFound(item_id));
    }

    #[test]
    fn test_remove_item_success() {
        let mut mock_order_store = MockOrderStore::new();
        let mut mock_table_store = MockTableStore::new();

        let table_id = 1;
        let item_id = 1;

        mock_table_store
            .expect_get_all_tables()
            .returning(move || Ok(vec![table_id]));

        mock_order_store
            .expect_remove_item()
            .with(eq(table_id), eq(item_id))
            .returning(move |_, _| Ok(()));

        let restaurant = SimpleRestaurant::new(
            Box::new(MockMenuStore::new()),
            Box::new(mock_order_store),
            Box::new(mock_table_store),
        );

        let result = restaurant.remove_item(table_id, item_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_remove_item_failure() {
        let mut mock_order_store = MockOrderStore::new();
        let mut mock_table_store = MockTableStore::new();

        let table_id = 1;
        let item_id = 1;

        mock_table_store
            .expect_get_all_tables()
            .returning(move || Ok(vec![table_id]));

        mock_order_store
            .expect_remove_item()
            .with(eq(table_id), eq(item_id))
            .returning(move |_, _| Err(RestaurantError::MenuNotFound(item_id)));

        let restaurant = SimpleRestaurant::new(
            Box::new(MockMenuStore::new()),
            Box::new(mock_order_store),
            Box::new(mock_table_store),
        );

        let result = restaurant.remove_item(table_id, item_id);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), RestaurantError::MenuNotFound(item_id));
    }

    #[test]
    fn test_get_items_success() {
        let mut mock_menu_store = MockMenuStore::new();
        let mut mock_order_store = MockOrderStore::new();
        let mut mock_table_store = MockTableStore::new();

        let table_id = 1;
        let item_id = 1;
        let menu_item = MenuItem {
            id: item_id,
            name: "Burger".to_string(),
            cooking_time_minutes: 10,
        };

        mock_table_store
            .expect_get_all_tables()
            .returning(move || Ok(vec![table_id]));

        mock_order_store
            .expect_get_item_ids()
            .with(eq(table_id))
            .returning(move |_| Ok(vec![item_id]));

        mock_menu_store
            .expect_get_all_menus()
            .returning(move || Ok(vec![menu_item.clone()]));

        let restaurant = SimpleRestaurant::new(
            Box::new(mock_menu_store),
            Box::new(mock_order_store),
            Box::new(mock_table_store),
        );

        let result = restaurant.get_items(table_id).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "Burger");
    }

    #[test]
    fn test_get_items_failure() {
        let mut mock_order_store = MockOrderStore::new();
        let mut mock_table_store = MockTableStore::new();

        let table_id = 1;

        mock_table_store
            .expect_get_all_tables()
            .returning(move || Ok(vec![table_id]));

        mock_order_store
            .expect_get_item_ids()
            .with(eq(table_id))
            .returning(move |_| Err(RestaurantError::TableNotFound(table_id)));

        let restaurant = SimpleRestaurant::new(
            Box::new(MockMenuStore::new()),
            Box::new(mock_order_store),
            Box::new(mock_table_store),
        );

        let result = restaurant.get_items(table_id);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            RestaurantError::TableNotFound(table_id)
        );
    }

    #[test]
    fn test_get_item_success() {
        let mut mock_menu_store = MockMenuStore::new();
        let mut mock_order_store = MockOrderStore::new();
        let mut mock_table_store = MockTableStore::new();

        let table_id = 1;
        let item_id = 1;
        let menu_item = MenuItem {
            id: item_id,
            name: "Burger".to_string(),
            cooking_time_minutes: 10,
        };

        mock_table_store
            .expect_get_all_tables()
            .returning(move || Ok(vec![table_id]));

        mock_order_store
            .expect_get_item_id()
            .with(eq(table_id), eq(item_id))
            .returning(move |_, _| Ok(item_id));

        mock_menu_store
            .expect_get_all_menus()
            .returning(move || Ok(vec![menu_item.clone()]));

        let restaurant = SimpleRestaurant::new(
            Box::new(mock_menu_store),
            Box::new(mock_order_store),
            Box::new(mock_table_store),
        );

        let result = restaurant.get_item(table_id, item_id).unwrap();
        assert_eq!(result.name, "Burger");
    }

    #[test]
    fn test_get_item_failure() {
        let mut mock_order_store = MockOrderStore::new();
        let mut mock_table_store = MockTableStore::new();

        let table_id = 1;
        let item_id = 1;

        mock_table_store
            .expect_get_all_tables()
            .returning(move || Ok(vec![table_id]));

        mock_order_store
            .expect_get_item_id()
            .with(eq(table_id), eq(item_id))
            .returning(move |_, _| Err(RestaurantError::MenuNotFound(item_id)));

        let restaurant = SimpleRestaurant::new(
            Box::new(MockMenuStore::new()),
            Box::new(mock_order_store),
            Box::new(mock_table_store),
        );

        let result = restaurant.get_item(table_id, item_id);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), RestaurantError::MenuNotFound(item_id));
    }
}
