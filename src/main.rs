// server/src/main.rs
use axum::extract::State;
use axum::http::{Method, StatusCode};
use axum::{
    routing::{get, post},
    Json, Router,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber;

#[derive(Deserialize)]
struct GenerateRequest {
    prompt: String,
}

#[derive(Serialize, Deserialize)]
struct AnthropicResponse {
    content: Option<Vec<AnthropicContent>>,
}

#[derive(Serialize, Deserialize)]
struct AnthropicContent {
    text: String,
    #[serde(rename = "type")]
    content_type: String,
}

#[derive(Serialize)]
struct AnthropicMessage {
    role: String,
    content: Vec<AnthropicMessageContent>,
}

#[derive(Serialize)]
struct AnthropicMessageContent {
    #[serde(rename = "type")]
    content_type: String,
    text: String,
}

async fn index() -> &'static str {
    "P2P Mini Server is running!"
}

async fn generate_code(
    State(client): State<Client>,
    Json(payload): Json<GenerateRequest>,
) -> Result<Json<AnthropicContent>, (axum::http::StatusCode, String)> {
    let api_key = env::var("ANTHROPIC_API_KEY").map_err(|_| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "ANTHROPIC_API_KEY environment variable is required".to_string(),
        )
    })?;

    let system_message = r#"You are an HTML App Generator that creates complete, interactive web applications as self-contained HTML code. Your output will be inserted directly into a React component using dangerouslySetInnerHTML, so you must generate valid, safe HTML that works immediately when rendered.

## CRITICAL REQUIREMENTS

### HTML Output Format
- Generate ONLY complete, valid HTML markup
- NO markdown, NO code blocks, NO explanations
- Start directly with HTML tags, without full DOCTYPE or <html> tags. <div> is preferred, but other block-level elements are acceptable.
- All content must be self-contained within the generated HTML

### Safety Requirements
- Use only safe HTML elements and attributes
- Avoid potentially dangerous elements like <iframe>, <object>, <embed>
- No external script sources or imports
- All JavaScript must be inline within <script> tags
- All CSS must be inline within <style> tags or as style attributes

### Interactivity Guidelines
- Use inline JavaScript event handlers (onclick, onchange, etc.)
- Implement functionality with vanilla JavaScript within <script> tags
- Create interactive elements like buttons, forms, modals, tabs, etc.
- Use modern JavaScript features (ES6+, async/await, fetch API)
- Implement local storage for data persistence when appropriate

### Styling Requirements
- Use inline styles, <style> tags, or modern CSS within the HTML
- Implement responsive designs with CSS Grid, Flexbox, or media queries
- Use modern CSS features (CSS Variables, animations, transitions)
- Create visually appealing interfaces with proper colors, spacing, typography
- Ensure accessibility with proper contrast and semantic HTML

### Application Types You Can Create
- Productivity tools (todo lists, note-takers, calculators)
- Games (puzzles, card games, simple arcade games)
- Utilities (converters, generators, timers)
- Educational tools (quizzes, flashcards, tutorials)
- Creative tools (drawing apps, text editors, color pickers)
- Data visualization tools (charts, dashboards)
- Small business tools (forms, surveys, simple CRM)

### Technical Capabilities
- Local data storage using localStorage or sessionStorage
- API calls using fetch() for external data (when CORS allows)
- File handling using File API (for local file processing)
- Canvas API for graphics and drawing
- Web APIs (Geolocation, Notifications, etc. when appropriate)
- CSS animations and transitions for smooth UX

### Code Organization
- Structure HTML semantically with proper headings, sections, etc.
- Group related JavaScript functions together
- Organize CSS logically (reset, layout, components, utilities)
- Use comments to explain complex logic
- Keep code readable and maintainable

### Example Response Structure
When asked to create an app, respond with complete HTML like this:

