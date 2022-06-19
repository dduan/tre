#!/usr/bin/env python3

import re
import sys

expected = re.search(r'name = "tre-command"\nversion = "(.+)"', open('Cargo.toml').read(), re.M).group(1)
versions = {}
versions['Cargo.lock'] = re.search(r'name = "tre-command"\nversion = "(.+)"', open('Cargo.lock').read(), re.M).group(1)
versions['CHANGELOG.md'] = re.search(r'#\s*([0-9.]+)', open('CHANGELOG.md').read(), re.M).group(1)
manual_match = re.search(r'.TH "TRE" "1" "[0-9-]+" "TRE (\d+)\\&.(\d+)\\&.(\d+)" "Tre Manual"', open('manual/tre.1').read())
versions['manual/tre.1'] = "{}.{}.{}".format(manual_match.group(1), manual_match.group(2), manual_match.group(3))

for file in versions:
    if expected != versions[file]:
        print(f"version mismatch: expected {expected}; found {versions[file]} in {file}", file=sys.stderr)
        exit(1)
