# Internet Terminal Audit System - Mac Client

> **Note**: This project is a next-generation macOS endpoint audit client, rebuilt using native system frameworks (NetworkExtension, ScreenCaptureKit, EndpointSecurity) and Tauri v2, abandoning the legacy Clash-based "magic modification" approach for higher performance and stability.

## 📖 Project Overview

This system is designed for enterprise environments to provide comprehensive network traffic auditing, behavior monitoring, and data loss prevention (DLP) capabilities on macOS endpoints.

It adopts a **Separated Architecture**:
1.  **GUI Interface**: Lightweight visualization and interaction.
2.  **Network Engine**: Kernel-level traffic interception and SSL audit.
3.  **Audit Service**: High-performance background monitoring (Screen, OCR, Peripherals).

## 🏗 Architecture

```mermaid
graph TD
    subgraph "Main App (GUI)"
        Tauri[Tauri v2 (Rust + Vue 3)]
    end

    subgraph "Network Extension (System)"
        NE[NetworkExtension (Swift + Rust)]
        NE_MitM[SSL MitM Proxy]
    end

    subgraph "Audit Service (Daemon)"
        SCK[ScreenCaptureKit]
        Vision[OCR & Privacy Masking]
        ES[Endpoint Security]
    end

    Tauri -- IPC/XPC --> NE
    Tauri -- IPC/XPC --> Audit Service
```

## 📂 Directory Structure

| Directory | Module Name | Description | Tech Stack |
| :--- | :--- | :--- | :--- |
| **`gui-app/`** | GUI Client | User Interface, Login, config management. | **Tauri v2**, Vue 3, Ant Design |
| **`network-extension/`** | Network Core | VPN Packet Tunnel, Traffic interception, SSL Audit. | **Swift** (Shell), **Rust** (Core) |
| **`audit-service/`** | Logic Core | Screen recording, OCR, File/Process protection. | **Swift** (XPC), Rust (DB/Logic) |

## ✨ Key Features

### 1. Network Traffic Audit
- **Global Takeover**: Uses `NetworkExtension` (TUN mode) to capture all system traffic, stable and unkillable.
- **SSL/TLS Deep Inspection**: Rust-based MitM proxy to decrypt and analyze HTTPS traffic (Headers, Body) in memory.
- **Policy Enforcement**: Domain filtering, sensitive keyword blocking.

### 2. Intelligent Screen Monitoring (DLP)
- **Zero-Latency Capture**: Utilizes `ScreenCaptureKit` for high-performance recording with <1% CPU usage.
- **Privacy First**: Integrated Apple `Vision` framework for real-time OCR. Automatically blurs sensitive data (passwords, bank info) **before** saving.
- **App-Specific Logic**: Customized capture frequencies for social apps (WhatsApp, Telegram) vs normal browsers.

### 3. System Security & Antagonism
- **Self-Protection**: Uses `Endpoint Security Framework` to prevent the client processes/files from being killed or modified by users.
- **Peripheral Audit**: Monitors USB device insertion (esp. Network adapters, storage).
- **Clipboard Audit**: Monitors clipboard content specifically during browser activity.

## 🚀 Getting Started

### Prerequisites
- **macOS**: Sonoma (14.0) or later (required for ScreenCaptureKit/ESF).
- **Development Tools**:
  - Xcode 15+ (for Swift/System Extensions)
  - Rust (latest stable)
  - Node.js (v18+)

### Running the GUI (Development)

The GUI application is the entry point for development.

```bash
cd gui-app
npm install
npm run tauri dev
```

> **Note**: The GUI depends on the background `audit-service` and `network-extension`. In a full deployment, these are built as System Extensions and XPC Services embedded within the main bundle. For pure UI development, the GUI runs in standalone mode with mock data if services are unavailable.

## 🛠 Technology Stack

- **Languages**: Rust (Performance Critical), Swift (System Glue), TypeScript (UI).
- **Frameworks**: Tauri v2, Vue 3, Ant Design Vue.
- **System APIs**: NetworkExtension, ScreenCaptureKit, EndpointSecurity, Vision, IOKit.
