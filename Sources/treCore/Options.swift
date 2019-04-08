public enum PathListingOption {
    /// Attempt to use ls-files, fallback to seeking with non-hidden files
    case normal
    /// List all but hidden files.
    case hideHiddenFiles
    /// Show all files.
    case showHiddenFiles
}

public struct Options {
    let root: String
    let listing: PathListingOption
    let createEditorAliases: Bool

    public init(root: String, listing: PathListingOption, createEditorAliases: Bool) {
        self.root = root
        self.listing = listing
        self.createEditorAliases = createEditorAliases
    }
}
