#![deny(warnings)]
#![deny(clippy::all)]

use crate::server::utils::error::RestaurantError;
use mockall::automock;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Represents a menu item in the restaurant.
///
/// This struct models a single menu item, which includes:
/// - `id`: A unique identifier for the menu item.
/// - `name`: The name of the menu item.
/// - `cooking_time`: The time it takes to prepare the item in minutes.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, ToSchema)]
pub struct MenuItem {
    /// Unique identifier of the menu item.
    pub id: u32,
    /// Name of the menu item.
    pub name: String,
    /// The cooking time required for this menu item (in minutes).
    pub cooking_time: u64,
}

/// The `MenuStore` trait defines the behavior of a menu store.
///
/// This trait abstracts the functionality for accessing and managing
/// the restaurant's menu items. A struct implementing this trait can
/// retrieve all available menu items.
///
/// # Methods
/// - `get_all_menus`: Retrieves all menu items in the store.
#[automock]
pub trait MenuStore: Send + Sync {
    /// Retrieves all menu items from the store.
    ///
    /// # Returns
    /// A `Result` which is:
    /// - `Ok(Vec<MenuItem>)` with a list of all menu items if successful.
    /// - `Err(RestaurantError)` if there is a failure.
    fn get_all_menus(&self) -> Result<Vec<MenuItem>, RestaurantError>;
}

/// The `TableStore` trait defines the behavior of a table store.
///
/// This trait provides functionality to retrieve all available tables
/// in the restaurant.
///
/// # Methods
/// - `get_all_tables`: Retrieves all available table IDs.
#[automock]
pub trait TableStore: Send + Sync {
    /// Retrieves all table IDs in the store.
    ///
    /// # Returns
    /// A `Result` which is:
    /// - `Ok(Vec<u32>)` with a list of all table IDs if successful.
    /// - `Err(RestaurantError)` if there is a failure.
    fn get_all_tables(&self) -> Result<Vec<u32>, RestaurantError>;
}

/// The `OrderStore` trait defines the behavior of an order store.
///
/// This trait manages the orders placed for each table in the restaurant.
/// It allows adding, removing, and retrieving menu items associated
/// with a table.
///
/// # Methods
/// - `add_item`: Adds an item to the order for a specific table.
/// - `remove_item`: Removes an item from the order for a specific table.
/// - `get_item_ids`: Retrieves all item IDs for a specific table.
/// - `get_item_id`: Retrieves a specific item ID for a table.
#[automock]
pub trait OrderStore: Send + Sync {
    /// Adds a menu item to a table's order.
    ///
    /// # Parameters
    /// - `table_id`: The ID of the table placing the order.
    /// - `item_id`: The ID of the menu item being added to the order.
    ///
    /// # Returns
    /// A `Result` which is:
    /// - `Ok(())` if the item was successfully added.
    /// - `Err(RestaurantError)` if there was a failure.
    fn add_item(&self, table_id: u32, item_id: u32) -> Result<(), RestaurantError>;

    /// Removes a menu item from a table's order.
    ///
    /// # Parameters
    /// - `table_id`: The ID of the table from which the item is being removed.
    /// - `item_id`: The ID of the menu item being removed from the order.
    ///
    /// # Returns
    /// A `Result` which is:
    /// - `Ok(())` if the item was successfully removed.
    /// - `Err(RestaurantError)` if there was a failure.
    fn remove_item(&self, table_id: u32, item_id: u32) -> Result<(), RestaurantError>;

    /// Retrieves all item IDs ordered by a specific table.
    ///
    /// # Parameters
    /// - `table_id`: The ID of the table whose ordered item IDs are being retrieved.
    ///
    /// # Returns
    /// A `Result` which is:
    /// - `Ok(Vec<u32>)` with the list of item IDs ordered by the table.
    /// - `Err(RestaurantError)` if there was a failure.
    fn get_item_ids(&self, table_id: u32) -> Result<Vec<u32>, RestaurantError>;

