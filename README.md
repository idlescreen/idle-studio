# idle-studio

Director surface for [IdleScreen](https://github.com/idlescreen/idle-core): queue
offline renders, long-form masters, optional music beds, channel presets.

Encoding and simulation belong in [idle-render](https://github.com/idlescreen/idle-render).
This repo is the creative / batch control plane (TUI first).

## Status

**Scaffold.** CLI entrypoint only.

```bash
cargo run -- --tui
# exits 2 until implemented
```

## Planned capabilities

- Pick `saver-*` effect, seed, fps, resolution, duration
- Job queue and overnight batch
- Segmented long encodes (1h chunks → concat)
- Short “reel” presets (social cuts)
- Music / ambient bed mix (later)
- Hand-off to upload checklist

## License

Apache-2.0.
