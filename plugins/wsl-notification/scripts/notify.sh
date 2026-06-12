#!/usr/bin/env bash
# Claude Code hook: WSL2 -> Windows toast.
# Reads the hook JSON payload on stdin, extracts `.message`, and dispatches a
# Windows toast via powershell.exe. No-ops cleanly outside WSL2.
#
# Usage: notify.sh [EVENT]
#   EVENT defaults to "Notification". Pass "Stop" for the Stop hook, whose
#   payload carries no `.message`, so a fixed fallback message is used.

set -u

event="${1:-Notification}"

if ! grep -qiE 'microsoft|wsl' /proc/version 2>/dev/null; then
  exit 0
fi

if ! command -v powershell.exe >/dev/null 2>&1; then
  exit 0
fi

if ! command -v jq >/dev/null 2>&1; then
  echo "wsl-notification: jq is required but not found in PATH" >&2
  exit 0
fi

title="Claude Code | ${event}"
input="$(cat)"
message="$(jq -r '.message // empty' <<<"$input")"

if [[ -z "$message" ]]; then
  case "$event" in
    Stop) message="Claude finished responding." ;;
    *) exit 0 ;;
  esac
fi

# Escape single quotes for PowerShell single-quoted strings.
ps_escape() { printf '%s' "$1" | sed "s/'/''/g"; }
ps_title=$(ps_escape "$title")
ps_message=$(ps_escape "$message")

powershell.exe -NoProfile -Command "
  [Windows.UI.Notifications.ToastNotificationManager, Windows.UI.Notifications, ContentType=WindowsRuntime] | Out-Null
  \$template = [Windows.UI.Notifications.ToastNotificationManager]::GetTemplateContent([Windows.UI.Notifications.ToastTemplateType]::ToastText02)
  \$elements = \$template.GetElementsByTagName('text')
  \$elements.Item(0).InnerText = '${ps_title}'
  \$elements.Item(1).InnerText = '${ps_message}'
  \$toast = [Windows.UI.Notifications.ToastNotification]::new(\$template)
  [Windows.UI.Notifications.ToastNotificationManager]::CreateToastNotifier('WSL').Show(\$toast)
" >/dev/null 2>&1 || true
