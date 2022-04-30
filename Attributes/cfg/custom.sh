#!/bin/sh
rustc --cfg some_condition custom.rs && ./custom
#rustc custom.rs && ./custom # error

