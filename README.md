# template-devcontainer

```bash
npx degit 46ki75/template-devcontainer
```

## Custom Features for Local Development

Create a directory at `.devcontainer/features/<YOUR_FEATURE_NAME>` containing following files:

- `devcontainer-feature.json`: Metadata describing the feature.
- `install.sh`: Shell script to install the feature.

### `devcontainer-feature.json`

- `id`: Identifier for this feature.
- `installAfter`: Specifies dependencies that must be installed before this feature.
- `customizations.vscode`: VSCode settings and extensions to apply when this features is used.

```json
{
  "id": "cargo-binstall",
  "name": "cargo-binstall (via cargo)",
  "version": "1.0.0",
  "customizations": {
    "vscode": {
      "settings": {
        "[rust]": { "editor.defaultFormatter": "rust-lang.rust-analyzer" }
      },
      "extensions": ["rust-lang.rust-analyzer"]
    }
  },
  "installsAfter": ["ghcr.io/devcontainers/features/rust"]
}
```

### `install.sh`

Dev Container features are defined using simple shell scripts.

```bash
#!/bin/bash

set -e -u -o pipefail

USERNAME="${USERNAME:-"vscode"}"

su "${USERNAME}" -c "curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash"
su "${USERNAME}" -c "cargo binstall just cargo-lambda --no-confirm"

echo "Done!"

# Add your custom installation steps below ---
```

### Need More Information?

Exploring the actual implementations in [Available Dev Container Features](https://containers.dev/features) is the best way to learn how to create your own.

For detailed specifications, see the [Dev Container metadata reference](https://containers.dev/implementors/json_reference/).
