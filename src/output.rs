use super::file_tree::FileType;
use super::formatting::FormattedEntry;
use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

fn color_print<T>(text: T, color: Color)
where
    T: Display,
{
    let stdout = BufferWriter::stdout(ColorChoice::Auto);
    let mut buffer = stdout.buffer();
    let mut spec = ColorSpec::new();
    spec.set_fg(Some(color));
    buffer.set_color(&spec).unwrap();
    write!(&mut buffer, "{}", text).expect("");
    buffer.reset().unwrap();
    stdout.print(&buffer).expect("stdout print failure");
}

fn print_entries(entries: Vec<FormattedEntry>) {
    for (index, entry) in entries.iter().enumerate() {
        print!("{}[", entry.prefix);
        color_print(index, Color::Red);
        print!("] ");
        match &entry.file_type {
            FileType::Directory => {
                color_print(&entry.name, Color::Blue);
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

fn create_edit_aliases(editor: String, entries: Vec<FormattedEntry>) {
    let user = env::var("USER").unwrap_or(String::from(""));
    let mut alias_file = env::temp_dir();
    alias_file.push(&format!("tag_aliases_{}", user));
    let file = File::create(&alias_file);
    if !file.is_ok() {
        eprintln!("[tre] failed to open {}", alias_file.to_str().unwrap());
    }
    let mut alias_file = file.unwrap();

    for entry in entries {
        writeln!(
            &mut alias_file,
            "alias e4=\"eval '{} \\\"{}\\\"'\"",
            editor, entry.path
        )
        .expect("[tre] failed to write to alias file.");
    }
}
