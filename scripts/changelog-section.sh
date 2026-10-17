#!/bin/sh
# Prints the body of one CHANGELOG.md section, used as the GitHub Release notes:
#
#   scripts/changelog-section.sh 0.2.0        # the "## [0.2.0] - <date>" section
#   scripts/changelog-section.sh Unreleased
#
# Fails if the section is missing or empty. Run from the repository root.
set -eu

if [ $# -ne 1 ]; then
    echo "usage: $0 <version|Unreleased>" >&2
    exit 2
fi

notes=$(awk -v heading="## [$1]" '
    index($0, heading) == 1 { found = 1; next }
    found && /^## \[/ { exit }
    found { print }
' CHANGELOG.md | sed -e '/./,$!d')

if [ -z "$notes" ]; then
    echo "CHANGELOG.md has no \"## [$1]\" section with content" >&2
    exit 1
fi
printf '%s\n' "$notes"
