# app-studio

Director surface for IdleScreen offline export. Queues render jobs for
render, exposes a CLI, and provides an interactive terminal UI for overnight
batches.

## Capabilities

- JSON job queue under the user config directory (or `--queue`)
- Enqueue with duration, seed, resolution, optional segment length and audio bed
- Run next job or drain the queue
- TUI list with keyboard controls

## Build

```bash
git clone https://github.com/idlescreen/app-studio.git
cd app-studio
cargo build --release
```

Set `IDLESCREEN_RENDER` to the render binary when it is not on `PATH`.

## CLI

```bash
app-studio enqueue -e beams -o /tmp/a.mkv --duration 8h --segment 1h
app-studio list
app-studio run --all
app-studio tui
```

## TUI keys

| Key | Action |
|-----|--------|
| j / k | Move selection |
| r | Run next pending job |
| a | Run all pending jobs |
| R | Reload queue from disk |
| q | Quit |

## Related

| Project | Role |
|---------|------|
| render | Simulation and encode engine |
| idle-core | Live daemon and plugin API |
| packages | Package host |

## License

Apache-2.0.
