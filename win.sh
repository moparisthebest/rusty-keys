#!/bin/sh
set -euxo pipefail

target="${1-x86_64-pc-windows-gnu}"

cargo build --target "$target" --release

cp keymap.toml target/"$target"/release/
cd target/"$target"/release/
strip rusty-keys.exe
zip -9 rusty-keys-"$target".zip rusty-keys.exe keymap.toml
mv rusty-keys-"$target".zip ../../..

exit 0

# on arch linux, run these to enable cross compiling to win64:

pacman -S mingw-w64-gcc

# you can change these, if you want
CHANNEL=stable
ARCH=x86_64

rustup install $CHANNEL-$ARCH-pc-windows-gnu && rustup target add $ARCH-pc-windows-gnu

for lib in crt2.o dllcrt2.o libmsvcrt.a; do cp -v /usr/x86_64-w64-mingw32/lib/$lib $HOME/.rustup/toolchains/$CHANNEL-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-pc-windows-gnu/lib/; done

cat >> ~/.cargo/config <<EOF
[target.$ARCH-pc-windows-gnu]
linker = "/usr/bin/$ARCH-w64-mingw32-gcc"
ar = "/usr/$ARCH-w64-mingw32/bin/ar"
EOF
