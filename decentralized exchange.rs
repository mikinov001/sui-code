// Import necessary libraries
use std::collections::HashMap;
use std::io;

// Define the Order struct
struct Order {
    trader: String,
    amount: u64,
    price: u64,
}

// Define the Exchange struct
struct Exchange {
    orders: HashMap<String, Order>,
}

// Implement the Exchange struct
impl Exchange {
    // Create a new instance of the Exchange struct
    fn new() -> Self {
        Self {
            orders: HashMap::new(),
        }
    }

    // Add an order to the exchange
    fn add_order(&mut self, order: Order) {
        self.orders.insert(order.trader.clone(), order);
    }

    // Remove an order from the exchange
    fn remove_order(&mut self, trader: &str) {
        self.orders.remove(trader);
    }

    // Get an order from the exchange
    fn get_order(&self, trader: &str) -> Option<&Order> {
        self.orders.get(trader)
    }

    // Match orders on the exchange
    fn match_orders(&mut self) {
        println!("Matching orders...");
        // Implement order matching logic here
    }
}

// Define the main function
fn main() {
    println!("Welcome to the Decentralized Exchange!");

    // Create an instance of the decentralized exchange
    let mut exchange = Exchange::new();

    // Main loop for user interaction
    loop {
        println!("\nPlease select an option:");
        println!("1. Add order");
        println!("2. Remove order");
        println!("3. Get order");
        println!("4. Match orders");
        println!("5. Exit");

        let mut choice = String::new();

        // Read user input
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        // Parse user choice
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => add_order(&mut exchange),
            2 => remove_order(&mut exchange),
            3 => get_order(&exchange),
            4 => exchange.match_orders(),
            5 => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice. Please enter a number between 1 and 5."),
        }
    }
}

// Define the add order function
fn add_order(exchange: &mut Exchange) {
    println!("Enter your trader name:");
    let mut trader = String::new();
    io::stdin().read_line(&mut trader).expect("Failed to read line");
    let trader = trader.trim().to_string();

    println!("Enter order amount:");
    let mut amount = String::new();
    io::stdin().read_line(&mut amount).expect("Failed to read line");
    let amount: u64 = match amount.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid amount.");
            return;
        }
    };

    println!("Enter order price:");
    let mut price = String::new();
    io::stdin().read_line(&mut price).expect("Failed to read line");
    let price: u64 = match price.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid price.");
            return;
        }
    };

    exchange.add_order(Order {
        trader,
        amount,
        price,
    });
    println!("Order added successfully!");
}

// Define the remove order function
fn remove_order(exchange: &mut Exchange) {
    println!("Enter your trader name:");
    let mut trader = String::new();
    io::stdin().read_line(&mut trader).expect("Failed to read line");
    let trader = trader.trim().to_string();

    exchange.remove_order(&trader);
    println!("Order removed successfully!");
}

// Define the get order function
fn get_order(exchange: &Exchange) {
    println!("Enter your trader name:");
    let mut trader = String::new();
    io::stdin().read_line(&mut trader).expect("Failed to read line");
    let trader = trader.trim();

    match exchange.get_order(trader) {
        Some(order) => {
            println!("Order found:");
            println!("Trader: {}", order.trader);
            println!("Amount: {}", order.amount);
            println!("Price: {}", order.price);
        }
        None => println!("Order not found."),
    }
}
