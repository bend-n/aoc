#!/bin/bash
curl -sH "Cookie: session=$(cat cooki)" "https://adventofcode.com/2025/day/$(date +"%-d")/input" -o - | tee src/inp.txt
