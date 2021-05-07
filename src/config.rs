pub struct Config {
    pub(crate) input_files: Vec<String>,
    pub(crate) pretty: bool,
    pub(crate) pretty_title: bool,
    pub(crate) pretty_versions: bool,
    pub(crate) pretty_revisions: bool,
    pub(crate) pretty_biblio: bool,
    pub(crate) pretty_content: bool,
}

impl Config {
    pub(crate) fn new(_args: &[String]) -> Config {
        let input_files = Vec::new();

        Config { input_files, pretty: false, pretty_title: false, pretty_versions: false, pretty_revisions: false, pretty_biblio: false, pretty_content: false }
    }
}