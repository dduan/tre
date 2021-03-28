#!/usr/bin/env python3

import re
import sys

expected = re.search(r'name = "tre-command"\nversion = "(.+)"', open('Cargo.toml').read(), re.M).group(1)
cargo_lock = re.search(r'name = "tre-command"\nversion = "(.+)"', open('Cargo.lock').read(), re.M).group(1)
changelog = re.search(r'# main\s+#\s*(.+)', open('CHANGELOG.md').read(), re.M).group(1)
manual_match = re.search(r'.TH "TRE" "1" "2020-08-05" "TRE (\d+)\\&.(\d+)\\&.(\d+)" "Tre Manual"', open('manual/tre.1').read())
manual = "{}.{}.{}".format(manual_match.group(1), manual_match.group(2), manual_match.group(3))

has_error = False
if expected != cargo_lock:
    print("version mismatch: expected {}; found {} in Cargo.lock.".format(expected, cargo_lock), file=sys.stderr)
    has_error = True

if expected != changelog:
    print("version mismatch: expected {}; found {} in CHANGELOG.md.".format(expected, cargo_lock), file=sys.stderr)
    has_error = True

if expected != manual:
    print("version mismatch: expected {}; found {} in manual/tre.1.".format(expected, cargo_lock), file=sys.stderr)
    has_error = True

if has_error:
    exit(1)
