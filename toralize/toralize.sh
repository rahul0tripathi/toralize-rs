#!/bin/bash

export DYLD_INSERT_LIBRARIES=$PWD/target/debug/libtoralize.dylib
echo "CUSTOM DYLIB LOADED"
# shellcheck disable=SC2068
${@}

unset DYLD_INSERT_LIBRARIES

