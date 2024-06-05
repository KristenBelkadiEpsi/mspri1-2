use postgres::{ Client, NoTls };
use postgres::Error as PostgresError;
use std::net::{ TcpListener, TcpStream };
use std::io::{ Read, Write };
use std::env;

#[macro_use]
extern crate serde_derive;

//Model: Product struct with id, name, price, stock, description, date_added
#[derive(Serialize, Deserialize)]
struct Product {
    id: Option<i32>,
    name: String,
    price: f64,
    stock: i32,
    description: String,
    date_added: String,
}

//DATABASE_URL
const DB_URL: &str = env!("DATABASE_URL");

// constants for the HTTP response
const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND_RESPONSE: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_SERVER_ERROR_RESPONSE: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";

// main function
fn main() {
    // Set database
    if let Err(e) = set_database() {
        println!("Error setting up database: {:?}", e);
        return;
    }

    // start server and print port
    let listener = TcpListener::bind("0.0.0.0:8080".to_string()).unwrap();
    println!("server started at http://{}", listener.local_addr().unwrap());

    // handle the client
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}

//handle_client function
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_line, content) = match &*request {
                r if r.starts_with("POST /products") => handle_post_request(r),
                r if r.starts_with("GET /products/") => handle_get_request(r),
                r if r.starts_with("GET /products") => handle_get_all_request(r),
                r if r.starts_with("PUT /products/") => handle_put_request(r),
                r if r.starts_with("DELETE /products/") => handle_delete_request(r),
                _ => (NOT_FOUND_RESPONSE.to_string(), "404 Not Found".to_string()),
            };

            stream.write_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

//CONTROLLERS

// handle_post_request function
fn handle_post_request(request: &str) -> (String, String) {
    match (get_product_request_body(&request), Client::connect(DB_URL, NoTls)) {
        (Ok(product), Ok(mut client)) => {
            client
                .execute(
                    "INSERT INTO products (name, price, stock, description, date_added) VALUES ($1, $2, $3, $4, $5)",
                    &[&product.name, &product.price, &product.stock, &product.description, &product.date_added],
                )
                .unwrap();

            (OK_RESPONSE.to_string(), "Product created".to_string())
        }
        _ => (INTERNAL_SERVER_ERROR_RESPONSE.to_string(), "Error".to_string()),
    }
}

//handle_get_request function
fn handle_get_request(request: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>(), Client::connect(DB_URL, NoTls)) {
        (Ok(id), Ok(mut client)) =>
            match client.query_one("SELECT * FROM products WHERE id = $1", &[&id]) {
                Ok(row) => {
                    let product = Product {
                        id: row.get(0),
                        name: row.get(1),
                        price: row.get(2),
                        stock: row.get(3),
                        description: row.get(4),
                        date_added: row.get(5),
                    };
                    (OK_RESPONSE.to_string(), serde_json::to_string(&product).unwrap())
                }
                _ => (NOT_FOUND_RESPONSE.to_string(), "Product not found".to_string()),
            }

        _ => (INTERNAL_SERVER_ERROR_RESPONSE.to_string(), "Error".to_string()),
    }
}

//handle_get_all_request function
fn handle_get_all_request(request: &str) -> (String, String) {
    match Client::connect(DB_URL, NoTls) {
        Ok(mut client) => {
            let mut products = Vec::new();

            for row in client.query("SELECT * FROM products", &[]).unwrap() {
                products.push(Product {
                    id: row.get(0),
                    name: row.get(1),
                    price: row.get(2),
                    stock: row.get(3),
                    description: row.get(4),
                    date_added: row.get(5),
                });
            }

            (OK_RESPONSE.to_string(), serde_json::to_string(&products).unwrap())
        }
        _ => (INTERNAL_SERVER_ERROR_RESPONSE.to_string(), "Error".to_string()),
    }
}

//handle_put_request function
fn handle_put_request(request: &str) -> (String, String) {
    match
    (
        get_id(&request).parse::<i32>(),
        get_product_request_body(&request),
        Client::connect(DB_URL, NoTls),
    )
    {
        (Ok(id), Ok(product), Ok(mut client)) => {
            client
                .execute(
                    "UPDATE products SET name = $1, price = $2, stock = $3, description = $4, date_added = $5 WHERE id = $6",
                    &[&product.name, &product.price, &product.stock, &product.description, &product.date_added, &id],
                )
                .unwrap();

            (OK_RESPONSE.to_string(), "Product updated".to_string())
        }
        _ => (INTERNAL_SERVER_ERROR_RESPONSE.to_string(), "Error".to_string()),
    }
}

//handle_delete_request function
fn handle_delete_request(request: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>(), Client::connect(DB_URL, NoTls)) {
        (Ok(id), Ok(mut client)) => {
            let rows_affected = client.execute("DELETE FROM products WHERE id = $1", &[&id]).unwrap();

            if rows_affected == 0 {
                return (NOT_FOUND_RESPONSE.to_string(), "Product not found".to_string());
            }

            (OK_RESPONSE.to_string(), "Product deleted".to_string())
        }
        _ => (INTERNAL_SERVER_ERROR_RESPONSE.to_string(), "Error".to_string()),
    }
}

// set_database function
fn set_database() -> Result<(), PostgresError> {
    // Connect to the database
    let mut client = Client::connect(DB_URL, NoTls)?;

    // Create a table
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS products (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            price FLOAT NOT NULL,
            stock INT NOT NULL,
            description TEXT NOT NULL,
            date_added TEXT NOT NULL
        )
    ")?;

    Ok(())
}

// get_id function
fn get_id(request: &str) -> &str {
    request.split("/").nth(2).unwrap_or_default().split_whitespace()
        .next().unwrap_or_default()
}

// deserialize product from request body
fn get_product_request_body(request: &str) -> Result<Product, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}

