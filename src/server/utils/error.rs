#![deny(warnings)]
#![deny(clippy::all)]

use std::fmt;

/// Enum representing errors that can occur in the Restaurant system.
///
/// This enum defines various types of errors that can arise while interacting
/// with the restaurant system, including issues with table and menu management,
/// lock errors, and data retrieval failures.
///
/// # Variants
/// - `LockError(String)`: Represents an error when a lock cannot be acquired.
/// - `TableNotFound(u32)`: Represents an error when a table with a given ID is not found.
/// - `MenuNotFound(u32)`: Represents an error when a menu item with a given ID is not found.
/// - `NoMenuForTable(u32, u32)`: Represents an error when a specific menu item for a table is not found.
/// - `NoMenusForTable(u32)`: Represents an error when no menu items are found for a given table.
/// - `MenusRetrieveError`: Represents an error that occurs when retrieving menus from the store.
/// - `TablesRetrieveError`: Represents an error that occurs when retrieving tables from the store.
#[derive(Debug, PartialEq)]
pub enum RestaurantError {
    /// Represents an error when a lock could not be acquired.
    ///
    /// The string provides additional information about the error.
    LockError(String),

    /// Represents an error when a table with the specified `table_id` is not found.
    ///
    /// - `table_id`: The ID of the table that was not found.
    TableNotFound(u32),

    /// Represents an error when a menu item with the specified `menu_id` is not found.
    ///
    /// - `menu_id`: The ID of the menu item that was not found.
    MenuNotFound(u32),

    /// Represents an error when a specific menu item is not found for a given table.
    ///
    /// - `table_id`: The ID of the table.
    /// - `menu_item_id`: The ID of the menu item that was not found.
    NoMenuForTable(u32, u32),

    /// Represents an error when no menu items are found for the specified table.
    ///
    /// - `table_id`: The ID of the table that has no menu items.
    NoMenusForTable(u32),

    /// Represents an error that occurs when trying to retrieve menus from the store.
    MenusRetrieveError,

    /// Represents an error that occurs when trying to retrieve tables from the store.
    TablesRetrieveError,
}

impl fmt::Display for RestaurantError {
    /// Formats the error message for each variant of `RestaurantError`.
    ///
    /// This method provides a human-readable representation of the error
    /// that can be used for logging or displaying error messages to the user.
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
                "No Menu item with menu item id:{}, is found for Table with table id:{}",
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
