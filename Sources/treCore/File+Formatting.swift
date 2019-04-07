import Pathos
import Rainbow

extension File {
    func treeText(inContext printHistory: [ObjectIdentifier: Int]) -> String {
        let nameText: String

        switch self.type {
        case .directory:
            nameText = self.name.blue
        case .link:
            nameText = "\(self.name.magenta) -> \(self.link ?? "")"
        case .other:
            nameText = self.name
        }

        var segments = [nameText]
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

    func print(inContext printHistory: inout [ObjectIdentifier: Int], into result: inout [String]) {
        result.append(self.treeText(inContext: printHistory))
        if let parent = self.parent {
            printHistory[ObjectIdentifier(parent)]? += 1
        }

        if case .directory = self.type {
            printHistory[ObjectIdentifier(self)] = 0
        }

        for child in self.children.values {
            child.print(inContext: &printHistory, into: &result)
        }
    }
}

func collectDirectoryInfo(root: String = ".", input: [String]) -> File {
    var directory = File(name: root, type: .directory)

    func splitFile(path: String) -> (String, File) {
        let (prefix, name) = Pathos.split(path: path)
        let parents = prefix
        let node: File
        if (try? isDirectory(atPath: path)) ?? false {
            node = .init(name: name, type: .directory)
        } else if (try? isSymbolicLink(atPath: path)) ?? false {
            node = .init(name: name, type: .link, link: (try? readSymbolicLink(atPath: path)) ?? "?")
        } else {
            node = .init(name: name, type: .other)
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
        directory.insert(node, ancestry: ancestrySegments)
    }

    return directory
}

func format(root: String = ".", input: [String]) -> String {
    let result = collectDirectoryInfo(root: root, input: input)
    var context = [ObjectIdentifier: Int]()
    var output = [String]()
    result.print(inContext: &context, into: &output)
    return output.joined(separator: "\n")
}
