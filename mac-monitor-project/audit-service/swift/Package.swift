// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "AuditService",
    platforms: [
        .macOS(.v13)
    ],
    products: [
        .executable(name: "AuditService", targets: ["AuditService"])
    ],
    targets: [
        .executableTarget(
            name: "AuditService",
            path: "Sources/AuditService",
            linkerSettings: []
        )
    ]
)
