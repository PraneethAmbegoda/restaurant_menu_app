#![deny(warnings)]
#![deny(clippy::all)]

use clap::Parser;
use rand::prelude::SliceRandom;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use reqwest::Client;
use restaurant_menu_app::server;
use serde_json::Value;
use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::task;
use tokio::time::sleep;

/// Command line argument parsing using `clap`
#[derive(Parser)]
struct Args {
    /// Port number of the server
    #[arg(short, long, default_value_t = 8081)]
    port: u16,
}

#[derive(serde::Deserialize, Debug)]
struct MenuItem {
    id: u32,
    name: String,
    cooking_time: u32,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Parse command-line arguments
    let args = Args::parse();

    // Start the server in a separate thread
    start_server_in_thread(args.port);

    // Wait for the server to start
    wait_for_server_start(args.port).await?;

    // Display the introduction message
    display_intro(args.port);

    // Create an HTTP client
    let client = Client::new();
    let base_url = format!("http://127.0.0.1:{}", args.port);

    // Enter the interactive loop
    interactive_loop(&client, &base_url).await;

    Ok(())
}

/// Starts the server in a separate thread
fn start_server_in_thread(port: u16) {
    thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            server::main::main(Some(port)).await.unwrap();
        });
    });
}

/// Displays the introduction and available options to the user
fn display_intro(port: u16) {
    println!("\n==================== Restaurant Management Client ====================\n");
    println!("Welcome to the Restaurant Management Client.");
    println!("This application allows you to interact with a virtual restaurant system,");
    println!("facilitating the management of tables, menus, and orders through a RESTful API.");
    println!("\nYou can perform the following operations:");
    println!("- Retrieve and view available menus");
    println!("- Get details of all active tables");
    println!("- Add or remove menu items to/from a table");
    println!("- Simulate complex operations with parallel requests\n");
    println!(
        "For API documentation, including the OpenAPI specification and Swagger UI, please visit:"
    );
    println!(
        "Swagger UI: http://127.0.0.1:{}/swagger-ui/\nOpenAPI JSON: http://127.0.0.1:{}/api-doc/openapi.json\n",
        port, port
    );
    println!("=======================================================================\n");
}

/// Interactive loop to handle client operations
async fn interactive_loop(client: &Client, base_url: &str) {
    loop {
        display_menu_options();

        let input = read_user_input().trim().to_string();

        match input.as_str() {
            "1" => get_menus(client, base_url).await,
            "2" => get_tables(client, base_url).await,
            "3" => add_menu_item(client, base_url).await,
            "4" => remove_menu_item(client, base_url).await,
            "5" => get_table_orders(client, base_url).await,
            "6" => get_specific_menu_item(client, base_url).await,
            "7" => run_simulation(client, base_url).await,
            "8" => {
                println!("Exiting the application. Goodbye!");
                break;
            }
            _ => println!("Invalid option. Please try again."),
        }
    }
}

/// Displays the available menu options
fn display_menu_options() {
    println!(
        "\nPlease select an operation:\n\
         1. Retrieve Available Menus \n\
         2. Get Active Tables \n\
         3. Add a Menu Item to a Table \n\
         4. Remove a Menu Item from a Table \n\
         5. Get All Orders for a Table\n\
         6. Get Specific Menu Item Ordered for a Table\n\
         7. Run Simulation (Parallel add/remove menu items for upto 100 tables)\n\
         8. Exit"
    );
}

/// Reads user input from the command line
fn read_user_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}

/// Retrieves and displays available menus
async fn get_menus(client: &Client, base_url: &str) {
    let url = format!("{}/api/v1/menus", base_url);
    let response = client.get(&url).send().await;

    match response {
        Ok(res) => println!("Menus: {:?}", res.text().await.unwrap()),
        Err(err) => println!("Error retrieving menus: {}", err),
    }
}

/// Retrieves and displays available tables
async fn get_tables(client: &Client, base_url: &str) {
    let url = format!("{}/api/v1/tables", base_url);
    let response = client.get(&url).send().await;

    match response {
        Ok(res) => println!("Tables: {:?}", res.text().await.unwrap()),
        Err(err) => println!("Error retrieving tables: {}", err),
    }
}

/// Adds a menu item to a table based on user input
async fn add_menu_item(client: &Client, base_url: &str) {
    let (table_id, menu_item_id) = get_table_and_menu_ids();

    let url = format!("{}/api/v1/add_item/{}/{}", base_url, table_id, menu_item_id);
    let response = client.post(&url).send().await;

    match response {
        Ok(res) => println!(
            "Menu item added successfully: {:?}",
            res.text().await.unwrap()
        ),
        Err(err) => println!("Error adding menu item: {}", err),
    }
}

