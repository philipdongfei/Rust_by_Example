#!/bin/bash
# Where library.rlib is the path to the compiled library, assumed that it's
# in the same directory here:
rustc executable.rs --extern rary=library.rlib --edition=2018 && ./executable

