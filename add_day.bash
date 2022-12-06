#!/bin/bash

mkdir "$1_$2"
cd "$1_$2"
cargo init . --name "advent-of-code-2022-$1-$2"
