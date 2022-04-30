#!/bin/sh
rustc panic.rs && valgrind ./panic
