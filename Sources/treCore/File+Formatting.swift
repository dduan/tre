import Pathos
import Rainbow

extension File {
    var nameText: String {
        switch self.type {
        case .directory:
            return self.name.blue
        case .link:
            return "\(self.name.magenta) -> \(self.link ?? "")"
        case .other:
            return self.name
        }
    }

    func treePrefix(inContext printHistory: [ObjectIdentifier: Int]) -> String {
        var segments = [String]()
        var current = self
        if let ancestor = current.parent {
            if let directParent = self.parent, directParent === ancestor {
                let count = printHistory[ObjectIdentifier(ancestor)] ?? 0
                if count == ancestor.children.count - 1 {
                    segments.append("└── ")
                } else {
                    segments.append("├── ")
                }
            }

            current = ancestor
        }

        while let ancestor = current.parent {
            let count = printHistory[ObjectIdentifier(ancestor)] ?? 0
            if count == ancestor.children.count {
                segments.append("    ")
            } else {
                segments.append("│   ")
            }

            current = ancestor
        }

        return segments.reversed().joined(separator: "")
    }

    func format(inContext printHistory: inout [ObjectIdentifier: Int], into result: inout [FormattedLine]) {
        let treePrefix = self.treePrefix(inContext: printHistory)
        let line = FormattedLine(treePrefix: treePrefix, fileName: self.nameText, filePath: self.fullPath)
        result.append(line)

        if let parent = self.parent {
            printHistory[ObjectIdentifier(parent)]? += 1
        }

        if case .directory = self.type {
            printHistory[ObjectIdentifier(self)] = 0
        }

        for child in self.children.values {
            child.format(inContext: &printHistory, into: &result)
        }
    }
}

func collectDirectoryInfo(root: String = ".", input: [(String, FileType)]) -> File {
    let directory = File(fullPath: root, name: root, type: .directory)
    let rootSegmentCount = root
        .split(separator: pathSeparatorCharacter)
        .filter { $0 != "." }
        .count

    for (path, type) in input {
        if isAbsolute(path: path) || path == "." {
            continue
        }

        let allSegments = path
            .split(separator: pathSeparatorCharacter)
            .filter { $0 != "." }
            .dropFirst(rootSegmentCount)
            .map(String.init)

        let name = allSegments.last ?? ""
        let ancestrySegments = allSegments.dropLast()

        let node: File
        switch type {
        case .directory:
            node = File(fullPath: path, name: name, type: .directory)
        case .symbolicLink:
            node = File(fullPath: path, name: name, type: .link,
                        link: (try? readSymbolicLink(atPath: path)) ?? "?")
        default:
            node = File(fullPath: path, name: name, type: .other)
        }

        directory.insert(node, fullPath: path, ancestry: ancestrySegments)
    }

    return directory
}

func format(root: String = ".", input: [(String, FileType)]) -> [FormattedLine] {
    let result = collectDirectoryInfo(root: root, input: input)
    var context = [ObjectIdentifier: Int]()
    var output = [FormattedLine]()
    result.format(inContext: &context, into: &output)
    return output
}
