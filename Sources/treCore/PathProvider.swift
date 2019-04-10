import Pathos
import Foundation

func paths(inDirectory root: String, includeHiddenFiles: Bool) -> [(String, FileType)] {
    var remains = [root]
    var results = [(String, FileType)]()

    func ignoreHidden(_ path: String) -> Bool {
        return includeHiddenFiles || !split(path: path).1.hasPrefix(".")
    }

    while !remains.isEmpty {
        let path = remains.removeFirst()
        let allChildren = ((try? children(inPath: path)) ?? [])
            .filter { ignoreHidden($0.0) }

        results += allChildren
        remains += allChildren
            .filter { $1 == .directory }
            .map { $0.0 }
    }

    return results
}

func gitFiles(inDirectory root: String, gitArguments: [String]) -> [(String, FileType)]? {
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
        let paths = output?.split(separator: "\n").map(String.init) ?? []
        return paths.map { path in
            if (try? isA(.directory, atPath: path)) ?? false {
                return (path, .directory)
            } else if (try? isA(.symbolicLink, atPath: path)) ?? false {
                return (path, .symbolicLink)
            } else {
                return (path, .file)
            }
        }
    } else {
        return nil
    }
}
