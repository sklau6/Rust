// Customers table
table! {
    customers (id) {
        id              -> Int4,
        customer_name   -> Varchar,
        customer_address-> Text,
    }
}

// Orders table
table! {
    orders (id) {
        id             -> Int4,
        product_name   -> Varchar,
        product_id     -> Int4,
        customer_id    -> Int4,
        amount         -> Int4,
        address        -> Text,
        fulfilled      -> Bool,
    }
}

// Products table
table! {
    products (id) {
        id             -> Int4,
        product_name   -> Varchar,
        product_type   -> Varchar,
        amount         -> Int4,
    }
}

// Stocks table
table! {
    stocks (id) {
        id             -> Int4,
        product_name   -> Varchar,
        product_id     -> Int4,
        amount         -> Int4,
    }
}

// Allowing the tables to appear in the same query
allow_tables_to_appear_in_same_query!(
    customers,
    orders,
    products,
    stocks,
);
