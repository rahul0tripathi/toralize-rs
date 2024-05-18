#!/bin/bash

export DYLD_INSERT_LIBRARIES=/Users/rahultripathi/Desktop/github/toralize-rs/target/debug/libtoralize.dylib

echo "CUSTOM DYLIB" $DYLD_INSERT_LIBRARIES
# shellcheck disable=SC2068
${@}

unset DYLD_INSERT_LIBRARIES

