//! idle-studio — queue and run offline idle-render jobs.

use clap::{Parser, Subcommand};
use idle_studio::job::StudioJob;
use idle_studio::queue::{default_queue_path, JobQueue, JobStatus};
use idle_studio::runner::run_job;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Debug, Parser)]
#[command(
    name = "idle-studio",
    about = "IdleScreen Director — job queue for idle-render"
)]
struct Args {
    /// Queue file (JSON)
    #[arg(long, global = true)]
    queue: Option<PathBuf>,

    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, Subcommand)]
enum Cmd {
    /// Add a job to the queue
    Enqueue {
        #[arg(long, short = 'e')]
        effect: String,
        #[arg(long, short = 'o')]
        output: PathBuf,
        #[arg(long, default_value = "10s")]
        duration: String,
        #[arg(long, default_value_t = 0x00C0_FFEEu64)]
        seed: u64,
        #[arg(long, default_value_t = 30)]
        fps: u32,
        #[arg(long, default_value_t = 1280)]
        width: u32,
        #[arg(long, default_value_t = 720)]
        height: u32,
        #[arg(long)]
        dry_run: bool,
        #[arg(long)]
        id: Option<String>,
    },
    /// List queue entries
    List,
    /// Run the next pending job (or all with --all)
    Run {
        #[arg(long)]
        all: bool,
    },
}

fn main() -> ExitCode {
    let args = Args::parse();
    let path = args.queue.unwrap_or_else(default_queue_path);
    let mut queue = match JobQueue::load(&path) {
        Ok(q) => q,
        Err(e) => {
            eprintln!("idle-studio: {e}");
            return ExitCode::from(2);
        }
    };

    let result = match args.cmd {
        Cmd::Enqueue {
            effect,
            output,
            duration,
            seed,
            fps,
            width,
            height,
            dry_run,
            id,
        } => {
            let id = id.unwrap_or_else(|| format!("job-{}", queue.entries.len() + 1));
            queue.enqueue(StudioJob {
                id: id.clone(),
                effect,
                plugin_path: None,
                seed,
                fps,
                duration,
                output,
                width,
                height,
                dry_run,
            });
            match queue.save(&path) {
                Ok(()) => {
                    eprintln!("enqueued {id} → {}", path.display());
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("idle-studio: {e}");
                    ExitCode::from(1)
                }
            }
        }
        Cmd::List => {
            for (i, e) in queue.entries.iter().enumerate() {
                println!(
                    "[{i}] {} {:?} {} -> {}",
                    e.job.id,
                    e.status,
                    e.job.effect,
                    e.job.output.display()
                );
            }
            ExitCode::SUCCESS
        }
        Cmd::Run { all } => {
            let mut code = ExitCode::SUCCESS;
            loop {
                let Some(idx) = queue.next_pending_index() else {
                    if !all {
                        eprintln!("no pending jobs");
                    }
                    break;
                };
                queue.entries[idx].status = JobStatus::Running;
                let _ = queue.save(&path);
                let job = queue.entries[idx].job.clone();
                match run_job(&job) {
                    Ok(msg) => {
                        queue.entries[idx].status = JobStatus::Done;
                        queue.entries[idx].message = msg;
                        eprintln!("done {}", job.id);
                    }
                    Err(e) => {
                        queue.entries[idx].status = JobStatus::Failed;
                        queue.entries[idx].message = e.to_string();
                        eprintln!("failed {}: {e}", job.id);
                        code = ExitCode::from(1);
                    }
                }
                let _ = queue.save(&path);
                if !all {
                    break;
                }
            }
            code
        }
    };
    result
}
