# Modifications

This is a modified version of [Zebar](https://github.com/glzr-io/zebar) by
Glzr Software Pte. Ltd. It is not the official Zebar and is not endorsed by or
affiliated with the Zebar project. Please report problems here, not upstream.

- **Upstream base:** Zebar 3.3.1 (commit `6aa335f`)
- **Modified by:** herlondf
- **Modified since:** September 2026
- **Source:** https://github.com/herlondf/zebar

Zebar is licensed under the GNU General Public License v3.0, and so is this
modified version. The original `LICENSE.md` is unchanged.

## Why this fork exists

Zebar and GlazeWM both refuse to run for a second logged-in Windows user,
because each of them claims a fixed TCP port and a port belongs to the machine
rather than the session. The first user to log in gets the port and everyone
else fails to start.

An upstream pull request for this was
[closed](https://github.com/glzr-io/zebar/pull/296): the maintainer wants to
solve the port handling himself, and rejected the approach used here for
reaching GlazeWM from a widget. The changes live on in this fork instead.

## What was changed

- **Asset server port** (`packages/desktop/src/asset_server.rs`) — keeps 6124
  whenever it is free and asks the OS for a free port when it isn't, rather
  than binding 6124 unconditionally. Without this, the second user's Zebar
  cannot start at all.
- **Widget capability** (`packages/desktop/capabilities/widget.json`) — the
  remote URL allowlist had port 6124 written into it, which left a widget
  served from any other port with no access to Tauri commands. It now accepts
  any loopback port.
- **Port injection** (`packages/desktop/src/widget_factory.rs`) — widgets get
  `window.__ZEBAR_PORTS` before any of their own code runs.
- **GlazeWM port discovery** (`packages/desktop/src/glazewm_ipc.rs`, new) —
  reads the port that GlazeWM publishes to `~/.glzr/glazewm/ipc-port-<session>`,
  falling back to 6123.
- **Initialization script**
  (`packages/desktop/resources/initialization-script.js`) — a host check had
  port 6124 hardcoded and silently dropped the service worker and
  `normalize.css` once the port stopped being fixed. It also wraps
  `window.WebSocket` to point `glazewm-js` at the right port, since widgets
  load that library from a CDN and cannot be reached through `client-api`.
  The wrapper is skipped entirely when the port really is 6123.
- **Client API served locally** (`packages/desktop/src/asset_server.rs`) —
  widgets can import it from `/__zebar/zebar.js` instead of a CDN, which
  means they work offline and pick up changes to `packages/client-api`
  without waiting for an npm release.
- **GlazeWM provider takes a port**
  (`packages/client-api/src/providers/glazewm/`) — it defaults to the
  instance running in this session. A widget importing the client API
  locally therefore reaches the right GlazeWM with no `window.WebSocket`
  patching. The patch stays for widgets that still import from a CDN.
- **Command provider** (`packages/desktop/src/providers/command/`) — runs a
  program on an interval and hands the widget its stdout, stderr and exit
  code. It is checked against the same `privileges.shellCommands` that
  `shellExec` is held to, before the emission cache is consulted, so a widget
  without the privilege cannot subscribe to one another widget started.
- **Branding** (`packages/desktop/tauri.conf.json`,
  `packages/desktop/installer.wxs`) — renamed to `Zebar Multi-Session` with its
  own bundle identifier, publisher and MSI upgrade code, so it installs
  alongside official Zebar instead of replacing it.

Configuration still lives in `~/.glzr/zebar` and `%APPDATA%\zebar`, shared with
official Zebar. Running both at once is not expected to work.

## Companion changes in GlazeWM

The GlazeWM half is proposed upstream rather than forked:
[glzr-io/glazewm#1437](https://github.com/glzr-io/glazewm/pull/1437).

## Releasing a new build

The MSI will only install over an earlier one if the version goes up, so bump
`version` in `packages/desktop/tauri.conf.json` for every build meant to
replace an installed one.
