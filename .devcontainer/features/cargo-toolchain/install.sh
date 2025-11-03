#!/bin/bash

set -e -u -o pipefail

USERNAME="${USERNAME:-"vscode"}"

su "${USERNAME}" -c "curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash"
su "${USERNAME}" -c "cargo binstall just cargo-lambda --no-confirm"

echo "Done!"
