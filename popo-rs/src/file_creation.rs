//! File responsible for the creation of the Markdown file
use crate::read_file::extract_tasks_from_file;
use crate::sections::Section;

use std::fs::File;
use std::io::Write;

#[derive(Debug, Clone)]
pub struct Task {
    content: String,
}

impl Task {
    fn content(&self) -> String {
        self.content.clone()
    }
}

impl<T> From<T> for Task
where
    T: AsRef<str>,
{
    fn from(content: T) -> Self {
        Task {
            content: content.as_ref().to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MarkdownFile {
    /// The path of the File
    /// TODO: Relative to the root path ?
    filename: String,
    /// The tasks to be done
    undone_tasks: Vec<Task>,
    /// Order in which the sections should appear
    section_order: Vec<Section>,
    /// Title of the file
    title: String,
    /// Index of the file
    index: usize,
}

impl MarkdownFile {
    /// Creates a new MarkdownFile file structure
    /// with a filename
    pub fn new<T: AsRef<str>>(filename: T) -> Self {
        MarkdownFile {
            filename: filename.as_ref().to_string(),
            undone_tasks: Vec::new(),
            section_order: Section::default_order(),
            title: String::new(),
            index: 0,
        }
    }

    /// Add the tasks from a file
    pub fn add_tasks_from_file<T: AsRef<str>>(&self, filename: T) -> Self {
        MarkdownFile {
            filename: self.filename.clone(),
            undone_tasks: extract_tasks_from_file(filename)
                .expect("Unable to extract tasks from the file"),
            section_order: self.section_order.clone(),
            title: self.title.clone(),
            index: self.index,
        }
    }

    /// Add tasks to the file
    pub fn add_tasks(&self, tasks: Vec<Task>) -> Self {
        MarkdownFile {
            filename: self.filename.clone(),
            undone_tasks: self
                .undone_tasks
                .iter()
                .chain(tasks.iter())
                .map(|x| x.clone())
                .collect(),
            section_order: self.section_order.clone(),
            title: self.title.clone(),
            index: self.index,
        }
    }

    /// Add the section order
    pub fn with_section_order(&self, order: Vec<Section>) -> Self {
        MarkdownFile {
            filename: self.filename.clone(),
            undone_tasks: self.undone_tasks.clone(),
            section_order: order,
            title: self.title.clone(),
            index: self.index,
        }
    }

    /// Use the given title as the title of the file
    pub fn with_title<T: AsRef<str>>(&self, title: T) -> Self {
        MarkdownFile {
            filename: self.filename.clone(),
            undone_tasks: self.undone_tasks.clone(),
            section_order: self.section_order.clone(),
            title: title.as_ref().to_string(),
            index: self.index,
        }
    }

    /// Use the given index as the index of the file
    pub fn with_index(&self, index: usize) -> Self {
        MarkdownFile {
            filename: self.filename.clone(),
            undone_tasks: self.undone_tasks.clone(),
            section_order: self.section_order.clone(),
            title: self.title.clone(),
            index: index,
        }
    }

    pub fn generate_file(&self) -> std::io::Result<()> {
        let mut s = Vec::new();
        for sec in self.section_order.iter() {
            let sec_s = sec.to_string();
            match sec {
                Section::Goals => {
                    writeln!(&mut s, "{}", sec_s)?;
                    for task in self.undone_tasks.iter() {
                        writeln!(&mut s, "{}\n", task.content())?;
                    }
                }
                Section::Title => {
                    let date = time::now();
                    writeln!(
                        &mut s,
                        "{} {} #{}: {:#02}/{:#02}/{}\n",
                        sec_s,
                        self.title,
                        self.index,
                        date.tm_mday,
                        date.tm_mon + 1,
                        date.tm_year + 1900,
                    )?;
                }
                _ => {
                    writeln!(&mut s, "{}\n", sec_s)?;
                }
            }

            let mut file = File::create(&self.filename)?;
            file.write_all(&s)?;
        }
        Ok(())
    }
}