    /// Retrieves the ID of a specific menu item ordered by a table.
    ///
    /// # Parameters
    /// - `table_id`: The ID of the table placing the order.
    /// - `item_id`: The ID of the menu item being retrieved.
    ///
    /// # Returns
    /// A `Result` which is:
    /// - `Ok(u32)` with the item ID if successful.
    /// - `Err(RestaurantError)` if there was a failure.
    fn get_item_id(&self, table_id: u32, item_id: u32) -> Result<u32, RestaurantError>;
}

/// The `Restaurant` trait combines `MenuStore`, `OrderStore`, and `TableStore`
/// into a single interface for managing a restaurant's operations.
///
/// This trait abstracts a complete restaurant's functionality, including managing
/// menu items, tables, and orders. It includes methods to get menu and table
/// details, as well as to manage and retrieve orders.
///
/// # Methods
/// - `get_all_menus`: Retrieves all menu items.
/// - `get_all_tables`: Retrieves all available tables.
/// - `add_item`: Adds a menu item to a table's order.
/// - `remove_item`: Removes a menu item from a table's order.
/// - `get_items`: Retrieves all menu items ordered at a table.
/// - `get_item`: Retrieves a specific menu item ordered at a table.
#[automock]
pub trait Restaurant: Send + Sync {
    /// Retrieves all menu items in the restaurant.
    ///
    /// # Returns
    /// A `Result` which is:
    /// - `Ok(Vec<MenuItem>)` with a list of all menu items.
    /// - `Err(RestaurantError)` if there is a failure.
    fn get_all_menus(&self) -> Result<Vec<MenuItem>, RestaurantError>;

    /// Retrieves all table IDs in the restaurant.
    ///
    /// # Returns
    /// A `Result` which is:
    /// - `Ok(Vec<u32>)` with a list of all table IDs.
    /// - `Err(RestaurantError)` if there is a failure.
    fn get_all_tables(&self) -> Result<Vec<u32>, RestaurantError>;

    /// Adds a menu item to a table's order.
    ///
    /// # Parameters
    /// - `table_id`: The ID of the table placing the order.
    /// - `item_id`: The ID of the menu item being added to the order.
    ///
    /// # Returns
    /// A `Result` which is:
    /// - `Ok(())` if the item was successfully added.
    /// - `Err(RestaurantError)` if there is a failure.
    fn add_item(&self, table_id: u32, item_id: u32) -> Result<(), RestaurantError>;

    /// Removes a menu item from a table's order.
    ///
    /// # Parameters
    /// - `table_id`: The ID of the table removing the item.
    /// - `item_id`: The ID of the menu item being removed.
    ///
    /// # Returns
    /// A `Result` which is:
    /// - `Ok(())` if the item was successfully removed.
    /// - `Err(RestaurantError)` if there is a failure.
    fn remove_item(&self, table_id: u32, item_id: u32) -> Result<(), RestaurantError>;

    /// Retrieves all menu items ordered by a specific table.
    ///
    /// # Parameters
    /// - `table_id`: The ID of the table whose ordered items are being retrieved.
    ///
    /// # Returns
    /// A `Result` which is:
    /// - `Ok(Vec<MenuItem>)` with the list of items ordered by the table.
    /// - `Err(RestaurantError)` if there is a failure.
    fn get_items(&self, table_id: u32) -> Result<Vec<MenuItem>, RestaurantError>;

    /// Retrieves a specific menu item ordered by a table.
    ///
    /// # Parameters
    /// - `table_id`: The ID of the table placing the order.
    /// - `item_id`: The ID of the menu item being retrieved.
    ///
    /// # Returns
    /// A `Result` which is:
    /// - `Ok(MenuItem)` with the requested menu item.
    /// - `Err(RestaurantError)` if there is a failure.
    fn get_item(&self, table_id: u32, item_id: u32) -> Result<MenuItem, RestaurantError>;
}
