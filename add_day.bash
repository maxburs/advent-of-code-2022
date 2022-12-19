#!/bin/bash
set -euo pipefail

folder_name="$1_$2"
mkdir $folder_name
name="advent-of-code-2022-$1-$2"
cargo init $folder_name --name $name
cat ./template.rs > "$folder_name/src/main.rs"
touch "$folder_name/example.txt"
cd $folder_name
