#!/bin/sh
# Packages release binaries with their docs and service files:
#
#   scripts/package-release.sh <version> <target> <dir containing collector-app and sunspec-sim>
#
# Writes dist/sunspec-collector-<version>-<target>.tar.gz (one top-level directory of the same
# name) and copies the binaries to dist/bin/<docker arch>/ for the release images. Run from the
# repository root.
set -eu

if [ $# -ne 3 ]; then
    echo "usage: $0 <version> <target> <bin-dir>" >&2
    exit 2
fi
version=$1
target=$2
bin_dir=$3

case "$target" in
    x86_64-*) docker_arch=amd64 ;;
    aarch64-*) docker_arch=arm64 ;;
    *) echo "unsupported target: $target" >&2; exit 2 ;;
esac

name="sunspec-collector-$version-$target"
stage="dist/$name"
rm -rf "$stage" "dist/$name.tar.gz"
mkdir -p "$stage" "dist/bin/$docker_arch"

for bin in collector-app sunspec-sim; do
    install -m 0755 "$bin_dir/$bin" "$stage/$bin"
    install -m 0755 "$bin_dir/$bin" "dist/bin/$docker_arch/$bin"
done
for file in README.md CHANGELOG.md LICENSE-MIT LICENSE-APACHE \
    docs/config.example.toml docs/sunspec-collector.env docs/sunspec-collector.service; do
    install -m 0644 "$file" "$stage/"
done
# The SunSpec model definitions compiled into the binaries (Apache-2.0).
install -m 0644 crates/sunspec-parser/models/LICENSE "$stage/LICENSE-SUNSPEC-MODELS"

# Root-owned entries and no gzip timestamp, so the archive does not leak the build machine's
# user names and only changes when its contents do.
if tar --version 2>/dev/null | grep -q 'GNU tar'; then
    set -- --owner=0 --group=0 --numeric-owner --sort=name
else
    set -- --uid 0 --gid 0
fi
tar -C dist "$@" -cf - "$name" | gzip -9n > "dist/$name.tar.gz"
rm -rf "$stage"
echo "dist/$name.tar.gz"
