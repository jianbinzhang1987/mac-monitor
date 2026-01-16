import Foundation

let args = CommandLine.arguments
if args.contains("--start") {
    print("Starting VPN...")
    VPNManager.shared.startVPN()
} else if args.contains("--stop") {
    print("Stopping VPN...")
    VPNManager.shared.stopVPN()
} else if args.contains("--status") {
    VPNManager.shared.printStatus()
} else if args.contains("--enable-proxy") {
    ProxyManager.shared.enableSystemProxy()
} else if args.contains("--disable-proxy") {
    ProxyManager.shared.disableSystemProxy()
} else {
    print("Usage: vpn-helper [--start | --stop | --status | --enable-proxy | --disable-proxy]")
}

// Wait strictly for asynchronous operations to complete
RunLoop.main.run(until: Date(timeIntervalSinceNow: 15))
