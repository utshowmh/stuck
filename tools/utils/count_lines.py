#!/usr/bin/env python3

from os import walk
from fire import Fire


def count_lines(root_path: str = "."):
    line_count = 0
    source_files = []

    for path in walk(root_path):
        if len(path) != 3:
            raise Exception("Underflow!!!")
        for file in path[2]:
            source_files.append(f"{path[0]}/{file}")

    for file in source_files:
        with open(file, 'r') as f:
            lines = len(f.read().split('\n'))
            line_count += lines
            print(f"found {lines} lines in {file}.")

    print("-"*100)
    print(f"found {line_count} lines in {len(source_files)} files.")


if __name__ == "__main__":
    Fire(count_lines)
