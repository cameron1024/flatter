use core::panic;
use std::{
    error::Error,
    fs::File,
    path::{Path, PathBuf},
};

use crate::yaml::RenderYaml;

const YAML_NAME: &str = "render.yaml";

pub fn find_yaml(root: PathBuf) -> Result<Option<RenderYaml>, Box<dyn Error>> {
    if root.is_file() {
        find_yaml_from_file(root)
    } else {
        let file = root.read_dir()?.find(|result| match result {
            Ok(entry) => entry.file_name().to_string_lossy() == YAML_NAME && entry.path().is_file(),
            Err(_) => false,
        });

        match file {
            Some(Ok(entry)) => find_yaml_from_file(entry.path()),
            Some(Err(e)) => Err(Box::new(e)),
            None => Ok(None),
        }
    }
}

// assumes pathbuf is a file
fn find_yaml_from_file(file: PathBuf) -> Result<Option<RenderYaml>, Box<dyn Error>> {
    if let Some(name) = file.file_name() {
        if name.to_string_lossy() == YAML_NAME {
            let yaml = serde_yaml::from_reader::<File, RenderYaml>(File::open(file)?)?;
            Ok(Some(yaml))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

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

    #[test]
    fn can_read_render_yaml_from_exact_path() {
        let path = test_asset("has_render_yaml/render.yaml");
        let yaml = find_yaml_from_file(path).unwrap().unwrap();
        assert_eq!(
            yaml,
            RenderYaml {
                scales: vec![1, 2, 3],
                threads: None,
            }
        );
    }

    #[test]
    fn ignores_non_yaml_files() {
        let path = test_asset("has_render_yaml/not_yaml_or_svg.doc");
        assert!(matches!(find_yaml_from_file(path), Ok(None)));

        let path = test_asset("has_render_yaml/doesnt_exist");
        assert!(matches!(find_yaml_from_file(path), Ok(None)));
    }

    #[test]
    fn finds_yamls_in_dir() {
        let path = test_asset("has_render_yaml");
        let yaml = find_yaml(path).unwrap().unwrap();
        assert_eq!(
            yaml,
            RenderYaml {
                scales: vec![1, 2, 3],
                threads: None,
            }
        );
    }

    #[test]
    fn ignores_dirs_with_no_yaml() {
        let path = test_asset("empty");
        assert!(matches!(find_yaml(path), Ok(None)));
    }

    #[test]
    fn handles_bad_yaml() {
        let path = test_asset("bad_render_yaml");
        assert!(matches!(find_yaml(path), Err(_)));
    }
}
