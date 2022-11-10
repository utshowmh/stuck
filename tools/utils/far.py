#!/usr/bin/env python3

from os import walk
from fire import Fire


def find_and_replace(find: str, replace: str, root_path: str = ".",):
    source_files = []

    for path in walk(root_path):
        if len(path) != 3:
            raise Exception("Underflow!!!")
        for file in path[2]:
            source_files.append(f"{path[0]}/{file}")

    for file in source_files:
        content = ""
        with open(file, 'r') as f:
            for word in f.read().split():
                if word == find:
                    content += replace
                else:
                    content += word
        with open(file, "w") as f:
            f.write(content)
        print(f"working on {file}.")

    print("-"*25)
    print(f"mission accomplished!")


if __name__ == "__main__":
    Fire(find_and_replace)
