mod insrcdata;

// N-N join : many-to-many relationships
//
// insrcdata doesn't support natively many-to-many relationship between two tables.
// You can solve the prblem with a third table which maintain two one-to-many relationships with the previous tables.
//
// This exemple maintain the many-to-many relationship between Client et Product with the Transaction table

// All product for a client
fn print_product_list(client: &insrcdata::Client) {
    print!("*  {}:", client.name());
    for transaction in client.transactions() {
        print!(" {}", transaction.product().name());
    }
    println!();
}

// All clients for a product
fn print_client_list(product: &insrcdata::Product) {
    print!("*  {}:", product.name());
    for transaction in product.transactions() {
        print!(" {}", transaction.client().name());
    }
    println!();
}

fn main() {
    println!("By clients");
    print_product_list(&insrcdata::Clients::John);
    print_product_list(&insrcdata::Clients::Alix);
    print_product_list(&insrcdata::Clients::David);

    println!("\nBy products");
    print_client_list(&insrcdata::Products::Apple);
    print_client_list(&insrcdata::Products::Banana);
    print_client_list(&insrcdata::Products::Peach);
    print_client_list(&insrcdata::Products::Cherry);
}
