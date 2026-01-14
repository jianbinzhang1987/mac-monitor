# GUI Client Engineering

This directory contains the user interface application using Tauri v2.

## Structure
- `src/`: Vue 3 Frontend code (Ant Design Vue)
- `src-tauri/`: Rust Sidecar logic, responsible for communicating with the Background Audit Service via XPC/IPC.

## Responsibilities
- User Login/Authentication
- Configuration Management (Rules, Policies)
- Log Visualization (Real-time traffic audit logs)
- Alert Push Notifications
