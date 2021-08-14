use std::path::{Path, PathBuf};

use log::{error, info, warn};

use crate::{
    paths::{find_svgs, find_yaml, rename_svg_to_png},
    yaml::RenderYaml,
};

#[derive(structopt::StructOpt, Debug)]
pub struct Args {
    #[structopt(
        short,
        long,
        parse(from_os_str),
        help = "Path to the input directory, or a single SVG file"
    )]
    pub output: PathBuf,

    #[structopt(
        short,
        long,
        parse(from_os_str),
        help = "Path to the output directory, or a single PNG file"
    )]
    pub input: PathBuf,

    #[structopt(short, long, help = "List of scale factors to render the PNGs at")]
    pub scales: Option<Vec<u32>>,

    #[structopt(
        short,
        long,
        help = "Number of threads to use for rendering (defaults to the number of logical CPU cores)"
    )]
    pub threads: Option<usize>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct SvgRenderJob {
    pub from: PathBuf,
    pub to: PathBuf,
    pub scale: u32,
}

impl Args {
    pub fn compute_jobs(&self) -> Vec<SvgRenderJob> {
        let yaml = if let Ok(Some(yaml)) = find_yaml(self.input.clone()) {
            info!(
                "Found config file: {}",
                self.input.join("render.yaml").to_string_lossy()
            );
            yaml
        } else {
            info!("No render.yaml found, falling back to defaults");
            RenderYaml::default()
        };
        

        let scales = self.scales.to_owned().unwrap_or(yaml.scales);
        let threads = self.threads.or(yaml.threads);

        if let Some(t) = threads {
            std::env::set_var("RAYON_NUM_THREADS", format!("{}", t));
        };
        info!(
            "using {} render threads",
            match self.threads {
                Some(threads) => threads,
                None => num_cpus::get(),
            }
        );

        let svgs = find_svgs(self.input.to_owned());
        let svg_scales = svgs
            .iter()
            .flat_map(|svg| scales.iter().map(move |scale| (svg, *scale)));

        if self.input.is_dir() && self.output.is_file() {
            warn!(
                "attempting to write directory to single file:\n\tinput directory: {}\n\toutput file: {}",
                self.input.to_path_buf().to_string_lossy(),
                self.output.to_path_buf().to_string_lossy(),
            );
        };
        if self.output.is_dir() {
            // if output is a directory, we can always write any number of svgs

            svg_scales
                .map(|(input, scale)| map_single(input, &self.output, scale))
                .collect::<Vec<_>>()
        } else {
            let svg_scales: Vec<_> = svg_scales.collect();
            match svg_scales.len() {
                0 => vec![],
                1 => {
                    let (input, scale) = svg_scales.first().unwrap();
                    vec![map_single(input, &self.output, *scale)]
                }
                _ => {
                    error!("ERROR: attempted to write multiple SVGs to single file:\n\tCurrent args: {:#?}", &self);
                    panic!()
                }
            }
        }
    }
}

fn map_single(input: &Path, output: &Path, scale: u32) -> SvgRenderJob {
    let temp = rename_svg_to_png(input.to_owned());
    let new_filename = temp.file_name().unwrap();
    let output = if output.is_dir() {
        output.join(new_filename)
    } else {
        output.to_path_buf()
    };

    let output = if scale != 1 {
        let parent = output.parent().unwrap();
        parent
            .join(format!("{}.0x", scale))
            .join(output.file_name().unwrap())
    } else {
        output
    };

    SvgRenderJob {
        from: input.to_owned(),
        to: output,
        scale,
    }
}

#[cfg(test)]
mod test {
    use crate::test::test_asset;

    use super::*;

    #[test]
    #[should_panic]
    fn compute_jobs_should_panic_if_dir_to_file_mapping() {
        Args {
            input: test_asset("multiple_svgs"), // directory
            output: test_asset("no_extension"), // file
            scales: Some(vec![1]),
            threads: None,
        }
        .compute_jobs();
    }

