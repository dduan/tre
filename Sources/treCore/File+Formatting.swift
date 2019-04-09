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
        while let ancestor = current.parent {
            let count = printHistory[ObjectIdentifier(ancestor)] ?? 0
            if let directParent = self.parent, directParent === ancestor {
                if count == ancestor.children.count - 1 {
                    segments.append("└── ")
                } else {
                    segments.append("├── ")
                }
            } else {
                if count == ancestor.children.count {
                    segments.append("    ")
                } else {
                    segments.append("│   ")
                }
            }

            current = ancestor
        }

        return segments.reversed().joined(separator: "")
    }

    func format(inContext printHistory: inout [ObjectIdentifier: Int], into result: inout [FormattedLine]) {
        let treePrefix = self.treePrefix(inContext: printHistory)
        let line = FormattedLine(
            treePrefix: treePrefix,
            fileName: self.nameText,
            filePath: self.fullPath)
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

func collectDirectoryInfo(root: String = ".", input: [String]) -> File {
    var directory = File(fullPath: root, name: root, type: .directory)

    func splitFile(path: String) -> (String, File) {
        let (prefix, name) = Pathos.split(path: path)
        let parents = prefix
        let node: File
        if (try? isA(.directory, atPath: path)) ?? false {
            node = File(fullPath: path, name: name, type: .directory)
        } else if (try? isA(.symbolicLink, atPath: path)) ?? false {
            node = File(fullPath: path, name: name, type: .link, link: (try? readSymbolicLink(atPath: path)) ?? "?")
        } else {
            node = File(fullPath: path, name: name, type: .other)
        }

        return (parents, node)
    }

    for path in input {
        if isAbsolute(path: path) || !exists(atPath: path) || path == "." {
            continue
        }

        let (fullAncestry, node) = splitFile(path: normalize(path: path))
        let ancestry = fullAncestry.dropFirst(commonPath(amongPaths: root, fullAncestry).count)
        let ancestrySegments = ancestry
            .split(separator: pathSeparatorCharacter)
            .map(String.init)
        directory.insert(node, fullPath: path, ancestry: ancestrySegments)
    }

    return directory
}

func format(root: String = ".", input: [String]) -> [FormattedLine] {
    let result = collectDirectoryInfo(root: root, input: input)
    var context = [ObjectIdentifier: Int]()
    var output = [FormattedLine]()
    result.format(inContext: &context, into: &output)
    return output
}
