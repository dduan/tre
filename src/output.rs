use super::file_tree::FileType;
use super::formatting::FormattedEntry;
use atty;
use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
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

#[cfg(target_os = "windows")]
fn open_alias_file_with_suffix(suffix: &str) -> io::Result<File> {
    let file_name = format!("tre_aliases_{}.{}", env::var("USERNAME").unwrap_or("".to_string()), suffix);
    let home = env::var("HOME").unwrap_or(r".".to_string());
    let tmp = env::var("TEMP").unwrap_or(home);
    let path: PathBuf = [tmp, file_name].iter().collect();
    let file = File::create(&path);
    if !file.is_ok() {
        eprintln!("[tre] failed to open {:?}", path);
    }

    file
}

#[cfg(target_os = "windows")]
pub fn create_edit_aliases(editor: &str, entries: &Vec<FormattedEntry>) {
    let powershell_alias = open_alias_file_with_suffix("ps1");
    if let Some(mut alias_file) = powershell_alias.ok() {
        for (index, entry) in entries.iter().enumerate() {
            let editor = format!("{}", if editor.is_empty() { "Start-Process" } else { editor });
            let result = writeln!(
                &mut alias_file,
                "doskey /exename=pwsh.exe e{}={} {}\ndoskey /exename=powershell.exe e{}={} {}",
                index, editor, entry.path,
                index, editor, entry.path,
            );

            if !result.is_ok() {
                eprintln!("[tre] failed to write to alias file.");
            }
        }
    }

    let cmd_alias = open_alias_file_with_suffix("bat");
    if let Some(mut alias_file) = cmd_alias.ok() {
        for (index, entry) in entries.iter().enumerate() {
            let editor = format!("{}", if editor.is_empty() { "START" } else { editor });
            let result = writeln!(
                &mut alias_file,
                "doskey /exename=cmd.exe e{}={} {}",
                index, editor, entry.path,
            );

            if !result.is_ok() {
                eprintln!("[tre] failed to write to alias file.");
            }
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn open_alias_file() -> io::Result<File> {
    let user = env::var("USER").unwrap_or("".to_string());
    let alias_file = format!("/tmp/tre_aliases_{}", &user);
    let path: PathBuf = [alias_file].iter().collect();
    let file = File::create(&path);
    if !file.is_ok() {
        eprintln!("[tre] failed to open {:?}", path);
    }

    file
}

#[cfg(not(target_os = "windows"))]
pub fn create_edit_aliases(editor: &str, entries: &Vec<FormattedEntry>) {
    let alias = open_alias_file();
    if let Some(mut alias_file) = alias.ok() {
        for (index, entry) in entries.iter().enumerate() {
            let result = writeln!(
                &mut alias_file,
                "alias e{}=\"eval '{} \\\"{}\\\"'\"",
                index, editor, entry.path.replace("'", "\\'")
            );

            if !result.is_ok() {
                eprintln!("[tre] failed to write to alias file.");
            }
        }
    }
}
