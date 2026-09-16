# Architecture

> Technical architecture of Universal DJ Library

## Overview

Universal DJ Library is built with [Tauri 2](https://tauri.app/), combining a React/TypeScript frontend with a Rust backend. The application follows a layered architecture with clear separation of concerns.

```text
┌───────────────────────────────────────────────────────┐
│                      TAURI 2                          │
│                                                       │
│  ┌─────────────────────────────────────────────────┐  │
│  │           React / TypeScript (WebView)          │  │
│  │                                                 │  │
│  │  Presentation Layer                             │  │
│  │  ├── Design System (CSS custom properties)      │  │
│  │  ├── Components (atomic UI components)          │  │
│  │  ├── Features (library, playlists, transfer)    │  │
│  │  ├── Stores (Zustand state management)          │  │
│  │  └── Services (typed IPC wrappers)              │  │
│  └──────────────────────┬──────────────────────────┘  │
│                         │ IPC (invoke / events)       │
│  ┌──────────────────────▼──────────────────────────┐  │
│  │                  Rust Core                      │  │
│  │                                                 │  │
│  │  Commands Layer ─── Tauri #[command] handlers   │  │
│  │       │                                         │  │
│  │  Application Layer ─── Service orchestration    │  │
│  │       │                                         │  │
│  │  Domain Layer ─── Models, business rules        │  │
│  │       │                                         │  │
│  │  Infrastructure Layer                           │  │
│  │  ├── Database (SQLite / rusqlite)               │  │
│  │  ├── Filesystem (scan, copy, hash)              │  │
│  │  ├── Audio (decode, analyze)                    │  │
│  │  ├── Devices (detect, monitor)                  │  │
│  │  └── Adapters (Rekordbox, Serato, Traktor...)   │  │
│  └─────────────────────────────────────────────────┘  │
└───────────────────────────────────────────────────────┘
```

## Rust Backend

### Layer Responsibilities

| Layer | Directory | Responsibility |
|---|---|---|
| **Commands** | `src-tauri/src/commands/` | Thin Tauri command handlers. Deserialize arguments, call services, serialize responses |
| **Application** | `src-tauri/src/application/` | Orchestrate domain operations. Transaction boundaries. Event emission |
| **Domain** | `src-tauri/src/domain/` | Pure business logic. Domain models. No I/O dependencies |
| **Infrastructure** | `src-tauri/src/infrastructure/` | Concrete implementations: database, filesystem, audio, devices, adapters |

### Data Flow

```text
Frontend (invoke)
    │
    ▼
Command Handler          ← Validates input, calls service
    │
    ▼
Application Service      ← Orchestrates operations, emits events
    │
    ▼
Domain Logic             ← Business rules, model transformations
    │
    ▼
Infrastructure           ← Database queries, file I/O, device access
```

### Concurrency

| Runtime | Use |
|---|---|
| **Tokio** | I/O-bound: file copy, device monitoring, database (via `spawn_blocking`) |
| **Rayon** | CPU-bound: audio analysis, file hashing, bulk metadata processing |

## Frontend

### Structure

```text
src/
├── app/              ← Root layout, routes, providers
├── components/
│   ├── ui/           ← Atomic design system (Button, Input, Table)
│   ├── layout/       ← Sidebar, Toolbar, Browser, Inspector
│   └── shared/       ← Waveform, TrackRow, SearchBar
├── features/         ← Feature modules with views + stores
├── hooks/            ← Reusable React hooks
├── services/         ← Typed IPC wrappers (invoke calls)
├── stores/           ← Global Zustand stores
├── types/            ← TypeScript type definitions
└── design-system/    ← CSS tokens, themes
```

### State Management

Each feature has an isolated [Zustand](https://github.com/pmndrs/zustand) store. Stores contain both state and actions, communicating with the Rust backend via typed service calls.

## Database

- **Engine**: SQLite via `rusqlite`
- **Mode**: WAL (Write-Ahead Logging) for concurrent read performance
- **Migrations**: Versioned SQL files, applied at startup via `PRAGMA user_version`
- **Search**: FTS5 virtual table for fast full-text search

## Universal DJ Model

The core data model is DJ-software agnostic. Every adapter converts to/from this model.

```text
UniversalLibrary
├── Track
│   ├── FileIdentity (hash, fingerprint, duration, size)
│   ├── TrackMetadata (title, artist, album, genre, ...)
│   ├── CuePoints (hot cues, memory cues)
│   ├── Loops
│   ├── Beatgrid
│   └── AnalysisData (BPM, key, waveform, energy)
├── Playlist
│   ├── Regular playlists (ordered track lists)
│   ├── Smart playlists (rule-based)
│   └── Playlist folders (hierarchy)
└── Tags
```

## Adapter System

Each DJ software is supported through an adapter implementing the `DjAdapter` trait:

```text
AdapterRegistry
├── RekordboxAdapter
├── SeratoAdapter
├── TraktorAdapter
├── EngineDjAdapter
└── ...
```

Adapters expose `DjCapabilities` declaring what they can import/export.

## Transfer Engine

```text
Source → Scan → Import → Map → Compare → Plan → Review → Copy → Verify → Export → Validate → Report
```

All transfers go through a user review step before any files are modified.

## Security

- Tauri capabilities with minimal permissions
- No arbitrary filesystem access from frontend
- All IPC inputs validated in Rust
- Strict Content Security Policy
