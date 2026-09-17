use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use reqwest::Client;
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::net::TcpListener;
use yup_oauth2::{read_service_account_key, ServiceAccountAuthenticator, authenticator::Authenticator};
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;

#[derive(Clone)]

// Since Authenticator type is complex, let's just initialize it per request 
// OR we can store the credentials path and build it, but actually caching the token is better.
// To keep it simple and match the generic approach, let's store the `Authenticator` in state.
struct GlobalState {
    api_key: String,
    auth: Authenticator<HttpsConnector<HttpConnector>>,
    client: Client,
}

#[derive(Deserialize)]
struct AnalyticsRequest {
    property_id: String,
    report_type: Option<String>, // "overview", "brands", "screens"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let _ = dotenvy::dotenv();

    let api_key = std::env::var("API_KEY").unwrap_or_else(|_| "secret".to_string());
    let key_path = std::env::var("GOOGLE_APPLICATION_CREDENTIALS")
        .unwrap_or_else(|_| "service_account.json".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    let secret = read_service_account_key(&key_path)
        .await
        .expect("Failed to read service account key");

    let auth = ServiceAccountAuthenticator::builder(secret)
        .build()
        .await
        .expect("Failed to create authenticator");

    let state = Arc::new(GlobalState {
        api_key,
        auth,
        client: Client::new(),
    });

    let app = Router::new()
        .route("/api/v1/analytics/report", post(get_analytics_data))
        .route("/api/report", post(get_analytics_data)) // Backward compatibility
        .with_state(state);

    let addr = format!("0.0.0.0:{}", port);
    println!("🚀 Analytics Backend is running on http://{}", addr);
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_analytics_data(
    headers: HeaderMap,
    State(state): State<Arc<GlobalState>>,
    Json(payload): Json<AnalyticsRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    
    // 1. Check API Key
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");
    let expected = format!("Bearer {}", state.api_key);
    if auth_header != expected && state.api_key != "unsecured" {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Invalid API Key"})),
        ));
    }

    // 2. Generate Token
    let scopes = &["https://www.googleapis.com/auth/analytics.readonly"];
    let token = state.auth.token(scopes).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Auth error: {}", e)})),
        )
    })?;

    // 3. Build Google Analytics Data API Request
    let url = format!(
        "https://analyticsdata.googleapis.com/v1beta/properties/{}:runReport",
        payload.property_id
    );

    let mut dimensions = vec![json!({"name": "eventName"})];
    let mut dimension_filter = json!(null);

    match payload.report_type.as_deref() {
        Some("brands") => {
            dimensions = vec![json!({"name": "customEvent:item_name"})];
            dimension_filter = json!({
                "filter": {
                    "fieldName": "eventName",
                    "stringFilter": { "value": "brand_click" }
                }
            });
        }
        Some("screens") => {
            dimensions = vec![json!({"name": "pageTitle"})];
            dimension_filter = json!({
                "filter": {
                    "fieldName": "eventName",
                    "stringFilter": { "value": "screen_view" }
                }
            });
        }
        _ => {}
    }

    let mut request_body = json!({
        "dateRanges": [{"startDate": "7daysAgo", "endDate": "today"}],
        "metrics": [{"name": "activeUsers"}, {"name": "eventCount"}],
        "dimensions": dimensions,
    });

    if !dimension_filter.is_null() {
        request_body["dimensionFilter"] = dimension_filter;
    }

    // 4. Send Request
    let res = state
        .client
        .post(&url)
        .bearer_auth(token.token().unwrap_or(""))
        .json(&request_body)
        .send()
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Request failed: {}", e)})),
            )
        })?;

    let json_res: Value = res.json().await.map_err(|e| {
        (
            StatusCode::BAD_GATEWAY,
            Json(json!({"error": format!("Failed to parse response: {}", e)})),
        )
    })?;

    Ok((StatusCode::OK, Json(json_res)))
}
