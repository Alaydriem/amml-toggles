<div align="center">

<img src="./assets/icon.png" alt="Toggles" width="256">

# Alaydriem's Toggles

[![Release](https://img.shields.io/github/v/release/alaydriem/amml-toggles?include_prereleases)](https://github.com/alaydriem/amml-toggles/releases)
[![Build](https://img.shields.io/github/actions/workflow/status/alaydriem/amml-toggles/build.yml?branch=master&label=build)](https://github.com/alaydriem/amml-toggles/actions/workflows/build.yml)
[![License](https://img.shields.io/github/license/alaydriem/amml-toggles)](./LICENSE)
![Minecraft Bedrock](https://img.shields.io/badge/Minecraft_Bedrock-1.26.60-62B47A)
[![AMML](https://img.shields.io/badge/AMML-mod-4A90D9)](https://github.com/alaydriem/amml)

</div>

Toggle sneak and auto-walk for Minecraft Bedrock, without holding a key.

Client-side only.

## Features

- Toggle sneak
- Toggle auto-walk
- Both toggles turn off when a menu opens or the player dies (each can be changed)

## Installation

Install it with the [AMML launcher](https://github.com/alaydriem/amml/releases/latest). The launcher also installs the AMML loader and Alaydriem's Mod Manager, which this mod needs.

## Controls

| Key | Action |
| --- | --- |
| <kbd>F3</kbd> + <kbd>Shift</kbd> | Toggle sneak |
| <kbd>F3</kbd> + <kbd>W</kbd> | Toggle auto-walk |
| <kbd>W</kbd> or <kbd>S</kbd> | Cancel auto-walk |
| <kbd>Shift</kbd> | Cancel sneak |

The keys can be changed on the mod manager's Keybinds page.

## Configuration

Settings are in the mod manager panel and saved to `mods/amml/@alaydriem/toggles/config/config.hcl`.

| Setting | Default | Description |
| --- | --- | --- |
| `toggles.sneak_enabled` | `true` | Let the sneak toggle key work |
| `toggles.walk_enabled` | `true` | Let the auto-walk toggle key work |
| `toggles.reset_on_menu` | `true` | Turn both toggles off when a menu or screen opens |
| `toggles.reset_on_death` | `true` | Turn both toggles off when the player dies |
| `keys.sneak` | `"F3+Shift"` | Toggle sneak key |
| `keys.walk` | `"F3+W"` | Toggle auto-walk key |
| `keys.game_sneak` | `"Shift"` | Must match the Sneak key in Minecraft's controls |
| `keys.game_forward` | `"W"` | Must match the Forward key in Minecraft's controls |
| `keys.cancel_walk_1` / `cancel_walk_2` | `"W"` / `"S"` | Keys that stop auto-walk |

## FAQ

**Auto-walk or sneak does nothing.** The mod presses the game's own Sneak and Forward keys. If you changed them in
Minecraft's controls, set `keys.game_sneak` and `keys.game_forward` to match.

## Building

Requires Windows x64, [mise](https://mise.jdx.dev), rustup and [cargo-nextest](https://nexte.st). No sibling checkout is needed: cargo fetches `amml` and `amml-modmanager` at the release tag named in `Cargo.toml`.

```powershell
mise trust
mise run preflight
mise run package
mise run test
```

`mise run package` writes `dist/@alaydriem/toggles/`. `mise run install [--instance <name>]` runs `package`, then copies it into `%LOCALAPPDATA%\amml\instances\<name>\mods\amml\@alaydriem\toggles\` (default instance `1.26.60.05`). Close the game first.

## License

BSD 3-Clause. Copyright (c) 2026 Charles R. Portwood II. See [LICENSE](./LICENSE).
