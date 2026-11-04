namespace   = "alaydriem"
name        = "toggles"
entry       = "alaydriem_toggles.dll"
api_version = 2
entries = [
  "ClientInstance.update",
  "ClientInstance.getMouseGrabbed",
  "ClientInstance.getLocalPlayer",
  "Actor.isAlive",
  "MinecraftGame.onAppFocusLost",
  "ClientExitLevel.hook",
]

listing {
  title       = "Toggles"
  summary     = "Toggle sneak and auto-walk without holding a key."
  description = <<-EOT
    Toggle sneak and walking with a single key for a hands-free experience.
  EOT
  authors     = ["Charles R. Portwood II"]
  license     = "BSD-3-Clause"
  source      = "https://github.com/alaydriem/amml-toggles"
  homepage    = "https://github.com/alaydriem/amml-toggles#readme"
  tags        = ["sneak", "auto-walk", "movement", "client", "utility"]
  icon        = "icon.png"
}
