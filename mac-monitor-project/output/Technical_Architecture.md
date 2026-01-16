# Mac Monitor System - Technical Documentation

## 1. System Architecture

### 1.1 Overview
The system adopts a hybrid architecture combining a native macOS Network Extension for traffic interception (TUN mode) and a user-space HTTP Proxy for lightweight auditing.

### 1.2 Components
- **GUI App (Tauri)**: Provides user interface and manages background processes.
- **Traffic Proxy (Rust)**:
  - **Role**: Independent Sidecar.
  - **Function**: Handles HTTP/HTTPS traffic forwarding, MITM (Man-in-the-Middle) decryption, and audit logging.
  - **Path**: `bin/traffic-proxy`
- **VPN Helper (Swift)**:
  - **Role**: Privileged helper tool.
  - **Function**: Manages system proxy settings (`networksetup`), trusts CA certificates (`security`), and controls the proxy process.
  - **Path**: `bin/vpn-helper`
- **Network Extension**:
  - **Role**: System Extension (Packet Tunnel).
  - **Function**: Global traffic takeover (when signed with Entitlements).

## 2. HTTP Proxy Mode (New)

### 2.1 Workflow
1. **Enable**: User clicks "Enable Proxy" -> VPN Helper starts `traffic-proxy` -> VPN Helper trusts Root CA -> VPN Helper sets Wi-Fi Proxy to 127.0.0.1:8050.
2. **Intercept**: Browser sends HTTP CONNECT -> Traffic Proxy establishes TLS tunnel -> Generates dynamic cert signed by Root CA -> Decrypts request -> Logs to IPC -> Forwards to Upstream.
3. **Disable**: User clicks "Disable" -> VPN Helper unsets Wi-Fi Proxy -> VPN Helper kills `traffic-proxy`.

### 2.2 Security
- **CA Certificate**: Generated locally using `rcgen`. Private key (`ca.key`) is persisted to ensure stability.
- **Validity**: Root CA is valid from 2025 to 4096.
- **Trust**: Added to System Keychain via `security add-trusted-cert`.

### 2.3 Automation
- **Privileged Helper**: The `vpn-helper` tool encapsulates all privileged operations (`sudo`), providing a single command for the GUI to invoke.
- **Auto-Start**: Integrated `tauri-plugin-autostart` to register the application as a Launch Agent for system login startup.
- **Auto-Proxy**: The GUI application automatically invokes `enable_proxy` on startup via `osascript`, ensuring traffic auditing begins immediately.

## 3. Build & Deploy
- **Build Script**: `build_all.sh` handles the compilation of all components.
- **Artifacts**:
  - `mac-monitor-gui.app`: Main bundle.
  - `traffic-proxy`: Embedded in `MsacOS/bin/`.
  - `vpn-helper`: Embedded in `MacOS/bin/`.

