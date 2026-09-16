# Design System

## Universal DJ Library Design System

### Philosophy

The design system is built for **professional DJ workflows**: dense, data-rich, keyboard-first, and fast. Every pixel serves the DJ.

### Colors

#### Dark Theme (Default)

| Token | Value | Use |
|---|---|---|
| `--color-bg-primary` | `#0d0d0d` | Main background |
| `--color-bg-secondary` | `#141414` | Sidebar, inspector |
| `--color-bg-tertiary` | `#1a1a1a` | Input backgrounds |
| `--color-bg-elevated` | `#222222` | Cards, elevated surfaces |
| `--color-bg-hover` | `#2a2a2a` | Hover states |
| `--color-accent` | `#00d4ff` | Primary accent (cyan) |
| `--color-success` | `#34d399` | Success states |
| `--color-warning` | `#fbbf24` | Warnings |
| `--color-error` | `#f87171` | Errors |

#### DJ-Specific Colors

| Token | Value | Use |
|---|---|---|
| `--color-cue-hot` | `#ff3b5c` | Hot cue markers |
| `--color-cue-memory` | `#00d4ff` | Memory cue markers |
| `--color-loop` | `#a855f7` | Loop regions |

### Typography

| Token | Size | Use |
|---|---|---|
| `--font-size-xs` | 11px | Section headers, badges |
| `--font-size-sm` | 12px | Table cells, sidebar items |
| `--font-size-base` | 13px | Body text |
| `--font-size-md` | 14px | Section titles |
| `--font-size-lg` | 16px | Page titles |

- **Primary font**: Inter — clean, highly legible at small sizes
- **Mono font**: JetBrains Mono — for BPM, key, timecodes, durations

### Layout

```text
┌─────────────────────────────────────────────────────┐
│                    TOOLBAR (48px)                    │
├──────────────┬──────────────────────────────────────┤
│              │                                      │
│ SIDEBAR      │          TRACK BROWSER               │
│ (240px)      │          (flex: 1)                   │
│              │                                      │
├──────────────┴──────────────────────────────────────┤
│              INSPECTOR (260px)                       │
└─────────────────────────────────────────────────────┘
```

### Components

#### Button Variants

- `btn--primary` — Accent color, primary actions
- `btn--secondary` — Outlined, secondary actions
- `btn--ghost` — Text only, tertiary actions

#### Badges

- `badge--success` — ✓ Up to date
- `badge--warning` — ⚠ Changes detected
- `badge--error` — ✗ Error
- `badge--info` — ℹ Information

### Accessibility

- All interactive elements support keyboard navigation
- Focus visible outlines use accent color
- Minimum contrast ratios maintained
- ARIA labels on navigation elements
- Semantic HTML structure
