use std::{error::Error, fs::{canonicalize, create_dir_all}, time::SystemTime};

use args::{Args, SvgRenderJob};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use render::render_job;
use structopt::StructOpt;

mod args;
mod paths;

mod render;

#[cfg(test)]
mod test;

fn main() {
    let start = SystemTime::now();
    let args = Args::from_args();
    let jobs = args.compute_jobs();
    jobs.iter().map(ensure_directory).for_each(Result::unwrap);
    jobs.par_iter().for_each(|job| {
        println!(
            "Rendering (Scale: {}x): {}",
            job.scale,
            job.from.to_string_lossy(),
        );
        render_job(job).unwrap();
    });
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!(
        "Done - rendered {} PNGs to: {}\n(Time taken: {}ms)",
        jobs.len(),
        canonicalize(args.output).unwrap().to_string_lossy(),
        duration.as_millis()
    )
}

fn ensure_directory(job: &SvgRenderJob) -> Result<(), Box<dyn Error>> {
    if !job.to.exists() {
        create_dir_all(job.to.parent().unwrap())?;
    };
    Ok(())
}
