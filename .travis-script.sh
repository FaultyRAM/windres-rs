#!/bin/sh
if ["$FORMATTING" = "true"]; then
    cargo install rustfmt-nightly;
    cargo fmt -- --write-mode diff;
else
    cargo build -vv;
    cargo test -vv;
fi
