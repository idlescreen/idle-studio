#[cfg(test)]
mod job_props {
    use crate::job::StudioJob;
    use proptest::prelude::*;
    use std::path::PathBuf;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(32))]

        #[test]
        fn args_always_include_effect_and_output(
            effect in "[a-z]{3,12}",
            seed: u64,
            fps in 1u32..=120,
        ) {
            let j = StudioJob {
                id: "t".into(),
                effect: effect.clone(),
                plugin_path: None,
                seed,
                fps,
                duration: "10s".into(),
                output: PathBuf::from("/tmp/out.mkv"),
                width: 1280,
                height: 720,
                dry_run: true,
                segment: None,
                audio: None,
            };
            let a = j.to_render_args();
            prop_assert!(a.iter().any(|x| x == &effect));
            prop_assert!(a.iter().any(|x| x.contains("out.mkv")));
            prop_assert!(a.contains(&"--dry-run".into()));
        }
    }
}
