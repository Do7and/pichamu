# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Pichamu is a Flutter-based photo management application with a Rust backend. The project uses a hybrid architecture with Flutter for the UI and Rust for performance-critical operations like image processing, database management, and background tasks.

## Architecture

### Multi-language Structure
- **Flutter Frontend** (`lib/`): UI components using Riverpod for state management
- **Rust Core** (`pichamu_core/`): Business logic, image processing, database operations
- **Rust Bridge** (`native/`): Flutter-Rust bridge using flutter_rust_bridge
- **Generated Code** (`lib/bridge/generated/`): Auto-generated Dart bindings

### Key Dependencies
- **Flutter**: photo_manager, permission_handler, image_picker, riverpod, dio
- **Rust**: image processing (image, image_hasher), database (rusqlite), async (tokio), HTTP (reqwest), job queue (apalis)

## Development Constraints

**NEVER execute these commands** (user will handle due to performance):
- `flutter build`
- `flutter pub get` 
- `flutter run`
- `cargo build`
- `cargo run`
- Any `flutter-rust-bridge` commands

## Error Checking Commands

After modifying code, use these commands to check for errors:

**Flutter code changes:**
```bash
flutter analyze | grep error
```

**Rust code changes:**
```bash
cargo check | grep error
```

## Flutter-Rust Bridge Development Rules

### Critical Bridge Principles
- **API Entry Point**: `native/src/api.rs` is the **ONLY** file for Flutter-callable functions
- **build.rs Configuration**: Bridge build configuration is already set up - `cargo build` generates bridge code
- **All Flutter-callable functions must be in `native/src/api.rs`** with appropriate `frb` attributes:
  - `#[frb(sync)]` for synchronous functions
  - `#[frb(mirror)]` for mirrored data structures
  - Default async for async functions

### Bridge Workflow
1. Write Rust functions in `native/src/api.rs` with proper `frb` attributes
2. After user runs `cargo build`, generated Dart code appears in `lib/bridge/generated/`  
3. **Always verify generated Dart files are correct** after bridge code generation

### Bridge Configuration
- Rust input: `native/src/api`
- Dart output: `lib/bridge/generated`
- Auto-formatting enabled for Dart output
- Bridge configuration in `pubspec.yaml` under `flutter_rust_bridge`

## Project Structure

### Workspace Configuration
- Cargo workspace with two members: `pichamu_core` and `native`
- Shared dependencies defined in workspace root `Cargo.toml`
- Core business logic separated from Flutter bridge code

### Key Directories
- `lib/`: Flutter application code
- `native/`: Flutter-Rust bridge implementation
- `pichamu_core/`: Core Rust library with business logic
- `android/`, `ios/`, `linux/`, `macos/`, `windows/`, `web/`: Platform-specific code

## Development Notes

### Rust Bridge Development
- Core logic goes in `pichamu_core` crate
- Bridge exports go in `native/src/api.rs`
- Generated files should not be manually edited
- Use `flutter_rust_bridge` version 2.11.1

### State Management
- Uses Riverpod for Flutter state management
- Image management via photo_manager package
- Permissions handled through permission_handler

### Database & Storage
- SQLite database through rusqlite
- Async operations using tokio
- Background job processing with apalis