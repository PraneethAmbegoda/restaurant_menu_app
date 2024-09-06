#![deny(warnings)]
#![deny(clippy::all)]

pub mod error;
pub mod models;

pub mod handlers;
pub mod in_memory_menu_store;
pub mod in_memory_order_store;
pub mod in_memory_table_store;
pub mod restaurant;
