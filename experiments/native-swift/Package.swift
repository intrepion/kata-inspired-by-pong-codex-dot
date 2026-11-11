// swift-tools-version: 5.9
import PackageDescription
let package = Package(name: "NativePong", products: [.library(name: "PongCore", targets: ["PongCore"])], targets: [.target(name: "PongCore"), .testTarget(name: "PongCoreTests", dependencies: ["PongCore"])])
