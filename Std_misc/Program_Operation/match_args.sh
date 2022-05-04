#!/bin/sh
rustc match_args.rs
./match_args Rust
./match_args 42
./match_args do something
./match_args do 42
./match_args increase 42
./match_args decrease 42

