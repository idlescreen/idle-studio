use crate::error::StudioError;
use crate::job::StudioJob;
use std::path::PathBuf;
use std::process::Command;

fn resolve_render_bin() -> Result<PathBuf, StudioError> {
    for key in ["RENDER", "IDLESCREEN_RENDER", "IDLE_RENDER"] {
        if let Ok(p) = std::env::var(key) {
            let pb = PathBuf::from(p);
            if pb.is_file() {
                return Ok(pb);
            }
        }
    }
    for name in ["render", "idle-render"] {
        if let Ok(out) = Command::new("which").arg(name).output() {
            if out.status.success() {
                let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if !s.is_empty() {
                    return Ok(PathBuf::from(s));
                }
            }
        }
    }
    // Sibling cargo target (dev)
    for path in [
        "../render/target/debug/render",
        "../render/target/release/render",
        "target/debug/render",
        "target/release/render",
    ] {
        let p = PathBuf::from(path);
        if p.is_file() {
            return Ok(p);
        }
    }
    Err(StudioError::RenderMissing)
}

/// Spawn `render` for one job; returns stderr/stdout combined message.
pub fn run_job(job: &StudioJob) -> Result<String, StudioError> {
    let bin = resolve_render_bin()?;
    let args = job.to_render_args();
    let output = Command::new(&bin)
        .args(&args)
        .output()
        .map_err(|e| StudioError::Render(format!("spawn {}: {e}", bin.display())))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let msg = format!("{stdout}{stderr}");
    if !output.status.success() {
        return Err(StudioError::Render(msg));
    }
    Ok(msg)
}
