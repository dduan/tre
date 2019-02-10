import treCore
import Foundation

let version = "0.0.1"
let help = """
usage: tre [option] [path]

Print files, directories, and symlinks in tree form.
Hidden files and those configured to be ignored by git will be (optionally) ignored.

options:
  -a --all      Print all files and directories, including hidden ones.
  -s --simple   Use normal print despite gitignore settings. '-a' has higher priority.
  -h --help     Show this help message.
  -v --version  Show version.

Project home page: https://github.com/dduan/tre
"""

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
                print(version)
                exit(0)
            } else if argument.lowercased() == "-h" || argument.lowercased() == "--help" {
                print(help)
                exit(0)
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

func execute(with options: Options) throws {
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

try execute(with: Options.from(arguments: CommandLine.arguments))
