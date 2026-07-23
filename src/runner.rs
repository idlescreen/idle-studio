use crate::error::StudioError;
use crate::job::StudioJob;
use std::path::PathBuf;
use std::process::Command;

fn resolve_render_bin() -> Result<PathBuf, StudioError> {
    if let Ok(p) = std::env::var("IDLE_RENDER") {
        let pb = PathBuf::from(p);
        if pb.is_file() {
            return Ok(pb);
        }
    }
    if let Ok(out) = Command::new("which").arg("idle-render").output() {
        if out.status.success() {
            let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !s.is_empty() {
                return Ok(PathBuf::from(s));
            }
        }
    }
    // Sibling cargo target (dev)
    let dev = PathBuf::from("../idle-render/target/debug/idle-render");
    if dev.is_file() {
        return Ok(dev);
    }
    let dev_rel = PathBuf::from("target/debug/idle-render");
    if dev_rel.is_file() {
        return Ok(dev_rel);
    }
    Err(StudioError::RenderMissing)
}

/// Spawn `idle-render` for one job; returns stderr/stdout combined message.
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
