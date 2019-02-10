// swift-tools-version:4.2

import PackageDescription

let package = Package(
    name: "tre",
    products: [
        .executable(name: "tre", targets: ["tre"]),
    ],
    dependencies: [
        .package(url: "https://github.com/onevcat/Rainbow", from: "3.0.0"),
        .package(url: "https://github.com/dduan/Pathos.git", .branch("master")),
    ],
    targets: [
        .target(name: "treCore", dependencies: ["Pathos", "Rainbow"]),
        .target(name: "tre", dependencies: ["treCore"]),
    ]
)
