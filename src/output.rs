use super::file_tree::FileType;
use super::formatting::FormattedEntry;
use atty;
use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

fn color_print<T>(text: T, color: Color) -> bool where T: Display {
    if atty::is(atty::Stream::Stdout) {
        let stdout = BufferWriter::stdout(ColorChoice::Auto);
        let mut buffer = stdout.buffer();
        let mut spec = ColorSpec::new();
        spec.set_fg(Some(color));
        buffer
            .set_color(&spec)
            .and_then(|_| write!(&mut buffer, "{}", text))
            .and_then(|_| buffer.reset())
            .and_then(|_| stdout.print(&buffer))
            .is_ok()
    } else {
        print!("{}", text);
        true
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

pub fn create_edit_aliases(editor: &str, entries: &Vec<FormattedEntry>) {
    let user = env::var("USER").unwrap_or("".to_string());
    let alias_file = format!("/tmp/tre_aliases_{}", &user);
    let alias_file_path = Path::new(&alias_file);
    let file = File::create(&alias_file_path);
    if !file.is_ok() {
        eprintln!("[tre] failed to open {}", alias_file);
        return
    }

    if let Some(mut alias_file) = file.ok() {
        for (index, entry) in entries.iter().enumerate() {
            let result = writeln!(
                &mut alias_file,
                "alias e{}=\"eval '{} \\\"{}\\\"'\"",
                index, editor, entry.path
            );

            if !result.is_ok() {
                eprintln!("[tre] failed to write to alias file.");
            }
        }
    } else {
        eprintln!("[tre] failed to open {}", alias_file);
    }

}
