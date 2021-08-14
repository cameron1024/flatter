
#[derive(Debug, serde::Deserialize, PartialEq, Eq)]
pub struct RenderYaml {
    #[serde(default = "default_scales")]
    pub scales: Vec<u32>,

    pub threads: Option<usize>,
}

impl Default for RenderYaml {
    fn default() -> Self {
        Self {
            scales: default_scales(),
            threads: None
        }
    }
}

fn default_scales() -> Vec<u32> {
    vec![1]
}


#[cfg(test)]
mod test {
    use super::*;

    fn parse(s: &str) -> RenderYaml {
        serde_yaml::from_str(s).unwrap()
    }

    #[test]
    fn should_be_able_to_parse_yaml() {
        let yaml = parse("scales: [1, 2, 3]\nthreads: 2");
        assert_eq!(yaml, RenderYaml {
            scales: vec![1, 2, 3],
            threads: Some(2),
        });
    }

    #[test]
    fn should_handle_missing_threads() {
        let yaml = parse("scales: [1, 2, 3]");
        assert_eq!(yaml, RenderYaml {
            scales: vec![1, 2, 3],
            threads: None,
        });
    }

    #[test]
    fn should_handle_missing_scales() {
        let yaml = parse("threads: 1");
        assert_eq!(yaml, RenderYaml {
            scales: vec![1],
            threads: Some(1),
        });
    }
}
