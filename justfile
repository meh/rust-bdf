# List the justfile recipes
list:
    just --list

# Generate the README from the lib.rs docs
readme:
    cargo doc2readme --template README.j2 --out README.md
