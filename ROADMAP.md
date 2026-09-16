# Roadmap

## Universal DJ Library — Public Roadmap

This roadmap shows the planned development phases. The community can influence priorities through [GitHub Discussions](https://github.com/hellov3an/universal-dj-library/discussions).

### Legend

| Status | Meaning |
|---|---|
| ✅ Stable | Production-ready, well-tested |
| 🟡 Beta | Functional, may have edge cases |
| 🔧 In Progress | Currently being developed |
| 📋 Planned | Scheduled for a future release |
| 🔬 Experimental | Proof of concept, may change significantly |

---

## Phase 0 — Foundation (v0.1.0) 📋

- [x] Tauri 2 + React + TypeScript + Rust project
- [x] SQLite database with migrations
- [x] Dark theme design system
- [x] Basic layout (sidebar, track browser, inspector)
- [ ] CI/CD (GitHub Actions)
- [ ] Open source documentation

## Phase 1 — Library Core (v0.2.0) 📋

- [x] Folder scan and metadata import
- [x] Track browser with virtualized table
- [x] Full-text search (FTS5)
- [x] Field-specific search filters
- [x] Track metadata editor
- [x] Bulk metadata editing
- [x] Tags system
- [x] Playlists with folders
- [x] Smart playlists (rule-based)
- [ ] Drag and drop

## Phase 2 — Audio Analysis (v0.3.0) 📋

- [x] BPM detection
- [x] Key detection (Camelot / Open Key)
- [x] Waveform generation
- [x] Beatgrid detection
- [x] Analysis queue with pause/resume/cancel
- [x] Background workers (non-blocking)

## Phase 3 — Devices & First Adapter (v0.4.0) 📋

- [x] USB / removable device detection
- [x] Hot-plug monitoring
- [x] DJ library detection on devices
- [x] Rekordbox adapter (import from USB)
- [x] Rekordbox adapter (export to USB)
- [x] Transfer engine
- [x] Transfer plan with preview
- [x] File copy with SHA-256 verification
- [x] Transfer report

## Phase 4 — MVP Release (v0.5.0) 📋

- [x] End-to-end Rekordbox transfer flow
- [x] Library health dashboard
- [x] Duplicate detection (hash, metadata)
- [x] Missing file detection and relinking
- [x] Preview audio player
- [x] Cue point editor
- [ ] macOS and Windows installers
- [ ] First public release

## Phase 5 — Additional Adapters (v0.6.0–v0.9.0) 📋

- [x] Serato adapter
- [x] Traktor adapter
- [x] Engine DJ adapter
- [x] Sync engine (bidirectional)
- [x] Backup system
- [x] Undo/redo
- [x] History log
- [ ] Light theme
- [x] Keyboard shortcut customization
- [ ] Generic import/export (JSON, CSV, M3U)

## Phase 6 — Stable Release (v1.0.0) 📋

- [ ] All core adapters at Beta+ status
- [x] Plugin system (dynamic loading)
- [ ] Auto-update system
- [ ] Full E2E test coverage
- [ ] Performance optimization (50k+ tracks)
- [ ] Accessibility audit
- [ ] User documentation

## Future 🔬

- [x] VirtualDJ adapter
- [x] Mixxx adapter
- [x] djay adapter
- [ ] AI auto-tagging (local ONNX models)
- [ ] Audio fingerprint matching
- [ ] Track similarity detection
- [ ] Natural language search
- [ ] Community plugin marketplace
- [ ] Mobile companion app (Tauri mobile)

---

*This roadmap is subject to change based on community feedback and technical feasibility.*
