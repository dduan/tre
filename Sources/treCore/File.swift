import Pathos

final class File {
    enum FileType {
        case directory
        case link
        case other
    }

    var children: [String: File] = [:]
    var type: FileType = .other
    var name: String
    var link: String?
    var parent: File?
    let fullPath: String

    init(fullPath: String, name: String, type: FileType, link: String? = nil) {
        self.fullPath = fullPath
        self.name = name
        self.type = type
        self.link = link
    }

    func insert(_ node: File, fullPath: String, ancestry: ArraySlice<String>) {
        var current = self

        for (i, ancestorName) in ancestry.enumerated() {
            if let nextAncestor = current.children[ancestorName] {
                current = nextAncestor
            } else {
                let path = ancestry[0...i].joined(separator: pathSeparator)
                let newAncestor = File(fullPath: path, name: ancestorName, type: .directory)
                newAncestor.parent = current
                current.children[ancestorName] = newAncestor
                current = newAncestor
            }
        }

        node.parent = current
        current.children[node.name] = node
    }
}
