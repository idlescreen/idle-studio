# idle-studio

Director for [idle-render](https://github.com/idlescreen/idle-render): JSON job
queue, CLI, and interactive TUI. Supports long encodes (`--segment`) and optional
audio beds (`--audio`).

## CLI

```bash
idle-studio enqueue -e beams -o /tmp/a.mkv --duration 8h --segment 1h --audio bed.mp3
idle-studio list
idle-studio run --all
idle-studio tui
```

Set `IDLE_RENDER` if `idle-render` is not on `PATH`.

## TUI keys

| Key | Action |
|-----|--------|
| j / k | Move selection |
| r | Run next pending |
| a | Run all pending |
| R | Reload queue file |
| q | Quit |

## License

Apache-2.0.
