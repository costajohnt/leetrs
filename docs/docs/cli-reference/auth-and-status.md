---
id: auth-and-status
title: auth & status
sidebar_position: 2
---

# 🔑 `leetrs auth` & `leetrs status`

Commands for managing LeetCode session credentials.

---

## 🔒 `leetrs auth`

Launches the interactive authentication prompt to obtain and store LeetCode session cookies.

### Usage

```bash
# Standard interactive authentication
leetrs auth

# With verbose diagnostic logging
leetrs --verbose auth
# or
leetrs -v auth
```

### Prompt Options

```text
🔒 LeetCode Authentication

? How would you like to authenticate?
❯ Auto-detect browser (Recommended)
  Paste cURL / Cookie header manually
  Extract from Firefox
  Extract from Chrome
  Extract from Brave
  Extract from Edge
  Extract from Arc
```

1. **Auto-detect browser**: Scans available browser profile databases across all supported browser families and automatically extracts cookies from the active session.
2. **Paste cURL / Cookie header manually**: Allows pasting a full copied cURL command (`-H 'Cookie: ...'` or `-b '...'`) or raw `Cookie:` header from browser Developer Tools.
3. **Extract from specific browser**: Direct extraction targeting **Firefox** (including forks like LibreWolf, Floorp, Waterfox), **Chrome**, **Brave**, **Edge**, or **Arc**.

:::info[Native Engine]
Browser extraction is performed by the native [`cookie-extract`](https://github.com/shadowmkj/leetrs/tree/main/crates/cookie-extract) workspace crate (replacing the deprecated `rookie` crate). It uses isolated temporary SQLite staging to prevent database lock contention while your browser is open.
:::

---

## 🔍 `leetrs status`

Displays the active authentication state and token information saved in `credentials.json`.

### Usage

```bash
leetrs status
```

### Output Example (Authenticated)

```text
✅ Currently authenticated!
🔑 csrftoken:
d9a8b7c6d5e4f3a2109876543210abcd

🔑 LEETCODE_SESSION:
eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

### Output Example (Unauthenticated)

```text
❌ Not authenticated. No valid credentials found.
Run `leetrs auth` to set up your account.
```
