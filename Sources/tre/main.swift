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

let arguments = CommandLine.arguments.dropFirst()
var pathArgument: String? = nil
var showAll = false
var simple = false

for argument in arguments {
    if argument.lowercased() == "-v" || argument.lowercased() == "--version" {
        print(version)
        exit(0)
    } else if argument.lowercased() == "-h" || argument.lowercased() == "--help" {
        print(help)
        exit(0)
    } else if argument.lowercased() == "-s" || argument.lowercased() == "--simple" {
        simple = true
    } else if argument.lowercased() == "-a" || argument.lowercased() == "--all" {
        showAll = true
    } else if !argument.hasPrefix("-") && pathArgument == nil {
        pathArgument = argument
    }
}

let root = pathArgument ?? "."
var gitDescendants: [String]? = nil

if !showAll && !simple {
    gitDescendants = gitFiles(inDirectory: root, gitArguments: [])
}

let descendants = try gitDescendants ?? paths(inDirectory: root, includeHiddenFiles: showAll)
print(try format(root: root, input: descendants))
