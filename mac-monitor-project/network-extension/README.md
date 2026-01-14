# Network Extension Module

This directory implements the macOS NetworkExtension (Packet Tunnel Provider).

## Structure
- `swift/`: Swift project (Xcode target), implements `NEPacketTunnelProvider`.
- `rust-core/`: Rust library, implements userspace TCP/IP stack (`smoltcp`) and SSL MitM logic.

## Build Flow
1. `rust-core` compiles to `libnetwork_stack.a` (Static Library).
2. `swift` target links against this static library.
3. Swift receives `IP Packets` from OS -> FFI -> Rust.
