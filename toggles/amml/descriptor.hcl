namespace    = "alaydriem"
name         = "toggles"
display_name = "Toggles"
version      = "26.60.5"
author       = "Alaydriem"
config_path  = "config.hcl"

claim "Input" {
  mode   = "Shared"
  reason = "Presses and releases the sneak and forward keys while a toggle is on"
}

keybind "sneak" {
  label       = "Toggle sneak"
  default     = "F3+Shift"
  capability  = "Input"
  scope       = "Global"
  config_path = "keys.sneak"
}

keybind "walk" {
  label       = "Toggle auto-walk"
  default     = "F3+W"
  capability  = "Input"
  scope       = "Global"
  config_path = "keys.walk"
}

section "toggles" {
  title = "Toggles"

  field "toggles.sneak_enabled" {
    label   = "Enable toggle sneak"
    help    = "Lets the sneak toggle key work"
    kind    = "bool"
    default = true
    apply   = "Live"
  }

  field "toggles.walk_enabled" {
    label   = "Enable auto-walk"
    help    = "Lets the auto-walk toggle key work"
    kind    = "bool"
    default = true
    apply   = "Live"
  }

  field "toggles.reset_on_menu" {
    label   = "Reset when a menu opens"
    help    = "Turns both toggles off when a menu or screen opens"
    kind    = "bool"
    default = true
    apply   = "Live"
  }

  field "toggles.reset_on_death" {
    label   = "Reset on death"
    help    = "Turns both toggles off when the player dies"
    kind    = "bool"
    default = true
    apply   = "Live"
  }
}

section "keys" {
  title = "Keys"

  field "keys.sneak" {
    label   = "Toggle sneak"
    help    = "Press this to turn sneak on or off"
    kind    = "chord"
    default = "F3+Shift"
    apply   = "Live"
  }

  field "keys.walk" {
    label   = "Toggle auto-walk"
    help    = "Press this to turn auto-walk on or off"
    kind    = "chord"
    default = "F3+W"
    apply   = "Live"
  }

  field "keys.game_sneak" {
    label   = "Game Sneak key"
    help    = "Must match the Sneak key in Minecraft's controls"
    kind    = "chord"
    default = "Shift"
    apply   = "Live"
  }

  field "keys.game_forward" {
    label   = "Game Forward key"
    help    = "Must match the Forward key in Minecraft's controls"
    kind    = "chord"
    default = "W"
    apply   = "Live"
  }

  field "keys.cancel_walk_1" {
    label   = "Cancel auto-walk key 1"
    help    = "Press this key to stop auto-walk"
    kind    = "chord"
    default = "W"
    apply   = "Live"
  }

  field "keys.cancel_walk_2" {
    label   = "Cancel auto-walk key 2"
    help    = "Press this key to stop auto-walk"
    kind    = "chord"
    default = "S"
    apply   = "Live"
  }
}
