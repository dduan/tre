# tre

A replacement for `tree` command that uses `git ls-files` as source of file when
possible.

_tre_ can also create shell aliases that, when executed, opens the file
associated with it with the default editor. Here's that function in action:

![Aliasing In Action](alias_demo.gif)

… in case you missed it: "3" is listed in front of "README.md" and typing "e3"
opened the file! See [how to set this up](#editor-aliasing).

## Install

### Homebrew

```
brew install dduan/formulae/tre
```

### Mint
```
mint install dduan/tre
```

## Editor aliasing

### Setting it up

tre provides a `-e` flag that, when used, turns on the "editor aliasing"
feature. Some shell configuration will make this work better.

#### Bash or Zsh

In `~/.bashrc` or `~/.zshrc` (for example)

```bash
tre() { command tre -e "$@" && source "/tmp/tre_aliases_$USER" 2>/dev/null; }
```

#### Fish

Create `~/.config/fish/functions/tre.fish`:

```fish
function tre
  command tre -e $argv; and source /tmp/tre_aliases_$USER ^/dev/null
end
```


### How it works

The first thing you'll notice is some numbers in front of each file name in
tre's output. If pick a number, say, "3", and enter `e3` in the shell, the file
after "3" will open in your default editor (specified by the environment
variable `EDITOR`).

Everytime tre runs with this flag, it updates a file `/tmp/tre_aliases_$USER`
and adds a alias for each result it displays. And the shell configuration simply
sources this file after the command. You can manually run

```bash
source /tmp/tre_aliases_$USER
```

… instead of configuring your shell (if you are _that_ patient!).

This feature is inspired by [tag](https://github.com/keith/tag).

## Help

```
usage: tre [option] [path]

Print files, directories, and symlinks in tree form.
Hidden files and those configured to be ignored by git will be (optionally)
ignored.

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
```

## License

MIT. See `LICENSE.md`.
