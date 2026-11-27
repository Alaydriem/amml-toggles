namespace    = "alaydriem"
name         = "toggles"
entry        = "alaydriem_toggles.dll"
api_version  = 3

uses {
  feature "toggles" {
    event "client.tick" {
      version  = 1
      required = true
    }
    event "input.key" {
      version  = 1
      required = true
    }
    event "input.injected_released" {
      version  = 2
      required = true
    }
    event "input.lease_lost" {
      version  = 1
      required = true
    }
    event "registry.provider_ready" {
      version  = 1
      required = true
    }
    event "registry.provider_gone" {
      version  = 1
      required = true
    }
    event "mods.loaded" {
      version  = 1
      required = true
    }
    event "input.lease_suspended" {
      version  = 1
      required = true
    }
    event "input.lease_resumed" {
      version  = 1
      required = true
    }
    area "input.lease" {
      version  = 1
      required = true
    }
    area "input.key_inject" {
      version  = 1
      required = true
    }
    area "client.mouse" {
      version  = 1
      required = true
    }
    event "overlay.screenshot_taken" {
      version  = 1
      required = false
    }
    event "overlay.screenshot_written" {
      version  = 1
      required = false
    }
  }
  feature "reset" {
    event "window.focus" {
      version  = 1
      required = true
    }
    event "world.left" {
      version  = 1
      required = true
    }
  }
  feature "death" {
    area "client.read" {
      version  = 1
      required = true
    }
    area "actor.read" {
      version  = 1
      required = true
    }
  }
}

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
