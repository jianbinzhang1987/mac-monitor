# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 🔨 Build & Run Commands

### Core Libraries (Rust)
This project relies on two Rust static libraries that must be built before the Swift/GUI components.

- **Build Network Core**:
  ```bash
  cd mac-monitor-project/network-extension/rust-core
  cargo build --release
  ```
  *Generates `libnetwork_protocol_stack.a`*

- **Build Audit Core**:
  ```bash
  cd mac-monitor-project/audit-service/rust-core
  cargo build --release
  ```
  *Generates `libaudit_logic_core.a`*

### GUI Application (Tauri)
- **Install Dependencies**: `cd mac-monitor-project/gui-app && pnpm install`
- **Run in Dev Mode**: `cd mac-monitor-project/gui-app && pnpm tauri dev`
- **Build Release**: `cd mac-monitor-project/gui-app && pnpm tauri build`

### Testing
- **Run Rust Tests**: `cargo test` (in specific `rust-core` directories)
- **Run GUI Tests**: `pnpm test` (in `gui-app`, if available)

## 🏗 Architecture Overview

This is a macOS terminal audit system using a **Hybrid Architecture**:

1.  **GUI App (Tauri v2 + Vue 3)**:
    - Located in `mac-monitor-project/gui-app`.
    - Handles user interaction, configuration, and log visualization.
    - Communicates with system extensions via XPC/IPC.

2.  **Network Extension (Swift + Rust)**:
    - Located in `mac-monitor-project/network-extension`.
    - **Swift**: Implements `PacketTunnelProvider` to interface with macOS NetworkExtension framework.
    - **Rust**: Handles the network protocol stack (smoltcp, rustls) and traffic interception/audit logic.
    - Runs as a system extension.

3.  **Audit Service (Swift + Rust)**:
    - Located in `mac-monitor-project/audit-service`.
    - **Swift**: Interfaces with `ScreenCaptureKit` (screen recording), `Vision` (OCR), and `EndpointSecurity` (process protection).
    - **Rust**: Handles data logic, filtering, and SQLite persistence.
    - Runs as a background XPC service/Daemon.

## ⚠️ Important Considerations
- **System Extensions**: Requires specific entitlements (`com.apple.developer.networking.networkextension`, `com.apple.developer.endpoint-security.client`).
- **SIP**: Development may require disabling System Integrity Protection (SIP) if correct signatures are not available.
- **FFI**: Rust code is exposed to Swift via C-compatible FFI. When modifying Rust structs used by Swift, ensure C header files or bindings are updated.

## 🧩 Code Style
- **Rust**: Standard `rustfmt`.
- **Swift**: Follow standard Swift guidelines.
- **TypeScript/Vue**: Follow project `eslint` / `prettier` configs.
