#!/bin/sh
rustc raii.rs && valgrind ./raii
