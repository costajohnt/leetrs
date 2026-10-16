# ADR-003: Modular Workspace Crate for Native Cookie Extraction and Rookie Deprecation

## Status
Accepted (Supersedes ADR-002)

## Date
2026-10-10

## Context
ADR-002 adopted the external `rookie` crate to extract `LEETCODE_SESSION` and `csrftoken` cookies from Chrome and Firefox SQLite databases. However:
1. The `rookie` crate is **abandonware / unmaintained** and introduces a heavy transitive dependency footprint.
2. Modern OS-level security changes (e.g., Windows Chrome 127+ App-Bound DPAPI encryption, macOS Keychain entitlements, and SQLite file locking on open browser instances) make monolithic out-of-process extraction fragile if not carefully isolated.
3. LeetCode authentication only requires cookies for `leetcode.com` across mainstream browsers (Firefox, Chrome, Brave, Edge, Arc).

Key requirements:
1. **Zero External Installs:** Users should not need to install extensions, companion apps, or browser drivers.
2. **Modular Architecture:** Cookie extraction must be decoupled into an independent, reusable internal workspace crate (`crates/cookie-extract`).
3. **Robust Direct Extraction:** Native Firefox (plaintext SQLite via safe temporary copy) and Chromium (Keychain/DPAPI/SecretService + AES decryption) extraction.
4. **Resilient Fallbacks:** Universal single-paste parser (cURL command / `Cookie:` header / token pairs) with instant validation against LeetCode's GraphQL API.

## Decision
1. Create a dedicated Cargo workspace crate at `crates/cookie-extract` with a clean, decoupled public API (`extract()`, `parse_raw_cookie_header()`).
2. Implement focused browser extractors:
   - `FirefoxExtractor`: Locates default profile in `profiles.ini`, copies `cookies.sqlite` to temp to avoid lock contention, and reads plaintext cookies.
   - `ChromiumExtractor`: Locates profile paths for Chrome, Brave, Edge, and Arc; derives encryption keys from OS keyrings (macOS Keychain, Linux SecretService, Windows DPAPI); and decrypts AES-CBC/GCM payloads.
   - `Parser`: Robustly parses copied cURL snippets, raw `Cookie:` headers, and individual tokens.
3. Remove the `rookie` crate and all its transitive dependencies from `leetrs`.
4. Update `leetrs auth` to use `cookie-extract`, automatically validating extracted/entered credentials before saving to `credentials.json`.

## Alternatives Considered

### 1. Maintain a Fork of `rookie`
- Pros: Minimal changes to `src/auth.rs`.
- Cons: Inherits ~15 browser targets, mobile emulators, and unneeded crypto dependencies.
- Rejected: Unnecessary maintenance burden for features `leetrs` does not need.

### 2. Browser Companion Extension / Userscript + Localhost Server
- Pros: 100% bypasses OS-level SQLite encryption.
- Cons: Requires users to install third-party browser extensions or userscripts, violating the zero-install constraint.
- Rejected: Excessive user onboarding friction.

### 3. Internal Module in `src/extractor/`
- Pros: Single `Cargo.toml`.
- Cons: SQLite and crypto dependencies pollute the main application crate; cannot be compiled or tested independently.
- Rejected: Workspace crate (`crates/cookie-extract`) provides cleaner architectural isolation.

## Consequences
- `rookie` is completely removed from the dependency tree, improving build times and security posture.
- `cookie-extract` is completely decoupled and independently testable via `cargo test -p cookie-extract`.
- Direct filesystem extraction works seamlessly on Firefox (all OSes) and Chromium (macOS, Linux, Windows pre-v127), while gracefully falling back to smart single-paste for restricted environments.
