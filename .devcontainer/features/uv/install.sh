#!/bin/bash

set -e -u -o pipefail

USERNAME="${USERNAME:-"vscode"}"

su "${USERNAME}" -c "curl -LsSf https://astral.sh/uv/install.sh | sh"

echo "Done!"
