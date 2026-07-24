use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// One offline render request (mirrors render CLI flags).
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
    #[serde(default)]
    pub segment: Option<String>,
    #[serde(default)]
    pub audio: Option<PathBuf>,
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
        if let Some(s) = &self.segment {
            a.push("--segment".into());
            a.push(s.clone());
        }
        if let Some(audio) = &self.audio {
            a.push("--audio".into());
            a.push(audio.display().to_string());
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
    fn args_include_segment_and_audio() {
        let j = StudioJob {
            id: "1".into(),
            effect: "beams".into(),
            plugin_path: None,
            seed: 1,
            fps: 30,
            duration: "2h".into(),
            output: PathBuf::from("/tmp/o.mkv"),
            width: 1280,
            height: 720,
            dry_run: false,
            segment: Some("1h".into()),
            audio: Some(PathBuf::from("/tmp/bed.mp3")),
        };
        let a = j.to_render_args();
        assert!(a.contains(&"--segment".into()));
        assert!(a.contains(&"1h".into()));
        assert!(a.contains(&"--audio".into()));
    }
}
