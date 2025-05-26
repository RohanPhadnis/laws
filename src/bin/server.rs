use std::sync::Arc;
use axum::http::Response;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;
use laws::errors::DbError;

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

fn convert_to_response(result: Result<serde_json::Value, laws::errors::DbError>) -> (axum::http::StatusCode, axum::Json<serde_json::Value>) {
    match result {
        Ok(result) => {
            (
                axum::http::StatusCode::OK,
                axum::Json::from(result),
            )
        }
        Err(e) => {
            let status_code = match e {
                DbError::MissingFields(_) => {axum::http::StatusCode::FAILED_DEPENDENCY}
                DbError::TableNotFound(_) => {axum::http::StatusCode::NOT_FOUND}
                DbError::BadInput(_) => {axum::http::StatusCode::BAD_REQUEST}
            };
            (
                status_code,
                axum::Json::from(serde_json::json!({"message": e.to_string()})),
                )
        }
    }
}

#[tokio::main]
async fn main() {

    let db = std::sync::Arc::new(laws::data_structures::Database::new("./data"));

    let app = axum::Router::new()
        // DB level CRUD
        .route(
            "/db",
            axum::routing::get(async |db: axum::extract::State<std::sync::Arc<laws::data_structures::Database>>| {
                convert_to_response(db.read_db().await).into_response()
            })
        )

        // table level CRUD
        .route(
            "/db/table/{table_name}",
               axum::routing::post(async |axum::extract::Path(table_name): axum::extract::Path<String>, db: axum::extract::State<std::sync::Arc<laws::data_structures::Database>>, axum::Json(info): axum::Json<serde_json::Value>| {
                   let db = db.clone();
                   convert_to_response(db.create_table(info).await).into_response()
               })
                   .get(async |axum::extract::Path(table_name): axum::extract::Path<String>, db: axum::extract::State<std::sync::Arc<laws::data_structures::Database>>, axum::Json(info): axum::Json<serde_json::Value>| {
                       convert_to_response(db.read_table(&table_name).await).into_response()
                   })
                   .delete(async |axum::extract::Path(table_name): axum::extract::Path<String>, db: axum::extract::State<std::sync::Arc<laws::data_structures::Database>>, axum::Json(info): axum::Json<serde_json::Value>| {
                       convert_to_response(db.delete_table(&table_name).await).into_response()
                   })
        )

        // document level CRUD
        .route(
            "/db/table/{table_name}/doc",
               axum::routing::post(async |axum::extract::Path(table_name): axum::extract::Path<String>, db: axum::extract::State<std::sync::Arc<laws::data_structures::Database>>, axum::Json(info): axum::Json<serde_json::Value>| {
                   convert_to_response(db.create_document(&table_name, info).await).into_response()
               })
                   .get(async |axum::extract::Path(table_name): axum::extract::Path<String>, db: axum::extract::State<std::sync::Arc<laws::data_structures::Database>>, axum::Json(info): axum::Json<serde_json::Value>| {
                       convert_to_response(db.read_document(&table_name, info).await).into_response()
                   })
                   .put(async |axum::extract::Path(table_name): axum::extract::Path<String>, db: axum::extract::State<std::sync::Arc<laws::data_structures::Database>>, axum::Json(info): axum::Json<serde_json::Value>| {
                       convert_to_response(db.update_document(&table_name, info).await).into_response()
                   })
                   .delete(async |axum::extract::Path(table_name): axum::extract::Path<String>, db: axum::extract::State<std::sync::Arc<laws::data_structures::Database>>, axum::Json(info): axum::Json<serde_json::Value>| {
                       convert_to_response(db.delete_document(&table_name, info).await).into_response()
                   })
        )
        .with_state(db.clone());

    let listener = tokio::net::TcpListener::bind("[::1]:6969").await.unwrap();
    axum::serve(listener, app).with_graceful_shutdown(shutdown_signal()).await.unwrap();

    db.save().await;
}