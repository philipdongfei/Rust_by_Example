#!/bin/sh
rustc doc.rs --crate-type lib
rustdoc --test --extern doc="libdoc.rlib" doc.rs

