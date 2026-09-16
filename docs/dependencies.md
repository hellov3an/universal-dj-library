# Dependencies

## Rust Dependencies

| Crate | Version | Purpose | License |
|---|---|---|---|
| `tauri` | ^2 | Desktop application framework | MIT/Apache-2.0 |
| `tauri-plugin-opener` | ^2 | Open URLs and files natively | MIT/Apache-2.0 |
| `tauri-plugin-dialog` | ^2 | Native file/folder dialogs | MIT/Apache-2.0 |
| `tauri-plugin-log` | ^2 | Structured logging | MIT/Apache-2.0 |
| `serde` | ^1 | Serialization/deserialization | MIT/Apache-2.0 |
| `serde_json` | ^1 | JSON support | MIT/Apache-2.0 |
| `rusqlite` | ^0.32 | SQLite bindings (bundled) | MIT |
| `tokio` | ^1 | Async runtime | MIT |
| `thiserror` | ^2 | Error derive macros | MIT/Apache-2.0 |
| `uuid` | ^1 | UUID generation (v4) | MIT/Apache-2.0 |
| `chrono` | ^0.4 | Date/time handling | MIT/Apache-2.0 |
| `walkdir` | ^2 | Directory traversal | MIT/Unlicense |
| `sha2` | ^0.10 | SHA-256 hashing | MIT/Apache-2.0 |
| `log` | ^0.4 | Logging facade | MIT/Apache-2.0 |

## Frontend Dependencies

| Package | Version | Purpose | License |
|---|---|---|---|
| `react` | ^19.1 | UI framework | MIT |
| `react-dom` | ^19.1 | React DOM renderer | MIT |
| `@tauri-apps/api` | ^2 | Tauri IPC bridge | MIT/Apache-2.0 |
| `@tauri-apps/plugin-dialog` | ^2 | Dialog bindings | MIT/Apache-2.0 |
| `@tauri-apps/plugin-log` | ^2 | Logging bindings | MIT/Apache-2.0 |
| `@tauri-apps/plugin-opener` | ^2 | Opener bindings | MIT/Apache-2.0 |
| `zustand` | ^5 | State management | MIT |

## Dev Dependencies

| Package | Version | Purpose | License |
|---|---|---|---|
| `typescript` | ^6 | Type system | Apache-2.0 |
| `vite` | ^8 | Build tool | MIT |
| `vitest` | ^3 | Test runner | MIT |
| `@testing-library/react` | ^16 | React test utilities | MIT |
| `eslint` | ^9 | Linter | MIT |
| `prettier` | ^3 | Code formatter | MIT |
| `@tauri-apps/cli` | ^2 | Tauri CLI | MIT/Apache-2.0 |

## License Compatibility

All current dependencies are compatible with the project's **MIT license**.

### Potential Future Concerns

| Crate | License | Risk |
|---|---|---|
| `symphonia` (planned) | MPL-2.0 | File-level copyleft, OK with MIT at project level |
| `rekordcrate` | GPL-3.0 | Would require GPL for entire project — **will NOT use** |
| `triseratops` | GPL-3.0 | Would require GPL for entire project — **will NOT use** |

The project will implement its own format parsers for DJ software databases to maintain MIT licensing.
