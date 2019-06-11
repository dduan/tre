use super::file_tree::FileType;
use super::formatting::FormattedEntry;
use atty;
use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

fn color_print<T>(text: T, color: Color)
where
    T: Display,
{
    if atty::is(atty::Stream::Stdout) {
        let stdout = BufferWriter::stdout(ColorChoice::Auto);
        let mut buffer = stdout.buffer();
        let mut spec = ColorSpec::new();
        spec.set_fg(Some(color));
        buffer.set_color(&spec).unwrap();
        write!(&mut buffer, "{}", text).expect("");
        buffer.reset().unwrap();
        stdout.print(&buffer).expect("stdout print failure");
    } else {
        print!("{}", text);
    }
}

pub fn print_entries(entries: &Vec<FormattedEntry>, create_alias: bool) {
    for (index, entry) in entries.iter().enumerate() {
        if create_alias {
            print!("{}[", entry.prefix);
            color_print(index, Color::Red);
            print!("] ");
        } else {
            print!("{}", entry.prefix);
        }
        match &entry.file_type {
            FileType::Directory => {
                if cfg!(windows) {
                    color_print(&entry.name, Color::Green);
                } else {
                    color_print(&entry.name, Color::Blue);
                }
            }
            FileType::File => {
                print!("{}", entry.name);
            }
            FileType::Link => {
                color_print(&entry.name, Color::Magenta);
                let link = &entry.link;
                print!(" -> {}", link.clone().unwrap_or("".to_string()));
            }
        }
        print!("\n")
    }
}

pub fn create_edit_aliases(editor: &String, entries: &Vec<FormattedEntry>) {
    let user = env::var("USER").unwrap_or("".to_string());
    let alias_file = format!("/tmp/tre_aliases_{}", &user);
    let alias_file = Path::new(&alias_file);
    let file = File::create(&alias_file);
    if !file.is_ok() {
        eprintln!("[tre] failed to open {}", alias_file.to_str().unwrap());
    }
    let mut alias_file = file.unwrap();

    for (index, entry) in entries.iter().enumerate() {
        writeln!(
            &mut alias_file,
            "alias e{}=\"eval '{} \\\"{}\\\"'\"",
            index, editor, entry.path
        )
        .expect("[tre] failed to write to alias file.");
    }
}
