# P2P Mini Server

A lightweight HTTP server built with Rust that provides an API for generating HTML applications using Anthropic's Claude AI model.

## Features

- **AI-Powered HTML Generation**: Generate complete, interactive web applications as self-contained HTML
- **RESTful API**: Simple HTTP endpoints for code generation
- **CORS Support**: Pre-configured for cross-origin requests
- **Environment Configuration**: Secure API key management via environment variables
- **Logging**: Built-in request tracing and error logging

## Quick Start

### Prerequisites

- Rust (latest stable version)
- Anthropic API key

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd p2p-mini-server
```

2. Set up environment variables:
```bash
echo "ANTHROPIC_API_KEY=your_api_key_here" > .env
```

3. Run the server:
```bash
cargo run
```

The server will start on `http://127.0.0.1:8080`

## API Endpoints

### GET `/`
Health check endpoint that returns server status.

**Response:**
```
P2P Mini Server is running!
```

### POST `/generate`
Generate HTML applications using AI.

**Request Body:**
```json
{
  "prompt": "Create a todo list app"
}
```

**Response:**
```json
{
  "text": "<div>...generated HTML code...</div>",
  "type": "text"
}
```

## Usage Examples

### Generate a Todo List App
```bash
curl -X POST http://127.0.0.1:8080/generate \
  -H "Content-Type: application/json" \
  -d '{"prompt": "Create a responsive todo list with local storage"}'
```

### Generate a Calculator
```bash
curl -X POST http://127.0.0.1:8080/generate \
  -H "Content-Type: application/json" \
  -d '{"prompt": "Build a scientific calculator with history"}'
```

## Configuration

### Environment Variables

- `ANTHROPIC_API_KEY` (required): Your Anthropic API key for Claude access

### Server Configuration

- **Host**: `127.0.0.1`
- **Port**: `8080`
- **CORS**: Enabled for all origins and methods

## Generated Applications

The server generates complete, self-contained HTML applications that include:

- **Inline CSS**: All styling embedded within the HTML
- **Vanilla JavaScript**: No external dependencies
- **Responsive Design**: Mobile-friendly layouts
- **Local Storage**: Data persistence capabilities
- **Interactive Features**: Buttons, forms, animations, etc.

### Supported Application Types

- Productivity tools (todo lists, note-takers, calculators)
- Games (puzzles, card games, simple arcade games)
- Utilities (converters, generators, timers)
- Educational tools (quizzes, flashcards, tutorials)
- Creative tools (drawing apps, text editors, color pickers)
- Data visualization tools (charts, dashboards)

## Dependencies

- **axum**: Web framework for HTTP server
- **tokio**: Async runtime
- **serde**: JSON serialization/deserialization
- **reqwest**: HTTP client for Anthropic API
- **tower-http**: CORS middleware
- **tracing**: Logging and observability
- **dotenv**: Environment variable loading

## Error Handling

The server provides detailed error responses for common issues:

- Missing API key configuration
- Anthropic API failures
- Invalid request formats
- Network connectivity issues

## Development

### Building
```bash
cargo build
```

### Running Tests
```bash
cargo test
```

### Debug Mode
```bash
RUST_LOG=debug cargo run
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License

MIT
