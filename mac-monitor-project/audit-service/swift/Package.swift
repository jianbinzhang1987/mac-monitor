// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "AuditService",
    platforms: [
        .macOS(.v14)
    ],
    products: [
        .executable(name: "AuditService", targets: ["AuditService"])
    ],
    targets: [
        .executableTarget(
            name: "AuditService",
            path: "Sources/AuditService",
            linkerSettings: [
                .unsafeFlags(["-L../rust-core/target/release", "-laudit_logic_core"]),
                .linkedLibrary("sqlite3"),
                .linkedFramework("CoreFoundation"),
                .linkedFramework("Security"),
                .linkedFramework("SystemConfiguration")
            ]
        )
    ]
)
