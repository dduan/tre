import Pathos

public func execute(with options: Options) {
    let descendants: [(String, FileType)]

    switch options.listing {
    case .normal:
        var gitDescendants: [(String, FileType)]? = nil
        gitDescendants = gitFiles(inDirectory: options.root, gitArguments: [])
        descendants = gitDescendants ?? paths(inDirectory: options.root, includeHiddenFiles: false)
    case .hideHiddenFiles:
        descendants = paths(inDirectory: options.root, includeHiddenFiles: false)
    case .showHiddenFiles:
        descendants = paths(inDirectory: options.root, includeHiddenFiles: true)
    }

    output(format(root: options.root, input: descendants), createEditorAlias: options.createEditorAliases)
}
