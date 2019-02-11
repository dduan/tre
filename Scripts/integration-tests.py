#!/usr/bin/env python
# -*- coding: utf8 -*-

from __future__ import print_function
import os
import shutil
import subprocess
import tempfile

# Set up the test fixture.
test_root = tempfile.mkdtemp()
tre = os.path.join(os.getcwd(), '.build/release/tre')
os.chdir(test_root)
for filename in ["README", ".gitignore"]:
    with open(filename, 'w') as f:
        f.write("")
subprocess.call(["git", "init"])
subprocess.call(["git", "add", "."])
subprocess.call(["git", "commit", "-m", '"initial commit"'])

# Test tre
print('Testing "tre"', end='')
test1_expectation = [
    "── .gitignore",
    "── README",
]

test1 = subprocess.check_output([tre])
assert(test1.startswith("."))
for expectation in test1_expectation:
    assert(expectation in test1)
print("...ok")

# Test tre -s
print('Testing "tre -s"', end='')
test2_expectation = [
    "── README",
]

test2 = subprocess.check_output([tre, "-s"])
assert(test2.startswith("."))
assert("── README" in test2)
assert(not "── .gitignore" in test2)
print("...ok")

# Test tre -a
print('Testing "tre -a"', end='')
test3_expectation = [
    "── hooksx",
    "── .gitignore",
    "── README",
]
test3 = subprocess.check_output([tre, "-a"])
assert(test3.startswith("."))
for expectation in test3_expectation:
    assert(expectation in test3)
print("...ok")

shutil.rmtree(test_root)

