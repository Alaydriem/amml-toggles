# SDD ledger — plan: docs/superpowers/plans/2026-10-30-toggles.md

Mode: operator override — no per-task reviewers, one final review. No commits. toggles/ is not a git repo.
Game instance: %LOCALAPPDATA%/amml/instances/1.26.60.05-toggles (PID in .superpowers/sdd/2026-10-30-toggles/game.pid)

## Preflight scan
| Tasks | Shared | Finding |
| --- | --- | --- |
| 1 / 5 / 6 | key codes, origin, thread | 5 and 6 read "Proof results"; consistent |
| 2 / 3 | FakeKeyOp, ManagerService API, ModEvent::FakeKeysReleased | consistent |
| 3 / 4 / 5 | AmmApi fields, amm_host_take_fake_keys | consistent |
| 4 / 7 | ManagerLink::watch_keys/send_key, FakeState | consistent |
| 6 / 7 | ToggleInput, TogglesConfig, KeyCommand | consistent |
| 1, 8 | install target | plan says `mise run install-mod` (LM_MODS_DIR = shared 26.51 LL copy) — conflicts with isolation rule |
| each task | self-consistency | ok |

Ruling: every in-game install goes only into %LOCALAPPDATA%/amml/instances/1.26.60.05-toggles/mods/amml (copy dist/ folders there), never LM_MODS_DIR or install-mod — operator isolation rule — cost if wrong: none, shared copies stay untouched.
Ruling: the game is started only by the controller (me), PID saved to game.pid, stopped only by that PID — operator rule — cost if wrong: none.
Ruling: no per-task reviewer; one final review — operator override of SDD — cost if wrong: defects found later, at final review.
Ruling: other mods removed from my instance's mods/amml except alaydriem-mod-manager (and toggles/spike) — isolation of the test — cost if wrong: nothing shared touched.
Task 0: complete (F3 chord parse present; manager check pass, test 982/982 pass)
Instance: 1.26.60.05-toggles created from 1.26.60.05-standalone; mods/amml = alaydriem-mod-manager only
Task 1: complete (gate passed: operator confirmed sneak+walk hold/release; Shift=0x10, W=0x57, origin=1, same thread; spike deleted, log kept in docs/findings/spike)
Game PID 50752 stopped by PID.
Ruling: operator rule mid-Task 2 — fake keys yield to any other active Input lease (Shared or Exclusive): key-down refused; lease activation releases others' fakes + FakeKeysReleased. Sent to the Task 2 implementer; spec+plan updated — cost if wrong: toggles turns off while e.g. a minimap holds Input briefly.
Task 2: complete (check pass; test 1017/1017; deviations in docs/findings/task-2/report.md; abi stopgap: FakeKeysReleased kind + InvalidArgument status already added, header stale -> Task 3)
Note: subagents cannot Write .md reports; controller saves hand-backs.
Task 3: complete (check pass; test 1027/1027; header-check pass; see docs/findings/task-3/report.md)
Task 4: complete (check pass; test 1031/1031)
Task 5: complete (check pass; test 1035/1035; build-mod pass)
Task 6: complete (check pass; test 17/17)
Task 7: complete (check pass; test 39/39; build-mod pass)
Note: manager overlay test frame_flow::a_new_device_on_the_bound_window_rebuilds_the_overlay flaked once under load; rerun 1035/1035 pass. Other agent commits 7066609/9a29ccf/0c35861 landed; they include our amm-core tests fake_keys.rs and key_watch.rs.
Task 8: in progress — manager test 1035/1035 (after 1 flaky overlay rerun), toggles check+test pass, both mods built and installed in my instance; game PID 52096; toggles registered + attached. Final review dispatched (opus) in parallel.
Final review: 1 Critical, 2 Important, 3 Minor (docs/findings/final-review/review.md)
Ruling: finding 5 (watched Key events not protected from overflow) parked — toggles drains 4 batches/tick, a dropped cancel only leaves walk on until the next press — cost if wrong: rare missed cancel.
Ruling: fix findings 1,2,3,4,6 in one fix dispatch — reviewer evidence is concrete — cost if wrong: small extra changes.
Final fixes: complete (manager 1039/1039, header-check, build; toggles 43/43, build). See docs/findings/final-fix/report.md
Ruling: no scoped re-review of the fix wave — operator asked for minimal reviewing; fixes are covered by new failing-first tests — cost if wrong: a fix regression found only in game.
Game restarted with fixed builds: PID 26804 (old 52096 stopped by PID)
Task 8: complete (operator in-game check on fixed build: looks good). Installed manager + toggles into main 1.26.60.05-standalone; old manager backed up to mods-backup/alaydriem-mod-manager-20261030-131218
