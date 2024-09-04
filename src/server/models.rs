#![deny(warnings)]
#![deny(clippy::all)]

use crate::server::error::RestaurantError;
use mockall::automock;
use serde::{Deserialize, Serialize};

/// Represents a menu item in the restaurant.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MenuItem {
    pub id: u32,
    pub name: String,
    pub cooking_time: u64,
}

/// The `MenuStore` trait defines the behavior of a menu store,
/// which provides access to all available menu items.
#[automock]
pub trait MenuStore: Send + Sync {
    fn get_all_menus(&self) -> Result<Vec<MenuItem>, RestaurantError>;
}

/// The `TableStore` trait defines the behavior of a table store,
/// which provides access to all available tables.
#[automock]
pub trait TableStore: Send + Sync {
    fn get_all_tables(&self) -> Result<Vec<u32>, RestaurantError>;
}

/// The `OrderStore` trait defines the behavior of an order store,
/// which manages the orders for each table.
#[automock]
pub trait OrderStore: Send + Sync {
    fn add_item(&self, table_id: u32, item_id: u32) -> Result<(), RestaurantError>;
    fn remove_item(&self, table_id: u32, item_id: u32) -> Result<(), RestaurantError>;
    fn get_item_ids(&self, table_id: u32) -> Result<Vec<u32>, RestaurantError>;
    fn get_item_id(&self, table_id: u32, item_id: u32) -> Result<u32, RestaurantError>;
}

/// The `Restaurant` trait combines `MenuStore`, `OrderStore`, and `TableStore`
/// into a single interface for managing a restaurant's operations.
#[automock]
pub trait Restaurant: Send + Sync {
    fn get_all_menus(&self) -> Result<Vec<MenuItem>, RestaurantError>;
    fn get_all_tables(&self) -> Result<Vec<u32>, RestaurantError>;
    fn add_item(&self, table_id: u32, item_id: u32) -> Result<(), RestaurantError>;
    fn remove_item(&self, table_id: u32, item_id: u32) -> Result<(), RestaurantError>;
    fn get_items(&self, table_id: u32) -> Result<Vec<MenuItem>, RestaurantError>;
    fn get_item(&self, table_id: u32, item_id: u32) -> Result<MenuItem, RestaurantError>;
}
