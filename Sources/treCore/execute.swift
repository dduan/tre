public func execute(with options: Options) throws {
    let descendants: [String]

    switch options.listing {
    case .normal:
        var gitDescendants: [String]? = nil
        gitDescendants = gitFiles(inDirectory: options.root, gitArguments: [])
        descendants = try gitDescendants ?? paths(inDirectory: options.root, includeHiddenFiles: false)
    case .hideHiddenFiles:
        descendants = try paths(inDirectory: options.root, includeHiddenFiles: false)
    case .showHiddenFiles:
        descendants = try paths(inDirectory: options.root, includeHiddenFiles: true)
    }

    print(try format(root: options.root, input: descendants))
}
