enum PathListingOption {
    /// Attempt to use ls-files, fallback to seeking with non-hidden files
    case normal
    /// List all but hidden files.
    case hideHiddenFiles
    /// Show all files.
    case showHiddenFiles
}

struct Options {
    let root: String
    let listing: PathListingOption

    static func from(arguments args: [String]) -> Options {
        let arguments = args.dropFirst()
        var pathArgument: String? = nil
        var listing = PathListingOption.normal

        for argument in arguments {
            if argument.lowercased() == "-v" || argument.lowercased() == "--version" {
                showVersion()
            } else if argument.lowercased() == "-h" || argument.lowercased() == "--help" {
                showHelp()
            } else if argument.lowercased() == "-s" || argument.lowercased() == "--simple" {
                listing = .hideHiddenFiles
            } else if argument.lowercased() == "-a" || argument.lowercased() == "--all" {
                listing = .showHiddenFiles
            } else if !argument.hasPrefix("-") && pathArgument == nil {
                pathArgument = argument
            }
        }

        return Options.init(root: pathArgument ?? ".", listing: listing)
    }
}
