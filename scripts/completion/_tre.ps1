
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'tre' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'tre'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'tre' {
            [CompletionResult]::new('-e', 'e', [CompletionResultType]::ParameterName, 'Create aliases for each displayed result, and add a number in front of file name to indicate the alias name. For example, a number "42" means an shell alias "e42" has been created. Running "e42" will cause the associated file or directory to be open with $EDITOR (or a default program for the file type on Windows), or a command specified along with this command')
            [CompletionResult]::new('--editor', 'editor', [CompletionResultType]::ParameterName, 'Create aliases for each displayed result, and add a number in front of file name to indicate the alias name. For example, a number "42" means an shell alias "e42" has been created. Running "e42" will cause the associated file or directory to be open with $EDITOR (or a default program for the file type on Windows), or a command specified along with this command')
            [CompletionResult]::new('-l', 'l', [CompletionResultType]::ParameterName, 'Limit depth of the tree in output')
            [CompletionResult]::new('--limit', 'limit', [CompletionResultType]::ParameterName, 'Limit depth of the tree in output')
            [CompletionResult]::new('-E', 'E', [CompletionResultType]::ParameterName, 'Exclude paths matching a regex pattern. Repeatable')
            [CompletionResult]::new('--exclude', 'exclude', [CompletionResultType]::ParameterName, 'Exclude paths matching a regex pattern. Repeatable')
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'When to color the output. `automatic` means when printing to a terminal, tre will include colors; otherwise it will disable colors')
            [CompletionResult]::new('--color', 'color', [CompletionResultType]::ParameterName, 'When to color the output. `automatic` means when printing to a terminal, tre will include colors; otherwise it will disable colors')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-a', 'a', [CompletionResultType]::ParameterName, 'Print all files and directories, including hidden ones')
            [CompletionResult]::new('--all', 'all', [CompletionResultType]::ParameterName, 'Print all files and directories, including hidden ones')
            [CompletionResult]::new('-s', 's', [CompletionResultType]::ParameterName, 'Use normal print despite gitignore settings. ''-a'' has higher priority')
            [CompletionResult]::new('--simple', 'simple', [CompletionResultType]::ParameterName, 'Use normal print despite gitignore settings. ''-a'' has higher priority')
            [CompletionResult]::new('-d', 'd', [CompletionResultType]::ParameterName, 'Only list directories in output')
            [CompletionResult]::new('--directories', 'directories', [CompletionResultType]::ParameterName, 'Only list directories in output')
            [CompletionResult]::new('-j', 'j', [CompletionResultType]::ParameterName, 'Output JSON instead of tree diagram')
            [CompletionResult]::new('--json', 'json', [CompletionResultType]::ParameterName, 'Output JSON instead of tree diagram')
            [CompletionResult]::new('-p', 'p', [CompletionResultType]::ParameterName, 'Generate portable (absolute) paths for editor aliases. By default, aliases use relative paths for better performance')
            [CompletionResult]::new('--portable', 'portable', [CompletionResultType]::ParameterName, 'Generate portable (absolute) paths for editor aliases. By default, aliases use relative paths for better performance')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
