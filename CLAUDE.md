# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Lightweight Rust server for a peer-to-peer platform. Part of a decentralized ecosystem built on Bitcoin/Lightning, IPv6 (no domains), PWA architecture, L402 payments, and front-end loaded design. Server generates self-contained HTML apps via Anthropic's Claude API to minimize server load and maximize client-side processing.

## Development Commands

### Core Commands
- `cargo run` - Start the development server (default port 10000, configurable via PORT env var)
- `cargo build` - Build the project
- `cargo test` - Run tests
- `cargo check` - Fast compilation check without producing executables

### Environment Setup
- Create `.env` file with `ANTHROPIC_API_KEY=your_api_key_here`
- Optional: Set `PORT` environment variable to change server port

### Debug Mode
- `RUST_LOG=debug cargo run` - Run with detailed logging

## Architecture

### Front-End Loaded Design
Single-file server (`src/main.rs`) keeps backend lean - all heavy processing happens client-side. Server only handles web data serving and CRUD operations.

### HTTP Server (Axum)
- **Routes**: `GET /` (health check), `POST /generate` (HTML generation)
- **CORS**: All origins allowed for decentralized access
- **State**: Shared reqwest client for Anthropic API

### AI Integration
- **Model**: Claude 3 Haiku via Messages API
- **Purpose**: Generates complete, self-contained HTML apps
- **System Prompt**: Extensive prompt engineering (lines 59-146)
- **Output**: Returns AnthropicContent with generated HTML

### Generated Applications
Creates self-contained PWA-compatible HTML with:
- Inline CSS/JavaScript (no external dependencies)
- Local storage for offline capability
- Responsive design (mobile-first, 3D UI capable)
- Interactive features using vanilla JavaScript

## Dependencies
- **axum** - Web framework
- **tokio** - Async runtime
- **serde/serde_json** - JSON serialization
- **reqwest** - HTTP client for external API calls
- **tower-http** - CORS middleware
- **tracing/tracing-subscriber** - Logging
- **dotenv** - Environment variable loading

## Configuration
- Server binds to `0.0.0.0:{PORT}` (default: 10000) for IPv6 compatibility
- Requires `ANTHROPIC_API_KEY` environment variable
- Uses Claude 3 Haiku model with 4096 max tokens and temperature 1.0
- Designed for <1GB RAM constraint and peer-to-peer deployment