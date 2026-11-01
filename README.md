<div align="center">

<img src="./assets/icon.png" alt="Toggles" width="256">

# Alaydriem's Toggles

[![Release](https://img.shields.io/github/v/release/alaydriem/toggles?include_prereleases)](https://github.com/alaydriem/toggles/releases)
[![Build](https://img.shields.io/github/actions/workflow/status/alaydriem/toggles/build.yml?branch=master&label=build)](https://github.com/alaydriem/toggles/actions/workflows/build.yml)
[![License](https://img.shields.io/github/license/alaydriem/toggles)](./LICENSE)
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

Requires:

- the **AMML loader**, which loads the mod into the game.
- [Alaydriem's Mod Manager](https://github.com/alaydriem/modmanager), 26.60 build with ABI 2.1. Without the mod
  manager the toggles stay off.

Download the zip from the [latest release](https://github.com/alaydriem/toggles/releases/latest) and extract it.
It holds the folder `@alaydriem/toggles`, with `alaydriem_toggles.dll` and `amml.hcl`. Put that folder in the AMML
mods folder of the game, so that the files are in `mods/amml/@alaydriem/toggles/`. Then enable the mod in the mod
manager.

| Toggles | Minecraft Bedrock | Mod Manager |
| --- | --- | --- |
| 0.1.0 | 1.26.60 | ABI 2.1 |

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

Requires Windows x64, [mise](https://mise.jdx.dev), rustup and [cargo-nextest](https://nexte.st). The `amml` and `modmanager-amml` repos must sit next to this one.

```powershell
mise trust
mise run preflight
mise run build-mod
mise run test
```

`mise run build-mod` writes `dist/@alaydriem/toggles`. `mise run install-mod` copies it into `LM_MODS_DIR/amml/@alaydriem/toggles` (set `LM_MODS_DIR` to the game's `mods` folder in `mise.local.toml`).

## License

BSD 3-Clause. Copyright (c) 2026 Charles R. Portwood II. See [LICENSE](./LICENSE).
