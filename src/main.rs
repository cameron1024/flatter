#![warn(clippy::print_stdout, clippy::print_stdout)]

use std::{error::Error, fs::{canonicalize, create_dir_all}, time::Instant};

use args::{Args, SvgRenderJob};
use flexi_logger::LoggerHandle;
use log::info;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use render::render_job;
use structopt::StructOpt;

use crate::render::default_options;

pub mod args;
pub mod paths;
pub mod render;
pub mod yaml;

#[cfg(test)]
mod test;

fn main() {
    let _ = init_log();
    let start = Instant::now();
    let args = Args::from_args();

    let options = default_options();
    let jobs = args.compute_jobs();
    jobs.iter().map(ensure_directory).for_each(Result::unwrap);
    jobs.par_iter().for_each(|job| {
        render_job(job, &options).unwrap();
    });

    info!(
        "Done - rendered {} PNGs to: {}\n(Time taken: {}ms)",
        jobs.len(),
        canonicalize(args.output).unwrap().to_string_lossy(),
        start.elapsed().as_millis()
    )
}

// need to keep handle alive to maintain logger
#[must_use]
fn init_log() -> LoggerHandle {
    flexi_logger::Logger::try_with_str("info")
        .unwrap()
        .start()
        .unwrap()
}

fn ensure_directory(job: &SvgRenderJob) -> Result<(), Box<dyn Error>> {
    if !job.to.exists() {
        create_dir_all(job.to.parent().unwrap())?;
    };
    Ok(())
}
