use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
};

use resvg::ScreenSize;
use tiny_skia::Pixmap;
use usvg::{FitTo, Options, SystemFontDB};

use crate::args::SvgRenderJob;

pub fn render_job(job: &SvgRenderJob) -> Result<(), Box<dyn Error>> {
    let svg_in = File::open(&job.from)?;
    let png_out = File::create(&job.to)?;
    render(svg_in, png_out, job.scale)
}

pub fn render(
    mut svg_in: impl Read,
    mut png_out: impl Write,
    scale: u32,
) -> Result<(), Box<dyn Error>> {
    let mut svg_bytes = vec![];
    svg_in.read_to_end(&mut svg_bytes)?;

    let mut opt = usvg::Options::default();
    opt.fontdb.load_system_fonts();
    opt.fontdb.set_generic_families();

    let render_tree = usvg::Tree::from_data(&svg_bytes, &Options::default())?;

    let pixmap_size = scale_size(render_tree.svg_node().size.to_screen_size(), scale);

    let mut pixmap = Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    resvg::render(
        &render_tree,
        FitTo::Size(pixmap_size.height(), pixmap_size.width()),
        pixmap.as_mut(),
    )
    .unwrap();
    let png_bytes = pixmap.encode_png()?;

    png_out.write_all(&png_bytes)?;

    Ok(())
}

fn scale_size(size: ScreenSize, scale: u32) -> ScreenSize {
    ScreenSize::new(size.width() * scale, size.height() * scale).unwrap()
}

#[cfg(test)]
mod test {
    use std::fs::read;

    use crate::test::test_asset;

    use super::*;

    #[test]
    fn scale_size_test() {
        assert_eq!(
            scale_size(ScreenSize::new(10, 10).unwrap(), 2),
            ScreenSize::new(20, 20).unwrap()
        );
        assert_eq!(
            scale_size(ScreenSize::new(25, 40).unwrap(), 3),
            ScreenSize::new(75, 120).unwrap()
        );
    }

    #[test]
    fn should_render_pngs() {
        let real_svg = test_asset("real_data/example.svg");
        let output = test_asset("real_data/example.png");
        if let Ok(mut file) = File::create(&output) {
            file.write_all(&[0; 0]).unwrap();
        }
        render_job(&SvgRenderJob {
            from: real_svg,
            to: output.clone(),
            scale: 1,
        })
        .unwrap();

        assert_eq!(
            read(output).unwrap(),
            read(test_asset("real_data/reference.png")).unwrap()
        );
    }

    #[test]
    fn should_produce_same_output_reliably() {
        let svg = read(test_asset("real_data/example.svg")).unwrap();
        let mut buffers = (0..10).map(|_| vec![]);
        for buffer in &mut buffers {
            render(&svg[..], buffer, 1).unwrap();
        }
        assert!(all_equal(buffers.collect()));
    }

    fn all_equal(items: Vec<impl Eq>) -> bool {
        items.windows(2).all(|slice| slice[0] == slice[1])
    }
}
