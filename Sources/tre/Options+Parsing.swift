import treCore

extension Options {
    static func from(arguments args: [String]) -> Options {
        let arguments = args.dropFirst()
        var pathArgument: String? = nil
        var listing = PathListingOption.normal
        var editorAliases = false

        for argument in arguments {
            if argument.lowercased() == "-v" || argument.lowercased() == "--version" {
                showVersion()
            } else if argument.lowercased() == "-h" || argument.lowercased() == "--help" {
                showHelp()
            } else if argument.lowercased() == "-s" || argument.lowercased() == "--simple" {
                listing = .hideHiddenFiles
            } else if argument.lowercased() == "-a" || argument.lowercased() == "--all" {
                listing = .showHiddenFiles
            } else if argument.lowercased() == "-e" || argument.lowercased() == "--editor-aliases" {
                editorAliases = true
            } else if !argument.hasPrefix("-") && pathArgument == nil {
                pathArgument = argument
            }
        }

        return Options(root: pathArgument ?? ".", listing: listing, createEditorAliases: editorAliases)
    }
}
