
use builtin;
use str;

set edit:completion:arg-completer[tre] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'tre'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'tre'= {
            cand -e 'Create aliases for each displayed result, and add a number in front of file name to indicate the alias name. For example, a number "42" means an shell alias "e42" has been created. Running "e42" will cause the associated file or directory to be open with $EDITOR (or a default program for the file type on Windows), or a command specified along with this command'
            cand --editor 'Create aliases for each displayed result, and add a number in front of file name to indicate the alias name. For example, a number "42" means an shell alias "e42" has been created. Running "e42" will cause the associated file or directory to be open with $EDITOR (or a default program for the file type on Windows), or a command specified along with this command'
            cand -l 'Limit depth of the tree in output'
            cand --limit 'Limit depth of the tree in output'
            cand -E 'Exclude paths matching a regex pattern. Repeatable'
            cand --exclude 'Exclude paths matching a regex pattern. Repeatable'
            cand -c 'When to color the output. `automatic` means when printing to a terminal, tre will include colors; otherwise it will disable colors'
            cand --color 'When to color the output. `automatic` means when printing to a terminal, tre will include colors; otherwise it will disable colors'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -V 'Print version information'
            cand --version 'Print version information'
            cand -a 'Print all files and directories, including hidden ones'
            cand --all 'Print all files and directories, including hidden ones'
            cand -s 'Use normal print despite gitignore settings. ''-a'' has higher priority'
            cand --simple 'Use normal print despite gitignore settings. ''-a'' has higher priority'
            cand -d 'Only list directories in output'
            cand --directories 'Only list directories in output'
            cand -j 'Output JSON instead of tree diagram'
            cand --json 'Output JSON instead of tree diagram'
        }
    ]
    $completions[$command]
}