/// Removes a menu item from a table based on user input
async fn remove_menu_item(client: &Client, base_url: &str) {
    let (table_id, menu_item_id) = get_table_and_menu_ids();

    let url = format!(
        "{}/api/v1/remove_item/{}/{}",
        base_url, table_id, menu_item_id
    );
    let response = client.delete(&url).send().await;

    match response {
        Ok(res) => println!(
            "Menu item removed successfully: {:?}",
            res.text().await.unwrap()
        ),
        Err(err) => println!("Error removing menu item: {}", err),
    }
}

/// Gets orders for a specific table
async fn get_table_orders(client: &Client, base_url: &str) {
    let table_id = get_table_id();
    let url = format!("{}/api/v1/get_items/{}", base_url, table_id);

    let response = client.get(&url).send().await;
    match response {
        Ok(res) => println!(
            "Orders for table {}: {:?}",
            table_id,
            res.text().await.unwrap()
        ),
        Err(err) => println!("Error retrieving orders: {}", err),
    }
}

/// Gets a specific menu item ordered for a table
async fn get_specific_menu_item(client: &Client, base_url: &str) {
    let (table_id, menu_item_id) = get_table_and_menu_ids();

    let url = format!("{}/api/v1/get_item/{}/{}", base_url, table_id, menu_item_id);
    let response = client.get(&url).send().await;

    match response {
        Ok(res) => println!(
            "Details of menu item {} for table {}: {:?}",
            menu_item_id,
            table_id,
            res.text().await.unwrap()
        ),
        Err(err) => println!("Error retrieving menu item details: {}", err),
    }
}

