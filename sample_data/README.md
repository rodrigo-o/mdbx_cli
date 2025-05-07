# Sample Data for Testing

This directory contains a sample data .dat file for testing purposes. The data is structured in a way that allows for easy parsing and manipulation.
The files have a couple of different tables:

- **users**: Contains user information such as name and email.
- **products**: Contains product information such as name and price.
- **orders**: Contains order information such as user, product, and quantity.   
- **inventory**: Contains inventory information such as product and stock.

Here is it in plain rust format:

```rust
    (
        "users",
        vec![
            ("user_1", r#"{"name": "Alice", "email": "alice@example.com"}"#),
            ("user_2", r#"{"name": "Bob", "email": "bob@example.com"}"#),
            ("user_3", r#"{"name": "Charlie", "email": "charlie@example.com"}"#),
            ("user_4", r#"{"name": "David", "email": "david@example.com"}"#),
            ("user_5", r#"{"name": "Eve", "email": "eve@example.com"}"#),
        ],
    ),
    (
        "products",
        vec![
            ("prod_1", r#"{"name": "Laptop", "price": 1200.00}"#),
            ("prod_2", r#"{"name": "Mouse", "price": 25.00}"#),
            ("prod_3", r#"{"name": "Keyboard", "price": 45.00}"#),
            ("prod_4", r#"{"name": "Monitor", "price": 300.00}"#),
            ("prod_5", r#"{"name": "Printer", "price": 150.00}"#),
        ],
    ),
    (
        "orders",
        vec![
            ("order_1", r#"{"user": "user_1", "product": "prod_1", "quantity": 1}"#),
            ("order_2", r#"{"user": "user_2", "product": "prod_2", "quantity": 2}"#),
            ("order_3", r#"{"user": "user_3", "product": "prod_3", "quantity": 1}"#),
            ("order_4", r#"{"user": "user_4", "product": "prod_4", "quantity": 3}"#),
            ("order_5", r#"{"user": "user_5", "product": "prod_5", "quantity": 1}"#),
        ],
    ),
    (
        "inventory",
        vec![
            ("inv_1", r#"{"product": "prod_1", "stock": 50}"#),
            ("inv_2", r#"{"product": "prod_2", "stock": 200}"#),
            ("inv_3", r#"{"product": "prod_3", "stock": 150}"#),
            ("inv_4", r#"{"product": "prod_4", "stock": 75}"#),
            ("inv_5", r#"{"product": "prod_5", "stock": 100}"#),
        ],
    )
```