# master

- Add '--directotries-only/-d' option, which limits output to only directories.
- Add '--json/-j' option, which makes tre output JSON instead of the tree diagram.

# 0.3.1

Support LS_COLORS, with a default set of colors.

# 0.3.0

Added editor/program aliasing support for Command Prompt and PowerShell on
Windows.

# 0.2.3

Fixed a issue where single quote in file name prevents editor aliasing

# 0.2.2

Improve stability. Reduce possibility of crashing.

# 0.2.1

Add initial Windows build.

# 0.2.0

**Breaking change**: The `-e` option now takes an optional argument. When the
argument is supplied, it'll be used as the command instead of the `$EDITOR`
environment variable for opening the selected path. In previous setup
instructions, the invocation in shell scripts is `tre -e PATH`. In this release,
that command will cause `tre` to think `PATH` is the editor argument. So user
will need to update their setup to be `tre PATH -e` instead (README has been
updated to reflect this.)

This release is rewritten from scratch in Rust, which brings some nice speed up
and Linux binary distribution. In the near futrue, Windows PowerShell support
could be added as well.

