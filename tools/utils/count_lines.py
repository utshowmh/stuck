#!/usr/bin/env python3

from os import walk

ROOT = "./src"

line_count = 0
source_files = []


def find_files():
    for path in walk(ROOT):
        if len(path) != 3:
            raise Exception("Underflow!!!")
        for file in path[2]:
            source_files.append(f"{path[0]}/{file}")


if __name__ == "__main__":
    find_files()
    for file in source_files:
        with open(file, 'r') as f:
            lines = len(f.read().split('\n'))
            line_count += lines
            print(f"found {lines} lines in {file}")
    print("---------------------------------------")
    print(f"found {line_count} lines in {len(source_files)} files.")
