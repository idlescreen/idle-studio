# idle-studio

Director / job queue for [idle-render](https://github.com/idlescreen/idle-render).

## Usage

```bash
idle-studio enqueue -e beams -o /tmp/a.mkv --duration 10s --dry-run
idle-studio list
idle-studio run
idle-studio run --all
```

Set `IDLE_RENDER` to the `idle-render` binary if it is not on `PATH`.

## License

Apache-2.0.
