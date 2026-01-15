# Repository Guidelines

## Project Structure & Module Organization
- `mac-monitor-project/`: macOS client stack (Tauri GUI + Swift/Rust system extensions).
  - `gui-app/`: Vue 3 + Tauri v2 UI.
  - `network-extension/`: Packet Tunnel (Swift) with Rust core in `network-extension/rust-core/`.
  - `audit-service/`: XPC service (Swift) with Rust core in `audit-service/rust-core/`.
- `mac-monitor-server/`: RuoYi-based backend (Spring Boot) and admin UI.
  - `ruoyi-admin/` and related `ruoyi-*` modules for Java services.
  - `ruoyi-ui/`: Vue 2 + Element UI admin frontend.
- `output/`, `文档资料/`, `logs/`: design docs and generated artifacts; treat as references.

## Build, Test, and Development Commands
- Rust cores: `cargo build --release` in `mac-monitor-project/network-extension/rust-core` and `mac-monitor-project/audit-service/rust-core`.
- Tauri GUI: `pnpm install` then `pnpm tauri dev` in `mac-monitor-project/gui-app`.
- Server backend: `mvn clean install` at `mac-monitor-server/`, `mvn spring-boot:run` in `mac-monitor-server/ruoyi-admin`.
- Server UI: `npm install` then `npm run dev` in `mac-monitor-server/ruoyi-ui`.
- Database init: `mysql -u root -p db < mac-monitor-server/sql/ry_20250522.sql` (plus `quartz.sql` if needed).

## Coding Style & Naming Conventions
- Rust: format with `rustfmt`; keep FFI structs stable and update headers if bindings change.
- Swift: follow standard Swift naming and file organization per target.
- TypeScript/Vue: use PascalCase for components (e.g., `DeviceList.vue`), kebab-case routes.
- Java: follow existing RuoYi patterns (Controller/Service/Mapper) and keep new SQL in module `resources/mapper/`.

## Testing Guidelines
- Rust: `cargo test` in each `rust-core` crate.
- Java: `mvn test` per module if tests are added.
- UI: no test scripts are defined; validate manually and keep screenshots for visible UI changes.

## Commit & Pull Request Guidelines
- Use Conventional Commits style seen in history (e.g., `feat: add ...`, `fix: ...`).
- PRs should include a clear summary, affected modules, and any required setup steps; attach screenshots for UI changes.

## Security & Configuration Tips
- macOS system extensions require proper entitlements (`networkextension`, `endpoint-security`) and signing; SIP restrictions may apply.
- Backend config lives in `mac-monitor-server/ruoyi-admin/src/main/resources/application.yml`.
