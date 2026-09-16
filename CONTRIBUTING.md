# Contributing to Universal DJ Library

Thank you for your interest in contributing! This project is built by the community, for the community.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Environment](#development-environment)
- [Architecture](#architecture)
- [Making Changes](#making-changes)
- [Creating an Adapter](#creating-an-adapter)
- [Testing](#testing)
- [Pull Request Process](#pull-request-process)
- [Commit Convention](#commit-convention)

## Code of Conduct

This project follows the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## Getting Started

### Prerequisites

- **Rust** (stable) — Install via [rustup](https://rustup.rs/)
- **Node.js** (v20+) — Install from [nodejs.org](https://nodejs.org/)
- **pnpm** (v10+) — Install via `npm install -g pnpm`
- **Platform-specific:**
  - **Windows**: [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) runtime
  - **macOS**: Xcode Command Line Tools (`xcode-select --install`)
  - **Linux**: See [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/)

### Setup

```bash
# Clone the repository
git clone https://github.com/user/universal-dj-library.git
cd universal-dj-library

# Install Node dependencies
pnpm install

# Run the development server
pnpm tauri dev
```

### Useful Commands

```bash
# Frontend
pnpm dev          # Start Vite dev server only
pnpm build        # Build frontend
pnpm test         # Run frontend tests
pnpm lint         # Run ESLint
pnpm typecheck    # Run TypeScript type checker
pnpm format       # Format code with Prettier

# Rust
cargo test                    # Run Rust tests
cargo clippy                  # Run linter
cargo fmt                     # Format Rust code
cargo check                   # Fast compilation check

# Full application
pnpm tauri dev                # Run full app in dev mode
pnpm tauri build              # Build production app
```

## Development Environment

### Recommended IDE Setup

- **VS Code** with extensions:
  - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
  - [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
  - [ESLint](https://marketplace.visualstudio.com/items?itemName=dbaeumer.vscode-eslint)
  - [Prettier](https://marketplace.visualstudio.com/items?itemName=esbenp.prettier-vscode)

## Architecture

```
src/                  ← React/TypeScript frontend
src-tauri/src/        ← Rust backend
  commands/           ← Tauri command handlers (thin layer)
  application/        ← Application services (orchestration)
  domain/             ← Domain models and business rules
  infrastructure/     ← Database, filesystem, adapters
```

See [ARCHITECTURE.md](ARCHITECTURE.md) for the full architecture documentation.

### Key Principles

1. **Frontend is a thin presentation layer** — all heavy logic lives in Rust
2. **Layered Rust architecture** — Commands → Services → Domain → Infrastructure
3. **Adapter pattern for DJ software** — each DJ software implements the `DjAdapter` trait
4. **Non-destructive operations** — never modify source data without explicit user consent

## Making Changes

### Branch Naming

```
feat/description     ← New feature
fix/description      ← Bug fix
docs/description     ← Documentation
refactor/description ← Code refactoring
test/description     ← Tests
```

### Workflow

1. Fork the repository
2. Create a feature branch from `main`
3. Make your changes
4. Write/update tests
5. Ensure all checks pass (see [Testing](#testing))
6. Submit a Pull Request

## Creating an Adapter

To add support for a new DJ software:

1. Create a new module under `src-tauri/src/infrastructure/adapters/`
2. Implement the `DjAdapter` trait
3. Register the adapter in `AdapterRegistry`
4. Add test fixtures under `fixtures/`
5. Document the adapter in `docs/adapters/`

```rust
pub trait DjAdapter: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn detect(&self, device: &StorageDevice) -> Result<DetectionResult, AppError>;
    fn scan(&self, device: &StorageDevice) -> Result<DjLibrary, AppError>;
    fn import(&self, source: &DjLibrary) -> Result<UniversalLibrary, AppError>;
    fn export(&self, library: &UniversalLibrary, destination: &StorageDevice, options: &ExportOptions) -> Result<ExportResult, AppError>;
    fn capabilities(&self) -> DjCapabilities;
    fn status(&self) -> AdapterStatus;
}
```

See [docs/adapters/](docs/adapters/) for existing adapter documentation.

## Testing

All changes must include appropriate tests.

### Running Tests

```bash
# All Rust tests
cargo test

# All frontend tests
pnpm test

# Specific Rust test
cargo test test_name

# With output
cargo test -- --nocapture
```

### Test Categories

| Type | Location | Tool |
|---|---|---|
| Rust unit tests | `src-tauri/src/**/tests.rs` | `cargo test` |
| Frontend unit tests | `src/**/*.test.ts(x)` | Vitest |
| E2E tests | `tests/e2e/` | Playwright |

### Test Fixtures

Test fixtures are in `fixtures/`. Never include:
- Real user libraries
- Copyrighted audio files
- Personal data

## Pull Request Process

1. **Title**: Use [Conventional Commits](#commit-convention) format
2. **Description**: Fill out the PR template completely
3. **CI**: All checks must pass
4. **Review**: At least one maintainer approval required
5. **Merge**: Squash merge to `main`

## Commit Convention

This project uses [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add Serato adapter import
fix: correct BPM rounding for Rekordbox export
refactor: extract transfer planning into separate module
docs: add Traktor adapter documentation
test: add round-trip tests for cue point conversion
chore: update dependencies
perf: optimize FTS5 search queries
build: update Tauri to v2.12
ci: add Windows ARM64 build
```

### Scopes (optional)

```
feat(adapters): ...
fix(transfer): ...
refactor(database): ...
docs(api): ...
```

## Code Quality

Before submitting a PR, ensure:

```bash
# Rust
cargo fmt --check       # Code formatting
cargo clippy            # Linting (must pass with no warnings)
cargo test              # All tests pass

# Frontend
pnpm lint               # ESLint (no errors)
pnpm typecheck          # TypeScript (no errors)
pnpm test               # All tests pass
```

## Questions?

- Open a [Discussion](https://github.com/user/universal-dj-library/discussions)
- Check existing [Issues](https://github.com/user/universal-dj-library/issues)

Thank you for contributing! 🎵