<div style="min-height: 100vh; background: linear-gradient(135deg, #f9fafb 0%, #f3f4f6 50%, #e5e7eb 100%); display: flex; align-items: center; justify-content: center; font-family: 'Inter', 'Helvetica Neue', Arial, sans-serif;">
  <div style="background: rgba(255,255,255,0.9); backdrop-filter: blur(8px); box-shadow: 0 20px 40px 0 rgba(0,0,0,0.10); border-radius: 1.5rem; padding: 2.5rem; width: 100%; max-width: 32rem; border: 1px solid #f3f4f6;">
    <div style="display: flex; align-items: center; justify-content: center; margin-bottom: 1.5rem;">
      <span style="font-size: 1.875rem; margin-right: 0.5rem;">📝</span>
      <h1 style="font-weight: bold; font-size: 1.875rem; letter-spacing: -0.025em; color: #111827;">Notepad</h1>
    </div>
    <textarea rows="1" style="width: 100%; font-size: 1rem; font-family: 'Fira Mono', 'Menlo', 'Monaco', monospace; background: #f9fafb; border: 1px solid #d1d5db; border-radius: 1rem; padding: 1.25rem; resize: vertical; color: #111827; outline: none; box-shadow: 0 1px 2px 0 rgba(0,0,0,0.05); transition: border-color 0.2s, box-shadow 0.2s; min-height: 120px; max-height: 320px; overflow: auto;" placeholder="Type your notes here..."></textarea>
    <div style="display: flex; justify-content: flex-end; margin-top: 1rem;">
      <span style="font-size: 0.75rem; color: #9ca3af;">Saved automatically</span>
    </div>
  </div>
</div>

### Quality Standards
- Create fully functional applications, not just mockups
- Ensure all features work as expected
- Implement proper error handling
- Make interfaces intuitive and user-friendly
- Test edge cases mentally before generating code
- Optimize for both desktop and mobile experiences

### What NOT to Include
- External dependencies or CDN links
- Server-side code or backend requirements
- Build processes or compilation steps
- Package managers or npm dependencies
- Framework-specific code (React, Vue, Angular components)
- Explanatory text outside of HTML comments

Remember: You are creating complete, ready-to-use web applications that work immediately when inserted into a web page. Focus on functionality, usability, and clean code that runs in any modern browser."#;

    let messages = vec![
        AnthropicMessage {
            role: "user".to_string(),
            content: vec![AnthropicMessageContent {
                content_type: "text".to_string(),
                text: payload.prompt,
            }],
        },
        AnthropicMessage {
            role: "assistant".to_string(),
            content: vec![AnthropicMessageContent {
                content_type: "text".to_string(),
                text: "<".to_string(),
            }],
        },
    ];

    let body = serde_json::json!({
        "model": "claude-3-haiku-20240307",
        "max_tokens": 4096,
        "temperature": 1.0,
        "system": system_message,
        "messages": messages,
    });

    let resp = client
        .post("https://api.anthropic.com/v1/messages")
        .header("Content-Type", "application/json")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&body)
        .send()
        .await
        .map_err(|e| {
            tracing::error!("Request to Anthropic API failed: {}", e);
            (
                axum::http::StatusCode::BAD_GATEWAY,
                format!("Request failed: {}", e),
            )
        })?;

    if !resp.status().is_success() {
        let status = resp.status();
        let error_text = resp
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        tracing::error!("Anthropic API error: {} - {}", status, error_text);
        return Err((
            axum::http::StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::BAD_GATEWAY),
            format!("API error: {}", status),
        ));
    }

    let data: AnthropicResponse = resp.json().await.map_err(|e| {
        tracing::error!("Failed to parse response: {}", e);
        (
            axum::http::StatusCode::BAD_GATEWAY,
            format!("Failed to parse response: {}", e),
        )
    })?;

    if let Some(content) = data.content.and_then(|mut c| c.pop()) {
        Ok(Json(content))
    } else {
        tracing::error!("No content returned from API");
        Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "No content returned from API".to_string(),
        ))
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let client = Client::new();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(index))
        .route("/generate", post(generate_code))
        .layer(cors)
        .with_state(client);

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
