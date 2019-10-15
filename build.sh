#!/bin/sh

# https://github.com/millerjs/modelm
# mount.cifs -o user=mopar,pass=,uid=1000,gid=1000,forceuid //mojave/mopar /mnt/mojave/

ssh mojave bash -s "$@" <<EOF
. .profile
cd /Users/mopar/IdeaProjects/rusty-keys
cargo "\$@"
EOF
