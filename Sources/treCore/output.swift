import Foundation
import Pathos
import Rainbow

extension FormattedLine {
    func render(atIndex index: Int, addAliasIndiactor: Bool) -> String {
        let aliasIndicator = addAliasIndiactor ? "\(index) ".red : ""
        return "\(self.treePrefix)\(aliasIndicator)\(self.fileName)"
    }
}

func output(_ formattedLines: [FormattedLine], createEditorAlias: Bool) {
    var aliases = [String]()
    var tree = [String]()

    for (index, formattedLine) in formattedLines.enumerated() {
        aliases.append("alias e\(index)=\"$EDITOR \\\"\(formattedLine.filePath)\\\"\"")
        tree.append(formattedLine.render(atIndex: index, addAliasIndiactor: createEditorAlias))
    }


    print(tree.joined(separator: "\n"))
    let username = ProcessInfo.processInfo.environment["USER"] ?? ""
    try? write(aliases.joined(separator: "\n"), atPath: "/tmp/tre_aliases_\(username)")
}