    #[test]
    fn compute_jobs_file_to_file() {
        let jobs = Args {
            input: test_asset("example.svg"),
            output: test_asset("another_file.doc"),
            scales: Some(vec![1]),
            threads: None,
        }
        .compute_jobs();
        assert_eq!(
            jobs,
            vec![SvgRenderJob {
                from: test_asset("example.svg"),
                to: test_asset("another_file.doc"),
                scale: 1,
            }]
        );
    }

    #[test]
    fn compute_jobs_file_to_dir() {
        let jobs = Args {
            input: test_asset("example.svg"),
            output: test_asset("multiple_svgs"),
            scales: Some(vec![1]),
            threads: None,
        }
        .compute_jobs();
        assert_eq!(
            jobs,
            vec![SvgRenderJob {
                from: test_asset("example.svg"),
                to: test_asset("multiple_svgs/example.png"),
                scale: 1,
            }]
        );
    }

    #[test]
    fn compute_jobs_dir_to_dir() {
        let jobs = Args {
            input: test_asset("multiple_svgs"),
            output: test_asset("empty"),
            scales: Some(vec![1]),
            threads: None,
        }
        .compute_jobs();
        assert_eq!(
            jobs,
            vec![
                SvgRenderJob {
                    from: test_asset("multiple_svgs/example2.svg"),
                    to: test_asset("empty/example2.png"),
                    scale: 1,
                },
                SvgRenderJob {
                    from: test_asset("multiple_svgs/example.svg"),
                    to: test_asset("empty/example.png"),
                    scale: 1,
                },
            ]
        )
    }

    #[test]
    fn compute_jobs_dir_to_dir_multiple_scales() {
        let jobs = Args {
            input: test_asset("multiple_svgs"),
            output: test_asset("empty"),
            scales: Some(vec![1, 2]),
            threads: None,
        }
        .compute_jobs();
        assert_eq!(
            jobs,
            vec![
                SvgRenderJob {
                    from: test_asset("multiple_svgs/example2.svg"),
                    to: test_asset("empty/example2.png"),
                    scale: 1,
                },
                SvgRenderJob {
                    from: test_asset("multiple_svgs/example2.svg"),
                    to: test_asset("empty/2.0x/example2.png"),
                    scale: 2,
                },
                SvgRenderJob {
                    from: test_asset("multiple_svgs/example.svg"),
                    to: test_asset("empty/example.png"),
                    scale: 1,
                },
                SvgRenderJob {
                    from: test_asset("multiple_svgs/example.svg"),
                    to: test_asset("empty/2.0x/example.png"),
                    scale: 2,
                },
            ]
        )
    }

    #[test]
    fn compute_jobs_uses_scales_from_yaml_if_args_not_provided() {
        let jobs = Args {
            input: test_asset("has_render_yaml"),
            output: test_asset("empty"),
            scales: None,
            threads: None,
        }
        .compute_jobs();
        assert_eq!(
            jobs,
            vec![
                SvgRenderJob {
                    from: test_asset("has_render_yaml/example.svg"),
                    to: test_asset("empty/example.png"),
                    scale: 1
                },
                SvgRenderJob {
                    from: test_asset("has_render_yaml/example.svg"),
                    to: test_asset("empty/2.0x/example.png"),
                    scale: 2
                },
                SvgRenderJob {
                    from: test_asset("has_render_yaml/example.svg"),
                    to: test_asset("empty/3.0x/example.png"),
                    scale: 3
                },
            ]
        );
    }

    #[test]
    fn compute_jobs_uses_scales_arg_over_yaml() {
        let jobs = Args {
            input: test_asset("has_render_yaml"),
            output: test_asset("empty"),
            scales: Some(vec![1]),
            threads: None,
        }
        .compute_jobs();
        assert_eq!(
            jobs,
            vec![SvgRenderJob {
                from: test_asset("has_render_yaml/example.svg"),
                to: test_asset("empty/example.png"),
                scale: 1
            },]
        );
    }
}
