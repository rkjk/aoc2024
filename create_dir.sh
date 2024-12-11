#!/bin/bash

# Check if the first argument is provided
if [ -z "$1" ]; then
    echo "Please provide the directory name as an argument."
    exit 1
fi

# Create the directory inside src/
mkdir -p src/$1

# Create empty files inside the new directory
touch src/$1/example
touch src/$1/input
touch src/$1/mod.rs

# Append the string to src/lib.rs
echo -e "\nmod $1;" >> src/lib.rs
