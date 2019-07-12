//! File responsible to extract undone tasks from a file
use crate::file_creation::Task;

use std::fs::File;
use std::io::{BufRead, BufReader};

/// Take the name of a file and return a vector of tasks that are not completed
pub fn extract_tasks_from_file<T: AsRef<str>>(filename: T) -> std::io::Result<Vec<Task>> {
    let path = std::path::Path::new(filename.as_ref());
    let file = File::open(path);
    if file.is_err() {
        return Ok(Vec::new());
    }
    let file = file?;

    let mut tasks: Vec<Task> = Vec::new();

    for line in BufReader::new(file).lines() {
        let line_string = line?;
        if line_string.trim().starts_with("- [ ]") {
            tasks.push(Task::from(line_string));
        }
    }
    Ok(tasks)
}
