// Imports necessary modules and functions
use crate::api::core_handler::get_payload_bytes;
use crate::models::Stock;
use crate::services::stock_service::*;
use actix_web::{error, web, Error, HttpResponse, Responder};
use serde_derive::{Deserialize, Serialize};
use serde_json;

// Defines the structure of stock increment input
#[derive(Serialize, Deserialize, Debug)]
pub struct StockIncr {
    id: i32,
    incr_amount: i32,
}

// Function to increment stock and send a response
pub async fn stock_increment(data: web::Json<StockIncr>) -> Result<HttpResponse, Error> {
    // Call to increment stock function from service
    let stock = increment_stock(data.id, data.incr_amount);

    // Sending the response
    Ok(HttpResponse::Ok().json(stock))
}

// Function to get list of all stocks
pub async fn stock_list() -> Result<impl Responder, Error> {
    // Call to get stock function from service
    let stocks = get_stock();

    // Check if stocks is empty, return error if true
    if stocks.len() > 0 {
        Ok(HttpResponse::Ok().json(stocks))
    } else {
        Err(error::ErrorBadRequest("No Stock"))
    }
}

// Function to create a new stock
pub async fn stock_create(payload: web::Payload) -> Result<HttpResponse, Error> {
    // Convert payload into bytes
    let body = get_payload_bytes(payload).await;

    match body {
        Ok(b) => {
            // Deserialize the body
            let obj = serde_json::from_slice::<Stock>(&b)?;

            // Call to create stock function from service
            let created_stock = create_stock(&obj);

            // Send the response
            Ok(HttpResponse::Ok().json(created_stock))
        }
        Err(e) => Err(e),
    }
}

// Function to delete a stock
pub async fn stock_delete(payload: web::Payload) -> Result<HttpResponse, Error> {
    // Convert payload into bytes
    let body = get_payload_bytes(payload).await;

    match body {
        Ok(b) => {
            // Deserialize the body
            let obj = serde_json::from_slice::<Stock>(&b)?;

            // Call to delete stock function from service
            let return_info = delete_stock(&obj);

            // Send the response
            Ok(HttpResponse::Ok().json(return_info))
        }
        Err(e) => Err(e),
    }
}

