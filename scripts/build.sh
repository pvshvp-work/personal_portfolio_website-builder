#! /usr/bin/env sh

SCRIPT_DIRECTORY="$(dirname -- "$(readlink -f -- "$0")")"
PROJECT_DIRECTORY="$(dirname -- "$SCRIPT_DIRECTORY")"

(
    cd "$PROJECT_DIRECTORY" \
    && dx build --release --features web \
    && cargo run --release --features ssr \
    && cp dist/index.html dist/404.html
)