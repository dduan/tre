complete -c tre -s e -l editor -d 'Create aliases for each displayed result, and add a number in front of file name to indicate the alias name. For example, a number "42" means an shell alias "e42" has been created. Running "e42" will cause the associated file or directory to be open with $EDITOR (or a default program for the file type on Windows), or a command specified along with this command' -r
complete -c tre -s l -l limit -d 'Limit depth of the tree in output' -r
complete -c tre -s E -l exclude -d 'Exclude paths matching a regex pattern. Repeatable' -r
complete -c tre -s c -l color -d 'When to color the output. `automatic` means when printing to a terminal, tre will include colors; otherwise it will disable colors' -r -f -a "{automatic	,always	,never	}"
complete -c tre -s h -l help -d 'Print help information'
complete -c tre -s V -l version -d 'Print version information'
complete -c tre -s a -l all -d 'Print all files and directories, including hidden ones'
complete -c tre -s s -l simple -d 'Use normal print despite gitignore settings. \'-a\' has higher priority'
complete -c tre -s d -l directories -d 'Only list directories in output'
complete -c tre -s j -l json -d 'Output JSON instead of tree diagram'
