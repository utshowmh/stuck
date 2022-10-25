#!/usr/bin/env bash

nasm -f elf64 output.asm
ld output.o -o output
./output