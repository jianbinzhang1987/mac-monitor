# Audit Service Module

This directory contains the background service responsible for system monitoring and data persistence.

## Structure
- `swift/`: Swift XPC Service.
  - Implements `ScreenCaptureKit` for screen recording.
  - Implements `EndpointSecurity` (`ESClient`) for process protection.
  - Implements `IOKit` observers for USB detection.
  - Implements `Vision` framework calls for OCR.
- `rust-core/`: Rust library.
  - Handles SQLite database operations.
  - Handles complex data filtering and audit logic.

## Logic Flow
1. Swift captures Screen/System Events.
2. Swift performs high-performance tasks (Vision OCR, Metal rendering).
3. Swift passes structured data/logs to Rust via FFI.
4. Rust encrypts and saves data to SQLite.
