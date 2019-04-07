// swift-tools-version:5.0

import PackageDescription

let package = Package(
    name: "tre",
    products: [
        .executable(name: "tre", targets: ["tre"]),
    ],
    dependencies: [
        .package(url: "https://github.com/onevcat/Rainbow", from: "3.0.0"),
        .package(url: "https://github.com/dduan/Pathos.git", from: "0.1.3"),
    ],
    targets: [
        .target(name: "treCore", dependencies: ["Pathos", "Rainbow"]),
        .target(name: "tre", dependencies: ["treCore"]),
    ]
)
