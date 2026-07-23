//! IdleScreen Studio — job queue for `idle-render`.

pub mod error;
pub mod job;
pub mod queue;
pub mod runner;

pub use error::StudioError;
pub use job::StudioJob;
pub use queue::{JobQueue, JobStatus};
pub use runner::run_job;
