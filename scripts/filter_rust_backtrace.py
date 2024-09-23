#!/usr/bin/python3
import fileinput

global mute
global lines

mute = False
lines = []

for line in fileinput.input():
    stripped_line = line.strip()
    if stripped_line[:2] != "at":
        if "tokio::" in stripped_line:
            mute = True
        elif "rust_begin_unwind" in stripped_line:
            mute = True
        elif "std::" in stripped_line:
            mute = True
        elif "core::" in stripped_line:
            mute = True
        elif "salsa::" in stripped_line:
            mute = True
        elif "<DB" in stripped_line:
            mute = True
        elif "<alloc::" in stripped_line:
            mute = True
        else:
            mute = False
    if not mute:
        if stripped_line[:2] != "at":
            lines.append(stripped_line)
        else:
            last_line = lines.pop()
            lines.append(last_line + "\t" + stripped_line)
for line in lines:
    print(line)
