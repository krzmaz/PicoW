#!/bin/sh

GENERATOR=""
if which ninja >/dev/null; then
    GENERATOR="-G Ninja"
fi
# Prepare env
FILE=./env.sh
if [ -f "$FILE" ]; then
    source $FILE
else
    echo "File $FILE not found! Trying to generate it"
    # Due to the check in main CMakeLists.txt this will not generate everything, as we need
    # to source the env variables before proceeding
    cmake -B build $GENERATOR
    # we need to remove the cache to make sure that ESP IDF build gets properly regenerated
    rm build/CMakeCache.txt
    source $FILE
fi

# Run cmake generation after sourcing env variables
cmake -B build $GENERATOR
# Build
if [ -z "$GENERATOR" ]; then
    make -j $(getconf _NPROCESSORS_ONLN) -C build $1
    echo "done. P.S.: Consider installing ninja - it's faster"
else
    ninja -C build $1
fi
