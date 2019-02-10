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

    init(name: String, type: FileType, link: String? = nil) {
        self.name = name
        self.type = type
        self.link = link
    }

    func insert(_ node: File, ancestry: [String]) {
        var current = self
        for ancestorName in ancestry {
            if let nextAncestor = current.children[ancestorName] {
                current = nextAncestor
            } else {
                let newAncestor = File(name: ancestorName, type: .directory)
                newAncestor.parent = current
                current.children[ancestorName] = newAncestor
                current = newAncestor
            }
        }

        node.parent = current
        current.children[node.name] = node
    }
}
