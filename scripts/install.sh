#!/bin/sh
# Installs a released collector-app on a Linux machine (x86_64 or aarch64, glibc 2.31+):
#
#   curl -fsSL https://github.com/imrans110/rusty-sunspec-collector/releases/latest/download/install.sh | sudo sh
#
# It downloads the release archive for this machine, checks it against the release's
# SHA256SUMS, and installs:
#
#   /usr/local/bin/collector-app and /usr/local/bin/sunspec-sim
#   /etc/systemd/system/sunspec-collector.service
#   /etc/sunspec-collector.env, only if it does not exist yet (upgrades keep your settings)
#   /usr/local/share/doc/sunspec-collector/ (README, CHANGELOG, licenses, config example)
#
# It does not start or restart the service.
#
# Environment:
#   SUNSPEC_VERSION   release to install, e.g. 0.2.0 (default: the release this script came from)
#   SUNSPEC_ROOT      install under this directory instead of / (image builds, tests)
#   SUNSPEC_TARGET    target triple to install instead of this machine's
#   SUNSPEC_BASE_URL  where the archive and SHA256SUMS are (default: the GitHub release)
set -eu

repo=imrans110/rusty-sunspec-collector
min_glibc=2.31

say() { printf '%s\n' "$*"; }
die() { printf 'install.sh: %s\n' "$*" >&2; exit 1; }

# True if version $1 >= $2 (major.minor).
version_ge() {
    awk -v a="$1" -v b="$2" 'BEGIN {
        split(a, x, "."); split(b, y, ".")
        exit !(x[1] + 0 > y[1] + 0 || (x[1] + 0 == y[1] + 0 && x[2] + 0 >= y[2] + 0))
    }'
}

detect_target() {
    [ "$(uname -s)" = Linux ] || die "release binaries are for Linux; this is $(uname -s)"
    case "$(uname -m)" in
        x86_64 | amd64) target=x86_64-unknown-linux-gnu ;;
        aarch64 | arm64) target=aarch64-unknown-linux-gnu ;;
        *) die "no release binary for $(uname -m); build from source (docs/build.md)" ;;
    esac
    glibc=$(getconf GNU_LIBC_VERSION 2>/dev/null | awk '{ print $2 }') || glibc=
    [ -n "$glibc" ] ||
        die "glibc not found; on musl systems such as Alpine use the Docker image instead"
    version_ge "$glibc" "$min_glibc" ||
        die "the binaries need glibc $min_glibc or newer; this system has $glibc"
}

fetch() {
    if command -v curl >/dev/null 2>&1; then
        curl -fsSL -o "$2" "$1" || die "download failed: $1"
    elif command -v wget >/dev/null 2>&1; then
        wget -qO "$2" "$1" || die "download failed: $1"
    else
        die "curl or wget is required"
    fi
}

sha256() {
    if command -v sha256sum >/dev/null 2>&1; then
        sha256sum "$1" | awk '{ print $1 }'
    elif command -v shasum >/dev/null 2>&1; then
        shasum -a 256 "$1" | awk '{ print $1 }'
    else
        die "sha256sum or shasum is required"
    fi
}

# Installs $2 as $3 with mode $1, replacing an existing file atomically (a running binary
# keeps its old inode).
put() {
    mkdir -p "$(dirname "$3")"
    install -m "$1" "$2" "$3.new"
    mv -f "$3.new" "$3"
}

main() {
    version=${SUNSPEC_VERSION:-@VERSION@}
    case "$version" in
        *@*) die "set SUNSPEC_VERSION (e.g. SUNSPEC_VERSION=0.2.0); this copy is not from a release" ;;
    esac
    version=${version#v}
    root=${SUNSPEC_ROOT:-}
    native=yes
    if [ -n "${SUNSPEC_TARGET:-}" ]; then
        target=$SUNSPEC_TARGET
        native=no
    else
        detect_target
    fi
    if [ -z "$root" ] && [ "$(id -u)" != 0 ]; then
        die "run as root, e.g. curl -fsSL <url> | sudo sh"
    fi
    base_url=${SUNSPEC_BASE_URL:-https://github.com/$repo/releases/download/v$version}
    name="sunspec-collector-$version-$target"

    tmp=$(mktemp -d)
    trap 'rm -rf "$tmp"' EXIT
    trap 'exit 1' INT TERM

    say "Downloading $name.tar.gz"
    fetch "$base_url/$name.tar.gz" "$tmp/$name.tar.gz"
    fetch "$base_url/SHA256SUMS" "$tmp/SHA256SUMS"
    expected=$(awk -v file="$name.tar.gz" '$2 == file || $2 == "*" file { print $1 }' "$tmp/SHA256SUMS")
    [ -n "$expected" ] || die "SHA256SUMS has no entry for $name.tar.gz"
    [ "$(sha256 "$tmp/$name.tar.gz")" = "$expected" ] || die "checksum mismatch for $name.tar.gz"
    tar -xzf "$tmp/$name.tar.gz" -C "$tmp"
    src="$tmp/$name"

    bin_dir="$root/usr/local/bin"
    doc_dir="$root/usr/local/share/doc/sunspec-collector"
    unit="$root/etc/systemd/system/sunspec-collector.service"
    env_file="$root/etc/sunspec-collector.env"

    put 0755 "$src/collector-app" "$bin_dir/collector-app"
    put 0755 "$src/sunspec-sim" "$bin_dir/sunspec-sim"
    for file in README.md CHANGELOG.md LICENSE-MIT LICENSE-APACHE LICENSE-SUNSPEC-MODELS \
        config.example.toml sunspec-collector.env; do
        put 0644 "$src/$file" "$doc_dir/$file"
    done
    if [ -f "$unit" ] && ! cmp -s "$src/sunspec-collector.service" "$unit"; then
        cp -p "$unit" "$unit.bak"
        say "Saved your previous unit as $unit.bak (use 'systemctl edit' for local changes)"
    fi
    put 0644 "$src/sunspec-collector.service" "$unit"
    if [ -f "$env_file" ]; then
        say "Kept $env_file; the shipped example is $doc_dir/sunspec-collector.env"
    else
        # 0600: the file may hold Kafka passwords; systemd reads it as root.
        put 0600 "$src/sunspec-collector.env" "$env_file"
    fi

    if [ "$native" = yes ]; then
        installed=$("$bin_dir/collector-app" --version) || die "the installed binary does not run"
        if [ -z "$root" ] && command -v systemctl >/dev/null 2>&1; then
            systemctl daemon-reload
        fi
    else
        installed="collector-app $version ($target)"
    fi

    say "Installed $installed to $bin_dir"
    say "Next: edit /etc/sunspec-collector.env (devices, Kafka brokers), then run"
    say "  systemctl enable --now sunspec-collector     # first install"
    say "  systemctl restart sunspec-collector          # upgrade"
}

# Everything runs from here, so a partly downloaded script (curl | sh) does nothing.
main "$@"
