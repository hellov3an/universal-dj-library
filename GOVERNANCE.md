# Governance

## Project Governance

Universal DJ Library is an open-source project governed by its community of contributors and maintainers.

## Roles

### Maintainers

Maintainers have write access to the repository and are responsible for:

- Reviewing and merging pull requests
- Managing releases
- Setting technical direction
- Enforcing the Code of Conduct
- Triaging issues

### Contributors

Anyone who submits a pull request, reports a bug, or participates in discussions is a contributor.

### Users

Anyone who uses the software.

## Decision Making

### Technical Decisions

- **Small changes**: Maintainer discretion
- **Significant changes**: Discussion in GitHub Discussions or issue, consensus among maintainers
- **Breaking changes**: Require RFC-style proposal in Discussions, community feedback period (minimum 7 days)

### Architecture Decision Records (ADR)

Significant architectural decisions are documented in `docs/architecture/decisions/` using ADR format.

## Becoming a Maintainer

Contributors who demonstrate:

1. Sustained, quality contributions over time
2. Deep understanding of the codebase
3. Good judgment in code review
4. Constructive community participation

may be invited to become maintainers by existing maintainers through consensus.

## Code Review

- All changes go through pull requests
- No direct pushes to `main`
- At least one maintainer approval required
- CI must pass before merge
- Squash merge is preferred

## Releases

- Follow [Semantic Versioning](https://semver.org/) (MAJOR.MINOR.PATCH)
- Release candidates for major/minor versions
- Changelog maintained in [CHANGELOG.md](CHANGELOG.md)
- Releases published via GitHub Releases

## Communication

- **GitHub Issues**: Bug reports, feature requests
- **GitHub Discussions**: General questions, ideas, proposals
- **Pull Requests**: Code contributions

## Conflict Resolution

1. Attempt resolution through respectful discussion
2. If unresolved, maintainers decide by consensus
3. If maintainers cannot reach consensus, the project lead makes the final decision

## Amendments

This governance document can be amended through a pull request with approval from a majority of maintainers.
