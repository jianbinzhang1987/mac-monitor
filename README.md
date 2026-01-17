# Internet Terminal Audit System / 互联网终端上网审计系统

[English](#english) | [中文](#chinese)

---

<a name="english"></a>
## 🇬🇧 English

### Project Overview

**Mac Monitor System** is a comprehensive endpoint audit and monitoring solution designed for enterprise environments. It consists of a high-performance native client built with Tauri/Rust and a robust backend management system based on the RuoYi framework.

The system provides deep visibility and control over macOS endpoints, enabling enterprise-grade network traffic auditing, intelligent screen recording (DLP), and system security monitoring.

### Key Components

1.  **Mac Client (`mac-monitor-project`)**
    *   **Next-Gen Architecture**: Built with Tauri v2 and native system frameworks (NetworkExtension, ScreenCaptureKit, EndpointSecurity), strictly avoiding legacy "magic modification" approaches.
    *   **High Performance**: Designed for minimal resource usage (&lt;1% CPU).
    *   **Deep Auditing**: Features kernel-level traffic interception (including SSL/TLS), real-time on-device OCR for privacy protection, and peripheral monitoring.

2.  **Management Server (`mac-monitor-server`)**
    *   **Framework**: Based on [RuoYi v3.9.1](https://gitee.com/y_project/RuoYi-Vue).
    *   **Tech Stack**: Spring Boot + Vue (Element UI).
    *   **Functionality**: A centralized dashboard for administrators to view audit logs (traffic, screen, behavior), manage policies, and oversee system status.

### 🚀 Key Features

*   **Network Traffic Audit**:
    *   Global traffic takeover via `NetworkExtension` (TUN mode).
    *   Rust-based MitM proxy to inspect HTTPS traffic header and body.
    *   Flexible proxy modes for various network environments.
*   **Intelligent Screen Monitoring (DLP)**:
    *   Zero-latency recording using Apple's `ScreenCaptureKit`.
    *   **Privacy-First**: Integrated `Vision` framework for real-time OCR to automatically mask sensitive data (passwords, banking info) before storage.
*   **System Security**:
    *   Self-protection mechanism via `Endpoint Security Framework` to prevent tampering.
    *   Peripheral (USB) and clipboard usage monitoring.
*   **Central Management**:
    *   Comprehensive dashboards for visualized data analysis.
    *   Role-based access control (RBAC) and department management.

### 🛠 Technology Stack

#### Client Side
*   **Core Logic**: Rust (Performance & Safety).
*   **System Glue**: Swift (NetworkExtension, XPC Services).
*   **GUI**: Tauri v2, Vue 3, Ant Design x Vue, TypeScript.
*   **macOS APIs**: NetworkExtension, ScreenCaptureKit, EndpointSecurity, Vision.

#### Server Side
*   **Backend**: Java 8/17, Spring Boot, Spring Security, Redis, MySQL.
*   **Frontend**: Vue 2, Element UI.

### 🏁 Getting Started

#### Prerequisites
*   **macOS**: Sonoma (14.0) or later.
*   **Java**: JDK 17 (recommended).
*   **Node.js**: v18+.
*   **Rust**: Latest stable version.
*   **Xcode**: 15+ (Required for building system extensions).

#### Installation & Running

**1. Start the Server**
```bash
cd mac-monitor-server
./start_server.sh
```
This script starts both the Spring Boot backend and the RuoYi frontend development server.

**2. Start the Client**
```bash
cd mac-monitor-project
./start_client_full.sh
```
This script acts as an all-in-one entry point to build and run the client components (Audit Service, GUI, etc.).

> **Note**: For pure GUI development without system extensions, you can run `npm run tauri dev` inside `mac-monitor-project/gui-app`.

---

<a name="chinese"></a>
## 🇨🇳 中文

### 项目简介

**互联网终端上网审计系统 (Mac Monitor System)** 是一套专为企业环境设计的综合终端审计与监控解决方案。系统由高性能的原生 macOS 客户端（基于 Tauri/Rust）和成熟的后台管理系统（基于 RuoYi 框架）组成。

本系统旨在为企业提供对 macOS 终端的深度可视化与管控能力，实现了企业级的网络流量审计、智能屏幕录制（DLP）以及系统安全监控。

### 核心组件

1.  **Mac 客户端 (`mac-monitor-project`)**
    *   **下一代架构**: 基于 Tauri v2 和原生系统框架 (NetworkExtension, ScreenCaptureKit, EndpointSecurity) 构建，彻底摒弃传统的"魔改"方案。
    *   **高性能**: 极致优化，CPU 占用率极低 (<1%)。
    *   **深度审计**: 具备内核级流量拦截（含 SSL/TLS 解密）、端侧实时 OCR 隐私保护以及外设监控能力。

2.  **管理服务端 (`mac-monitor-server`)**
    *   **框架**: 基于 [RuoYi v3.9.1](https://gitee.com/y_project/RuoYi-Vue) 开发。
    *   **技术栈**: Spring Boot + Vue (Element UI)。
    *   **功能**: 提供集中式管理面板，用于查看审计日志（流量、屏幕、行为）、管理策略以及系统状态监控。

### 🚀 核心功能

*   **网络流量审计**:
    *   通过 `NetworkExtension` (TUN 模式) 实现全局流量接管。
    *   基于 Rust 的 MitM 代理，支持 HTTPS 流量（头部及内容）的解密与分析。
    *   支持多种代理模式，适应复杂的网络环境。
*   **智能屏幕监控 (DLP)**:
    *   使用 Apple `ScreenCaptureKit` 实现零延迟屏幕录制。
    *   **隐私优先**: 集成 `Vision` 框架进行实时 OCR，自动对敏感数据（密码、银行信息等）进行打码遮罩，从源头保护隐私。
*   **系统安全**:
    *   利用 `Endpoint Security Framework` 实现自我保护，防止客户端被恶意篡改或关闭。
    *   监控外设（USB）插入及剪贴板使用情况。
*   **集中管理**:
    *   提供丰富的数据可视化仪表盘。
    *   支持基于角色的访问控制 (RBAC) 和部门层级管理。

### 🛠 技术栈

#### 客户端 (Client)
*   **核心逻辑**: Rust (高性能与内存安全)。
*   **系统交互**: Swift (NetworkExtension, XPC 服务)。
*   **用户界面**: Tauri v2, Vue 3, Ant Design x Vue, TypeScript。
*   **macOS API**: NetworkExtension, ScreenCaptureKit, EndpointSecurity, Vision。

#### 服务端 (Server)
*   **后端**: Java 8/17, Spring Boot, Spring Security, Redis, MySQL。
*   **前端**: Vue 2, Element UI。

### 🏁 快速开始

#### 环境要求
*   **macOS**: Sonoma (14.0) 或更高版本。
*   **Java**: JDK 17 (推荐)。
*   **Node.js**: v18+。
*   **Rust**: 最新稳定版。
*   **Xcode**: 15+ (编译系统扩展必须)。

#### 安装与运行

**1. 启动服务端**
```bash
cd mac-monitor-server
./start_server.sh
```
该脚本将启动 Spring Boot 后端服务以及 RuoYi 前端开发服务器。

**2. 启动客户端**
```bash
cd mac-monitor-project
./start_client_full.sh
```
该脚本是一键式启动入口，负责编译并运行客户端的各个组件（审计服务、GUI 等）。

> **注意**: 如果仅需进行 GUI 界面开发而无需系统扩展功能，可以在 `mac-monitor-project/gui-app` 目录下直接运行 `npm run tauri dev`。
