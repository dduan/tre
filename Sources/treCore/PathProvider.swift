import Pathos
import Foundation

func paths(inDirectory root: String, includeHiddenFiles: Bool) -> [String] {
    var remains = [root]
    var results = [String]()

    func ignoreHidden(_ path: String) -> Bool {
        return includeHiddenFiles || !split(path: path).1.hasPrefix(".")
    }

    while !remains.isEmpty {
        let path = remains.removeFirst()
        let childDirectories = (try? children(inPath: path, ofType: .directory))?.filter(ignoreHidden) ?? []
        let childFiles = (try? children(inPath: path, ofType: .file))?.filter(ignoreHidden) ?? []
        let childSymbols = (try? children(inPath: path, ofType: .symbolicLink))?.filter(ignoreHidden) ?? []

        results += childDirectories + childFiles + childSymbols
        remains += childDirectories
    }

    return results.sorted()
}

func gitFiles(inDirectory root: String, gitArguments: [String]) -> [String]? {
    let task = Process()
#if os(macOS)
    task.launchPath = "/usr/bin/env"
#elseif swift(>=5.0)
    task.executableURL = URL(fileURLWithPath: "/usr/bin/env")
#else
    task.launchPath = "/usr/bin/env"
#endif
    task.arguments = ["git", "ls-files", root]

    let pipe = Pipe()
    let errorPipe = Pipe()
    task.standardOutput = pipe
    task.standardError = errorPipe
#if os(macOS)
    task.launch()
#elseif swift(>=5.0)
    try? task.run()
#else
    task.launch()
#endif

    let data = pipe.fileHandleForReading.readDataToEndOfFile()
    task.waitUntilExit()

    if task.terminationStatus == 0 {
        let output = String(data: data, encoding: .utf8)
        return output?.split(separator: "\n").map(String.init) ?? []
    } else {
        return nil
    }
}