// Function to update a stock
pub async fn stock_update(payload: web::Payload) -> Result<HttpResponse, Error> {
    // Convert payload into bytes
    let body = get_payload_bytes(payload).await;

    match body {
        Ok(b) => {
            // Deserialize the body
            let obj = serde_json::from_slice::<Stock>(&b)?;

            // Call to update stock function from service
            let stock = update_stock(&obj);

            // Send the response
            Ok(HttpResponse::Ok().json(stock))
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::stock_handler;
    use crate::models::{ReturnInfo, Stock};
    use actix_web::{test, web, App};

    async fn create_test_stock(payload: &'static [u8]) -> Stock {
        let mut app = test::init_service(App::new().service(
            web::resource("/stock/create").route(web::post().to(stock_handler::stock_create)),
        ))
        .await;

        let req = test::TestRequest::post()
            .uri("http://localhost:8080/stock/create")
            .header("content-type", "application/json")
            .set_payload(payload)
            .to_request();

        let created_stock: Stock = test::read_response_json(&mut app, req).await;

        created_stock
    }

    async fn delete_test_stock(stock_id: i32) -> usize {
        let mut app = test::init_service(App::new().service(
            web::resource("/stock/delete").route(web::post().to(stock_handler::stock_delete)),
        ))
        .await;

        let stock = Stock {
            id: stock_id,
            product_name: String::from(""),
            product_id: 0,
            amount: 0,
        };

        let payload = serde_json::to_string(&stock).unwrap();

        let req = test::TestRequest::post()
            .uri("http://localhost:8080/stock/delete")
            .header("content-type", "application/json")
            .set_payload(payload)
            .to_request();

        let return_info: ReturnInfo = test::read_response_json(&mut app, req).await;

        return_info.amount
    }

    #[actix_rt::test]
    async fn test_index_create_stock() {
        let mut app = test::init_service(App::new().service(
            web::resource("/stock/create").route(web::post().to(stock_handler::stock_create)),
        ))
        .await;

        let payload = r#"{"id":0,
        "product_name":"Harry Potter",
        "product_id":3,
        "amount":3
    }"#
        .as_bytes();

        let req = test::TestRequest::post()
            .uri("http://localhost:8080/stock/create")
            .header("content-type", "application/json")
            .set_payload(payload)
            .to_request();

        let created_stock: Stock = test::read_response_json(&mut app, req).await;

        assert_eq!(created_stock.amount, 3);

        let num_deleted = delete_test_stock(created_stock.id).await;

        assert_eq!(num_deleted, 1);
    }

    async fn create_test_stocks_for_update_test() {
        let payload = r#"{"id":0,
        "product_name":"Hitch Hikers Guide to the Galaxy",
        "product_id":123,
        "amount":23
    }"#
        .as_bytes();

        create_test_stock(&payload).await;
    }

    #[actix_rt::test]
    async fn test_index_update_stock() {
        create_test_stocks_for_update_test().await;

        let mut app = test::init_service(App::new().service(
            web::resource("/stock/update").route(web::post().to(stock_handler::stock_update)),
        ))
        .await;

        let payload = r#"{"id":0,
        "product_name":"Hitch Hikers Guide to the Galaxy",
        "product_id":123,
        "amount":223
    }"#
        .as_bytes();

        let req = test::TestRequest::post()
            .uri("http://localhost:8080/stock/update")
            .header("content-type", "application/json")
            .set_payload(payload)
            .to_request();

        let created_stock: Stock = test::read_response_json(&mut app, req).await;

        assert_eq!(created_stock.amount, 223);

        delete_test_stock(created_stock.id).await;
    }

    async fn create_test_stocks_for_increment_test() -> Stock {
        let payload = r#"{"id":0,
        "product_name":"Harry Potter 6",
        "product_id":987,
        "amount":2
    }"#
        .as_bytes();

        create_test_stock(&payload).await
    }

    #[actix_rt::test]
    async fn test_index_increment_stock() {
        let stock = create_test_stocks_for_increment_test().await;
        let mut app = test::init_service(App::new().service(
            web::resource("/stock/increment").route(web::post().to(stock_handler::stock_increment)),
        ))
        .await;

        let stock_incr = StockIncr {
            id: stock.id,
            incr_amount: 5,
        };

        let stock_incr_string = serde_json::to_string(&stock_incr).unwrap();

        let req = test::TestRequest::post()
            .uri("http://localhost:8080/stock/increment")
            .header("content-type", "application/json")
            .set_payload(stock_incr_string)
            .to_request();

        let created_stock: Stock = test::read_response_json(&mut app, req).await;

        assert_eq!(created_stock.amount, 7);

        delete_test_stock(created_stock.id).await;
    }

    async fn create_test_stocks_for_list_test() {
        let payload = r#"{"id":0,
        "product_name":"Harry Potter 6",
        "product_id":987,
        "amount":2
    }"#
        .as_bytes();

        create_test_stock(&payload).await;
    }

    #[actix_rt::test]
    async fn test_index_list_stock() {
        create_test_stocks_for_list_test().await;
        //let mut app = test::init_service(App::new().service(stock_handler::stock_list)).await;
        let mut app = test::init_service(
            App::new().route("/stock/list", web::get().to(stock_handler::stock_list)),
        )
        .await;

        let payload = r#""#.as_bytes();

        let req = test::TestRequest::get()
            .uri("http://localhost:8080/stock/list")
            .header("content-type", "application/json")
            .set_payload(payload)
            .to_request();

        let stocks: Vec<Stock> = test::read_response_json(&mut app, req).await;

        assert!(stocks.len() > 0);

        delete_test_stock(stocks[0].id).await;
    }

    async fn create_test_stocks_for_delete_test() -> Stock {
        let payload = r#"{"id":0,
        "product_name":"Harry Potter 6",
        "product_id":987,
        "amount":2
    }"#
        .as_bytes();

        create_test_stock(&payload).await
    }

    #[actix_rt::test]
    async fn test_index_delete_stock() {
        let stock = create_test_stocks_for_delete_test().await;
        let mut app = test::init_service(App::new().service(
            web::resource("/stock/delete").route(web::post().to(stock_handler::stock_delete)),
        ))
        .await;

        let stock_to_delete = serde_json::to_string(&stock).unwrap();

        let req = test::TestRequest::post()
            .uri("http://localhost:8080/stock/delete")
            .header("content-type", "application/json")
            .set_payload(stock_to_delete)
            .to_request();

        let return_info: ReturnInfo = test::read_response_json(&mut app, req).await;

        assert_eq!(return_info.amount, 1);
    }
}
