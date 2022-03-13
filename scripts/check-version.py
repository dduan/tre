#!/usr/bin/env python3

import re
import sys

expected = re.search(r'name = "tre-command"\nversion = "(.+)"', open('Cargo.toml').read(), re.M).group(1)
versions = {}
versions['Cargo.lock'] = re.search(r'name = "tre-command"\nversion = "(.+)"', open('Cargo.lock').read(), re.M).group(1)
versions['CHANGELOG.md'] = re.search(r'# main\s+#\s*(.+)', open('CHANGELOG.md').read(), re.M).group(1)
versions['flake.nix'] = re.search(r'version = "(.+)"', open('flake.nix').read(), re.M).group(1)
manual_match = re.search(r'.TH "TRE" "1" "2020-08-05" "TRE (\d+)\\&.(\d+)\\&.(\d+)" "Tre Manual"', open('manual/tre.1').read())
versions['manual/tre.1'] = "{}.{}.{}".format(manual_match.group(1), manual_match.group(2), manual_match.group(3))

for file in versions:
    if expected != versions[file]:
        print("version mismatch: expected {expeted}; found {versions[file]} in {file}", file=sys.stderr)
        exit(1)
