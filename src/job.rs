use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// One offline render request (mirrors idle-render CLI flags).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StudioJob {
    pub id: String,
    pub effect: String,
    #[serde(default)]
    pub plugin_path: Option<PathBuf>,
    #[serde(default = "default_seed")]
    pub seed: u64,
    #[serde(default = "default_fps")]
    pub fps: u32,
    pub duration: String,
    pub output: PathBuf,
    #[serde(default = "default_w")]
    pub width: u32,
    #[serde(default = "default_h")]
    pub height: u32,
    #[serde(default)]
    pub dry_run: bool,
}

fn default_seed() -> u64 {
    0x00C0_FFEE
}
fn default_fps() -> u32 {
    30
}
fn default_w() -> u32 {
    1280
}
fn default_h() -> u32 {
    720
}

impl StudioJob {
    pub fn to_render_args(&self) -> Vec<String> {
        let mut a = vec![
            "--effect".into(),
            self.effect.clone(),
            "--seed".into(),
            self.seed.to_string(),
            "--fps".into(),
            self.fps.to_string(),
            "--duration".into(),
            self.duration.clone(),
            "--width".into(),
            self.width.to_string(),
            "--height".into(),
            self.height.to_string(),
            "-o".into(),
            self.output.display().to_string(),
        ];
        if let Some(p) = &self.plugin_path {
            a.push("--plugin-path".into());
            a.push(p.display().to_string());
        }
        if self.dry_run {
            a.push("--dry-run".into());
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn args_include_effect_and_output() {
        let j = StudioJob {
            id: "1".into(),
            effect: "beams".into(),
            plugin_path: None,
            seed: 1,
            fps: 30,
            duration: "10s".into(),
            output: PathBuf::from("/tmp/o.mkv"),
            width: 1280,
            height: 720,
            dry_run: true,
        };
        let a = j.to_render_args();
        assert!(a.contains(&"beams".into()));
        assert!(a.iter().any(|x| x.contains("o.mkv")));
        assert!(a.contains(&"--dry-run".into()));
    }
}
