# Final-review fix report

Saved by the controller from the fix agent's hand-back. Results: modmanager-amml check passes, test 1039/1039, header-check passes, build-mod passes. toggles check passes, test 43/43, build-mod passes. Logs are in this folder.

1. **Critical, fixed.** `InputRouter::swallowed(vk)`. A handover now needs the key down and not swallowed (`send_key`, `finish_fake_release`). Tests: `chord_off_while_chord_key_held_queues_the_up`, `release_all_while_swallowed_key_held_queues_the_up`.
2. **Important, fixed.** `ModEvent::Enabled` calls `link.rewatch()`. Test: `enabled_event_rewatches`.
3. **Important, fixed.** New host function `amm_host_release_fake_keys` (not in the ABI table) and `GameCalls::release_fake_keys`. Manager disable releases and flushes before the Tick hooks are removed, only while the game ticks. Tests: `disable_releases_and_flushes_fake_keys_before_removing_hooks`, `disable_sends_no_fake_keys_once_the_game_stopped_ticking`.
4. **Minor, fixed.** `ToggleController::forget(kind)`. A refused down forgets only its own toggle. Tests: `forget_clears_only_that_kind`, `a_refused_walk_down_keeps_sneak_on`.
6. **Minor, fixed.** `SettingsReader::normalise` maps left/right modifiers to the plain code. Test: `lshift_setting_normalises_to_shift`.

Task Completed
