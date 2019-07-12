pub mod config_file;
pub mod file_creation;
pub mod read_file;
pub mod sections;

extern crate subprocess;
use crate::config_file::*;
use crate::file_creation::*;
use clap::{App, SubCommand};

/// Returns the name of the last MD and HTML files
fn get_files_names(index: usize, root_path: String) -> (String, String) {
    let date = time::now();
    (
        format!(
            "{}/{:#03}_{:#02}{:#02}{:#02}.md",
            root_path,
            index,
            date.tm_mday,
            date.tm_mon + 1,
            date.tm_year + 1900
        ),
        format!(
            "/tmp/{:#03}_{:#02}{:#02}{:#02}.html",
            index,
            date.tm_mday,
            date.tm_mon + 1,
            date.tm_year + 1900
        ),
    )
}

fn main() -> std::io::Result<()> {
    let json_file = "config.json";
    let config = ConfigFile::from_json(json_file);

    // let filename = config.get_last_file();
    // println!("LAST FILE: {}", filename);

    let index = config.get_last_index();
    let title = config.title();

    let (last_md, last_html) = get_files_names(index, config.root_path());
    let (future_md, _future_html) = get_files_names(index + 1, config.root_path());

    let matches = App::new("Popo")
        .version("0.1")
        .author("Guilloteau Q. <Quentin.Guilloteau@grenoble-inp.org>")
        .about("TODO manager in the command line")
        .subcommand(
            SubCommand::with_name("see")
                .about("Opens the last html file")
                .version("0.1")
                .author("Guilloteau Q. <Quentin.Guilloteau@grenoble-inp.org>"),
        )
        .subcommand(
            SubCommand::with_name("new")
                .about("Creates a new md file")
                .version("0.1")
                .author("Guilloteau Q. <Quentin.Guilloteau@grenoble-inp.org>"),
        )
        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("see") {
        config.generate_html_file(&last_md, &last_html);
        subprocess::Exec::cmd("firefox")
            .arg(last_html)
            .join()
            .unwrap();
    } else if let Some(_matches) = matches.subcommand_matches("new") {
        MarkdownFile::new(&future_md)
            .add_tasks_from_file(last_md)
            .with_title(title)
            .with_index(index + 1)
            .generate_file()?;

        config.open_file_in_editor(&future_md);
    } else {
        config.open_file_in_editor(&last_md);
    }

    Ok(())
}
