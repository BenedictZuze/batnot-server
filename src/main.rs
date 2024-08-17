use axum::http::StatusCode;
use axum::routing::get;
use axum::Json;
use axum::Router;
use battery::Manager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new().route("/check", get(check_battery));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

#[derive(serde::Serialize)]
struct BatteryInfo {
    percentage: String,
    health: String,
}

// For error handling
// #[derive(thiserror::Error, Debug)]
// enum BatteryError {
//     #[error("Unable to access battery information")] AccessError(#[from] battery::Error),
//     #[error("No batteries found")] NoBatteries,
// }

async fn check_battery() -> Result<Json<BatteryInfo>, (StatusCode, String)> {
    let manager = Manager::new().unwrap();
    let mut battery = manager
        .batteries()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        .unwrap()
        .next()
        .ok_or((StatusCode::NOT_FOUND, "No batteries found".to_string()))
        .unwrap()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        .unwrap();

    let percentage = format!("{:?}", battery.state_of_charge() * 100.0);
    let health = format!("{:?}", battery.state_of_health() * 100.0);

    manager
        .refresh(&mut battery)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        .unwrap();

    Ok(Json(BatteryInfo { percentage, health }))
}
