# KEDB Index

Mutable. Updated whenever an entry in `entries/` is created or modified. Every
entry in `entries/` MUST have a corresponding row here — entries missing from the
index are invisible to future sessions.

Sort rows by `Event Date` descending (most recent first). The `Description`
column is a one-line summary (≤200 chars) used to decide whether to open the
full entry.

| Slug | Title | Status | Severity | Event Date | Description |
| ---- | ----- | ------ | -------- | ---------- | ----------- |
