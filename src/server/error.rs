#![deny(warnings)]
#![deny(clippy::all)]

use std::fmt;

/// Enum representing errors that can occur in the Restaurant system.
#[derive(Debug, PartialEq)]
pub enum RestaurantError {
    LockError(String),
    TableNotFound(u32),
    MenuNotFound(u32),
    NoMenuForTable(u32, u32),
    NoMenusForTable(u32),
    MenusRetrieveError,
    TablesRetrieveError,
}

impl fmt::Display for RestaurantError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RestaurantError::LockError(err) => write!(f, "Lock error: {}", err),
            RestaurantError::MenuNotFound(item_id) => {
                write!(f, "Menu item with menu id:{} not found", item_id)
            }
            RestaurantError::MenusRetrieveError => write!(f, "Error when retrieving Menus"),
            RestaurantError::TableNotFound(table_id) => {
                write!(f, "Table with table id:{} not found", table_id)
            }
            RestaurantError::TablesRetrieveError => write!(f, "Error when retrieving Tables"),
            RestaurantError::NoMenuForTable(table_id, menu_item_id) => write!(
                f,
                "No Menu item with menu item id:{}, is fount for Table with table id:{}",
                menu_item_id, table_id
            ),
            RestaurantError::NoMenusForTable(table_id) => write!(
                f,
                "No Menu items added for table with table id:{}",
                table_id
            ),
        }
    }
}

impl std::error::Error for RestaurantError {}
