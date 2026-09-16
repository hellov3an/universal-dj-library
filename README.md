# Universal DJ Library

> **Your music belongs to you — not to your DJ software.**

[![CI](https://github.com/user/universal-dj-library/actions/workflows/ci.yml/badge.svg)](https://github.com/user/universal-dj-library/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

---

**Open source. Free forever. Local-first. DJ software agnostic.**

Universal DJ Library is a free, open-source desktop application that lets DJs manage a single master music library and transfer it between different DJ software, computers, and USB drives.

```text
                    UNIVERSAL DJ LIBRARY
                           │
             ┌─────────────┼─────────────┐
             │             │             │
             ▼             ▼             ▼
        Rekordbox       Serato       Traktor
             │             │             │
             ▼             ▼             ▼
          USB A          USB B          SSD
```

## Features

- ✅ **Universal DJ Library** — One library to rule them all
- ✅ **USB Transfer** — Copy your library to any USB drive
- ✅ **Cross-DJ Migration** — Move between Rekordbox, Serato, Traktor, and more
- ✅ **Metadata Management** — Title, artist, album, genre, year, comments
- ✅ **Cue Points** — Hot cues, memory cues with colors and labels
- ✅ **Beatgrids** — Preserve and edit beat grids
- ✅ **BPM & Key** — Detection and management (Camelot, Open Key, standard)
- ✅ **Playlists** — Hierarchical playlists with folders
- ✅ **Smart Playlists** — Rule-based dynamic playlists
- ✅ **Duplicate Detection** — Find duplicates by hash, metadata, or audio fingerprint
- ✅ **Device Synchronization** — Keep USB drives in sync with your library
- ✅ **Library Backup** — Automatic and manual backups
- ✅ **Audio Analysis** — BPM, key, waveform, energy detection
- ✅ **Bulk Editing** — Edit thousands of tracks at once
- ✅ **Search** — Fast full-text and field-specific search

## Philosophy

| Principle | What it means |
|---|---|
| **Free** | All features are free. No subscriptions, no paywalls, no limits |
| **Open Source** | MIT licensed. Code is public. Contributions welcome |
| **Local-first** | Your data stays on your machine. No cloud required |
| **Offline-first** | Works without internet |
| **Privacy-first** | No account required. No tracking. No telemetry by default |
| **Non-destructive** | Never modifies or deletes data without explicit confirmation |
| **DJ-first** | Designed for real DJ workflows |

## Killer Feature: Universal Transfer

Transfer your complete library between DJ software:

```text
USB A                          USB B
Serato         ──────►         Rekordbox

Tracks          ✓              Tracks
Playlists       ✓              Playlists
Cue Points      ✓              Cue Points
BPM             ✓              BPM
Key             ✓              Key
Beatgrids       ✓              Beatgrids
Artwork         ✓              Artwork
```

No data is silently lost. Incompatibilities are reported before transfer.

## Screenshots

*Coming soon*

## Download

*Coming soon — builds for macOS and Windows*

## Tech Stack

| Component | Technology |
|---|---|
| Desktop Framework | [Tauri 2](https://tauri.app/) |
| Backend | Rust |
| Frontend | React + TypeScript |
| Build Tool | Vite |
| Database | SQLite |
| State Management | Zustand |
| Package Manager | pnpm |

## Development

### Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Node.js](https://nodejs.org/) (v20+)
- [pnpm](https://pnpm.io/) (v10+)
- Platform-specific requirements:
  - **Windows**: [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) (pre-installed on Windows 11)
  - **macOS**: Xcode Command Line Tools (`xcode-select --install`)

### Quick Start

```bash
# Clone the repository
git clone https://github.com/user/universal-dj-library.git
cd universal-dj-library

# Install dependencies
pnpm install

# Run in development mode
pnpm tauri dev

# Run tests
cargo test
pnpm test

# Build for production
pnpm tauri build
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed development setup.

## Architecture

```text
┌───────────────────────────────────────────┐
│                 TAURI 2                   │
│                                           │
│  ┌─────────────────────────────────────┐  │
│  │        React / TypeScript           │  │
│  │        UI · Library · Search        │  │
│  │     Playlists · Waveforms · Devices │  │
│  └─────────────────────────────────────┘  │
│                    │                      │
│             Tauri Commands                │
│                    │                      │
│  ┌─────────────────────────────────────┐  │
│  │              RUST                   │  │
│  │     Database · Filesystem · Audio   │  │
│  │   Devices · Transfer · Adapters     │  │
│  └─────────────────────────────────────┘  │
└───────────────────────────────────────────┘
```

See [ARCHITECTURE.md](ARCHITECTURE.md) for the full technical architecture.

## Adapter Status

| DJ Software | Import | Export | USB | Status |
|---|---|---|---|---|
| Rekordbox | 🟡 | 🟡 | 🟡 | Alpha |
| Serato | ⬜ | ⬜ | ⬜ | Planned |
| Traktor | ⬜ | ⬜ | ⬜ | Planned |
| Engine DJ | ⬜ | ⬜ | ⬜ | Planned |
| VirtualDJ | ⬜ | ⬜ | ⬜ | Planned |
| Mixxx | ⬜ | ⬜ | ⬜ | Planned |

🟢 Stable · 🟡 Alpha/Beta · 🔴 Experimental · ⬜ Planned

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for:

- Development setup
- Architecture overview
- How to create an adapter
- Testing guidelines
- Pull request process

## Community

- [GitHub Discussions](https://github.com/user/universal-dj-library/discussions) — Ask questions, share ideas
- [Issues](https://github.com/user/universal-dj-library/issues) — Report bugs, request features
- [Roadmap](ROADMAP.md) — See what's planned

## License

[MIT](LICENSE) — Free to use, modify, and distribute.

## Acknowledgements

This project is built on the shoulders of giants:

- [Tauri](https://tauri.app/) — Desktop application framework
- [Deep Symmetry](https://djl-analysis.deepsymmetry.org/) — Rekordbox format documentation
- [Mixxx](https://mixxx.org/) — Open source DJ software and format documentation
- [symphonia](https://github.com/pdeljanov/Symphonia) — Rust audio decoding

---

**Universal DJ Library** — *Your music. Your library. Your choice.*
