//! File for the representation of the configuration file
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;

/// Representation of the configuration file
#[derive(Debug, Clone, Deserialize)]
pub struct ConfigFile {
    /// The path to the root of the project
    /// where is stored the Markdown files
    root_path: String,
    /// Path to the CSS file
    /// Optional
    css_path: Option<String>,
    /// Path to the location where the user wants to
    /// store the html files
    /// Optional
    /// If empty, we could either put them in /tmp or with the sources
    /// TODO: Decide
    doc_path: Option<String>,
    /// Favorite text editor
    text_editor: String,
    /// Title of the file
    title: String,
}

impl ConfigFile {
    /// Take the content of a JSON file and returns the structure associated
    pub fn from_json<T: AsRef<str>>(json_file: T) -> Self {
        let mut file = File::open(json_file.as_ref()).expect("Could not open the config file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Could not read the config file");
        serde_json::from_str(contents.as_str()).expect("Could not extract the config file properly")
    }

    /// Returns the root path of the structure
    pub fn root_path(&self) -> String {
        self.root_path.clone()
    }

    /// Returns the css path of the structure
    pub fn css_path(&self) -> Option<String> {
        self.css_path.clone()
    }

    /// Returns the title of the files
    pub fn title(&self) -> String {
        self.title.clone()
    }

    /// Returns the doc path of the structure
    /// If the doc path is empty, then use the root path
    /// as the doc path
    pub fn doc_path(&self) -> String {
        self.doc_path
            .clone()
            .unwrap_or_else(|| self.root_path.clone())
    }

    /// Open the given file in the favorite text editor
    pub fn open_file_in_editor<T: AsRef<str>>(&self, filename: T) {
        subprocess::Exec::cmd(self.text_editor.as_str())
            .arg(filename.as_ref())
            .join()
            .unwrap();
    }

    /// Generate the html file
    pub fn generate_html_file<T: AsRef<str>, S: AsRef<str>>(
        &self,
        input_filename: T,
        output_filename: S,
    ) {
        if self.css_path.is_none() {
            subprocess::Exec::cmd("pandoc")
                .args(&[
                    input_filename.as_ref(),
                    "--from",
                    "markdown_github",
                    "--to",
                    "html5",
                    "-o",
                    output_filename.as_ref(),
                ])
                .join()
                .unwrap();
        } else {
            subprocess::Exec::cmd("pandoc")
                .args(&[
                    input_filename.as_ref(),
                    "--from",
                    "markdown_github",
                    "--to",
                    "html5",
                    "-c",
                    self.css_path().unwrap().as_str(),
                    "-o",
                    output_filename.as_ref(),
                ])
                .join()
                .unwrap();
        }
        subprocess::Exec::cmd("sed")
            .args(&[
                "--in-place",
                r"s/\[ \]/<input type='checkbox' disabled=''\/>/g",
                output_filename.as_ref(),
            ])
            .join()
            .unwrap();
        subprocess::Exec::cmd("sed")
            .args(&[
                "--in-place",
                r"s/\[x\]/<input type='checkbox' checked=''\/>/g",
                output_filename.as_ref(),
            ])
            .join()
            .unwrap();
    }

    /// Returns the last index of the files
    pub fn get_last_index(&self) -> usize {
        // Getting the files
        let command1 = subprocess::Exec::cmd("ls").arg(self.root_path()).arg("-1q");
        // Counting the files
        let command2 = subprocess::Exec::cmd("wc").arg("-l");
        // Piping the commands
        let r = subprocess::Pipeline::new(command1, command2)
            .capture()
            .expect("Could not capture")
            .stdout_str();
        // Parsing the result into a usize
        r.trim()
            .parse::<usize>()
            .expect("Could not parse the result")
    }

    /// Returns the last file of the files
    pub fn get_last_file(&self) -> String {
        // Getting the files
        let command1 = subprocess::Exec::cmd("ls").arg(self.root_path()).arg("-1q");
        // Taking the last one
        let command2 = subprocess::Exec::cmd("tail").arg("-n").arg("1");

        // Piping the commands
        let r = subprocess::Pipeline::new(command1, command2)
            .capture()
            .expect("Could not capture")
            .stdout_str();

        format!("{}/{}", self.root_path(), r.trim())
    }
}
