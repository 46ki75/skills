# wsl-notification

A Claude Code plugin that surfaces `Notification` and `Stop` hook events as
Windows toast notifications when Claude Code is running inside **WSL2**.

The hook reads the JSON payload Claude Code sends on stdin, extracts
`.message`, and dispatches a toast via `powershell.exe`.

- **`Notification`** fires when Claude needs permission or the prompt has been
  idle for ~60 seconds. The toast body is the event's `.message`.
- **`Stop`** fires whenever Claude finishes responding. Its payload carries no
  `.message`, so a fixed `"Claude finished responding."` body is used. This
  still works under bypass-permissions mode, where `Notification` permission
  events never fire.

## Requirements

- WSL2 on Windows. The hook detects non-WSL environments via `/proc/version`
  and exits cleanly, so it is safe to install on systems where it cannot run —
  but it will only produce notifications inside WSL2.
- `powershell.exe` reachable on `PATH` (default on WSL2).
- `jq` installed inside the WSL2 distribution.

## Install

This plugin is distributed through the `46ki75-skills` marketplace:

```bash
/plugin marketplace add 46ki75/skills
/plugin install wsl-notification@46ki75-skills
```

## What it does

The plugin registers a `Notification` and a `Stop` hook:

```json
{
  "hooks": {
    "Notification": [
      {
        "matcher": "",
        "hooks": [
          {
            "type": "command",
            "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/notify.sh Notification"
          }
        ]
      }
    ],
    "Stop": [
      {
        "matcher": "",
        "hooks": [
          {
            "type": "command",
            "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/notify.sh Stop"
          }
        ]
      }
    ]
  }
}
```

`${CLAUDE_PLUGIN_ROOT}` is expanded by Claude Code to this plugin's install
directory at runtime.

## Layout

```text
wsl-notification/
├── .claude-plugin/
│   └── plugin.json
├── hooks/
│   └── hooks.json
└── scripts/
    └── notify.sh
```

## License

MIT.
