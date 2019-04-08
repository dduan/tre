import Foundation

let help = """
usage: tre [option] [path]

Print files, directories, and symlinks in tree form.

Hidden files and those configured to be ignored by git will be (optionally)
ignored.

With correct configuration, each displayed file can have a shell alias created
for it, which opens the file in the default editor.

options:
  -a --all            Print all files and directories, including hidden ones.
  -s --simple         Use normal print despite gitignore settings. '-a' has
                      higher priority.
  -e --editor-aliases Create aliases for each displayed result in
                      /tmp/tre_aliases_$USER and add a number in front of file
                      name to indicate the alias name. For example, a number
                      "42" means an shell alias "e42" has been created. Running
                      "e42" will cause the associated file or directory to be
                      open with $EDITOR.
  -h --help           Show this help message.
  -v --version        Show version.

Project home page: https://github.com/dduan/tre
"""

func showHelp() {
    print(help)
    exit(0)
}
