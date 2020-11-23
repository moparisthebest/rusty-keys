#!/bin/bash
set -exo pipefail

echo "starting build for TARGET $TARGET"

export CRATE_NAME=rusty-keys

DISABLE_TESTS=${DISABLE_TESTS:-0}

SUFFIX=""

echo "$TARGET" | grep -E '^x86_64-pc-windows-gnu$' >/dev/null && SUFFIX=".exe"

[ "$TARGET" == 'riscv64gc-unknown-linux-gnu' ] && echo 'riscv64gc-unknown-linux-gnu is not yet supported by inotify, skipping build...' && exit 0

# no main impl for these platforms
echo "$TARGET" | grep -E '(android|solaris$)' >/dev/null && DISABLE_TESTS=1

cross build --target $TARGET --release

# to check how they are built
file "target/$TARGET/release/rusty-keys$SUFFIX"

if [ $DISABLE_TESTS -ne 1 ]
then
    # only going to run --help I guess
    cross run --target $TARGET --release --bin rusty-keys -- -h
fi

# if this commit has a tag, upload artifact to release
strip "target/$TARGET/release/rusty-keys$SUFFIX" || true # if strip fails, it's fine
mkdir -p release
mv "target/$TARGET/release/rusty-keys$SUFFIX" "release/rusty-keys-$TARGET$SUFFIX"

echo 'build success!'
exit 0
