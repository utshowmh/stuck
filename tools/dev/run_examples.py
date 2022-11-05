#!/usr/bin/env python3

from subprocess import run
from os import walk

STUCK_PATH = "./target/release/stuck"
ROOT = "./examples"

example_paths = []

for path in walk(ROOT):
    if len(path) != 3:
        raise Exception("Underflow!!!")
    for program in path[2]:
        example_paths.append(f"{path[0]}/{program}")


def run_command(command: str):
    print(f"[CMD] '{command}'")
    run(command.split(" "))


if __name__ == "__main__":
    run_command("cargo build --release")
    for example_path in example_paths:
        run_command(f"{STUCK_PATH} {example_path}")
        input("[INFO] press [ENTER] to proceed")
