use core::panic;
use std::path::{Path, PathBuf};



pub fn find_svgs(root: PathBuf) -> Vec<PathBuf> {
    if root.exists() {
        if is_svg(&root) {
            vec![root]
        } else if root.is_dir() {
            root.read_dir()
                .expect("couln't read dir")
                .map(|result| result.unwrap().path())
                .filter(|path| is_svg(path.as_path()))
                .collect::<Vec<PathBuf>>()
        } else {
            vec![]
        }
    } else {
        panic!("Path doesn't exist: {}", root.to_str().unwrap());
    }
}

pub fn is_svg(path: &Path) -> bool {
    path.is_file()
        && path
            .extension()
            .map_or_else(|| false, |e| e.to_str() == Some("svg"))
}

pub fn rename_svg_to_png(svg: PathBuf) -> PathBuf {
    svg.with_extension("png")
}

#[cfg(test)]
mod test {

    use crate::test::test_asset;

    use super::*;

    #[test]
    fn test_svg_to_png() {
        assert_eq!(
            rename_svg_to_png(PathBuf::from("example.svg")),
            PathBuf::from("example.png")
        );
        assert_eq!(
            rename_svg_to_png(PathBuf::from("noExtension")),
            PathBuf::from("noExtension.png")
        );
        assert_eq!(
            rename_svg_to_png(PathBuf::from("deeply/nested/image.svg")),
            PathBuf::from("deeply/nested/image.png")
        );
    }

    #[test]
    fn test_is_svg() {
        assert!(is_svg(&test_asset("example.svg")));
        assert!(!is_svg(&test_asset("another_file.doc")));
        assert!(!is_svg(&test_asset("doesnt_exist")));
        assert!(!is_svg(&test_asset("no_extension")));
        assert!(!is_svg(&test_asset("")));
    }

    #[test]
    #[should_panic]
    fn test_find_svgs_panic() {
        find_svgs(test_asset("/doesnt_exist"));
    }

    #[test]
    fn test_find_svgs() {
        assert_eq!(find_svgs(test_asset("/")), vec![test_asset("example.svg")]);
        assert_eq!(
            find_svgs(test_asset("example.svg")),
            vec![test_asset("example.svg")]
        );
        assert_eq!(find_svgs(test_asset("another_file.doc")).len(), 0);
        assert_eq!(find_svgs(test_asset("no_extension")).len(), 0);
        assert_eq!(
            find_svgs(test_asset("multiple_svgs/")),
            vec![
                test_asset("multiple_svgs/example2.svg"),
                test_asset("multiple_svgs/example.svg")
            ]
        );
    }

}