/// Runs the simulation: adding/removing menu items from tables in parallel
async fn run_simulation(client: &reqwest::Client, base_url: &str) {
    // Ask the user for the number of tables to simulate
    println!("Enter the number of tables for the simulation (max 100, default 10): ");
    let input = read_user_input().trim().to_string();

    // Parse input and ensure it's within the allowed range
    let num_tables: usize = input.trim().parse().unwrap_or(10);
    if num_tables > 100 {
        println!("Error: The maximum number of tables allowed for simulation is 100.");
        return;
    }

    println!("\n========== Starting Simulation ==========");
    println!(
        "1. Select Tables for Simulation: A random selection of {} tables is performed.",
        num_tables
    );
    println!("2. Simultaneous Add and Remove Operations: Menu items are added and removed in parallel, ensuring that only items that were added are removed.");
    println!("3. Retain Some Items After Simulation: Some items are randomly selected to remain on the table.");
    println!("4. Final Status Printing: The final status of each table is printed in parrellel.\n");
    println!("==========================================\n");

    let num_tables: usize = input.trim().parse().unwrap_or(10);
    let num_tables = num_tables.min(100); // Ensure max 100

    let tables_response = client
        .get(&format!("{}/api/v1/tables", base_url))
        .send()
        .await
        .unwrap();
    let json_response: Value =
        serde_json::from_str(&tables_response.text().await.unwrap()).unwrap();
    let table_ids: Vec<u32> = serde_json::from_value(json_response["data"].clone()).unwrap();

    let menus_response = client
        .get(&format!("{}/api/v1/menus", base_url))
        .send()
        .await
        .unwrap();

    let json_response: Value = serde_json::from_str(&menus_response.text().await.unwrap()).unwrap();
    let menus: Vec<MenuItem> = serde_json::from_value(json_response["data"].clone()).unwrap();
    let menu_ids: Vec<u32> = menus.iter().map(|menu| menu.id).collect();

    let mut rng = StdRng::from_entropy();
    // Select the user-defined number of random tables for simulation
    let selected_tables: Vec<u32> = table_ids
        .choose_multiple(&mut rng, num_tables)
        .cloned()
        .collect();
    let table_items: Arc<Mutex<HashMap<u32, Vec<u32>>>> = Arc::new(Mutex::new(HashMap::new()));

    // Add items to tables
    let mut add_handles = Vec::new();
    for &table_id in &selected_tables {
        let client_clone = client.clone();
        let base_url_clone = base_url.to_string();
        let table_items_clone = Arc::clone(&table_items);
        let menu_ids_clone = menu_ids.clone();

        let add_handle = task::spawn(async move {
            let mut rng = StdRng::from_entropy();
            let menu_items_to_add: Vec<u32> = menu_ids_clone
                .choose_multiple(&mut rng, 3)
                .cloned()
                .collect();

            {
                let mut table_items_lock = table_items_clone.lock().await;
                table_items_lock
                    .entry(table_id)
                    .or_default()
                    .extend(menu_items_to_add.clone());
            }

            for &menu_item_id in &menu_items_to_add {
                let url_add = format!(
                    "{}/api/v1/add_item/{}/{}",
                    base_url_clone, table_id, menu_item_id
                );
                println!("Ordering menu item {} for table {}", menu_item_id, table_id);
                let response = client_clone.post(&url_add).send().await;
                match response {
                    Ok(res) => {
                        if res.status().is_success() {
                            println!(
                                "Successfully ordered menu item {} for table {}",
                                menu_item_id, table_id
                            );
                        } else {
                            println!(
                                "Failed to order menu item {} for table {}: {}",
                                menu_item_id,
                                table_id,
                                res.status()
                            );
                        }
                    }
                    Err(err) => {
                        println!(
                            "Error occurred while ordering menu item {} for table {}: {}",
                            menu_item_id, table_id, err
                        );
                    }
                }
            }
        });
        add_handles.push(add_handle);
    }

    for handle in add_handles {
        handle.await.unwrap();
    }

    // Remove items from tables
    let mut remove_handles = Vec::new();
    for &table_id in &selected_tables {
        let client_clone = client.clone();
        let base_url_clone = base_url.to_string();
        let table_items_clone = Arc::clone(&table_items);

        let remove_handle = task::spawn(async move {
            let items_to_remove: Vec<u32>;
            {
                let table_items_lock = table_items_clone.lock().await;
                if let Some(items) = table_items_lock.get(&table_id) {
                    items_to_remove = items
                        .choose_multiple(
                            &mut StdRng::from_entropy(),
                            rand::thread_rng().gen_range(0..items.len()),
                        )
                        .cloned()
                        .collect();
                } else {
                    return;
                }
            }

            for &menu_item_id in &items_to_remove {
                let url_remove = format!(
                    "{}/api/v1/remove_item/{}/{}",
                    base_url_clone, table_id, menu_item_id
                );
                println!(
                    "Removing menu item {} from table {}",
                    menu_item_id, table_id
                );
                let response = client_clone.delete(&url_remove).send().await;

                match response {
                    Ok(res) => {
                        if res.status().is_success() {
                            println!(
                                "Successfully removed menu item {} from table {}",
                                menu_item_id, table_id
                            );
                        } else {
                            println!(
                                "Failed to remove menu item {} from table {}: {}",
                                menu_item_id,
                                table_id,
                                res.status()
                            );
                        }
                    }
                    Err(err) => {
                        println!(
                            "Error occurred while removing menu item {} from table {}: {}",
                            menu_item_id, table_id, err
                        );
                    }
                }
            }
        });
        remove_handles.push(remove_handle);
    }

    for handle in remove_handles {
        handle.await.unwrap();
    }

    println!("\n========== Final Table Status ==========");
    let mut status_handles = Vec::new();
    for &table_id in &selected_tables {
        let client_clone = client.clone();
        let base_url_clone = base_url.to_string();

        let handle = tokio::spawn(async move {
            let url_get_items = format!("{}/api/v1/get_items/{}", base_url_clone, table_id);
            let response = client_clone.get(&url_get_items).send().await.unwrap();
            let json_response: Value =
                serde_json::from_str(&response.text().await.unwrap()).unwrap();
            let menu_items: Vec<MenuItem> =
                serde_json::from_value(json_response["data"].clone()).unwrap();

            for item in menu_items {
                println!(
                    "For Table: {}  Menu Item ID: {}, Name: {}, Cooking Time: {} minutes",
                    table_id, item.id, item.name, item.cooking_time
                );
            }
        });

        status_handles.push(handle);
    }

    // Wait for all status checks to complete
    for handle in status_handles {
        handle.await.unwrap();
    }

    println!("=========================================\n");
    println!("Simulation complete.");
}

/// Gets the table and menu IDs from the user
fn get_table_and_menu_ids() -> (u32, u32) {
    println!("Enter table number(positive interger):");
    let table_id = read_user_input().trim().parse().unwrap();

    println!("Enter menu item number(positive interger):");
    let menu_item_id = read_user_input().trim().parse().unwrap();

    (table_id, menu_item_id)
}

/// Gets the table ID from the user
fn get_table_id() -> u32 {
    println!("Enter table number(positive interger):");
    read_user_input().trim().parse().unwrap()
}

/// Function to wait until the server is ready and accepting connections
async fn wait_for_server_start(port: u16) -> std::io::Result<()> {
    let addr = format!("127.0.0.1:{}", port);
    let mut retries = 10;

    while retries > 0 {
        if TcpListener::bind(&addr).is_ok() {
            // Server is ready
            return Ok(());
        }
        retries -= 1;
        sleep(Duration::from_secs(1)).await;
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::AddrNotAvailable,
        "Server failed to start",
    ))
}
