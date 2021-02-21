use super::diagram_formatting::FormattedEntry;
use lscolors::{self, LsColors, Style};
use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

fn color_print<T>(text: T, color: &ColorSpec) -> bool
where
    T: Display,
{
    if atty::is(atty::Stream::Stdout) {
        let stdout = BufferWriter::stdout(ColorChoice::Auto);
        let mut buffer = stdout.buffer();
        buffer
            .set_color(&color)
            .and_then(|_| write!(&mut buffer, "{}", text))
            .and_then(|_| buffer.reset())
            .and_then(|_| stdout.print(&buffer))
            .is_ok()
    } else {
        print!("{}", text);
        true
    }
}

pub fn print_entries(entries: &[FormattedEntry], create_alias: bool, lscolors: &LsColors) {
    for (index, entry) in entries.iter().enumerate() {
        if create_alias {
            print!("{}[", entry.prefix);

            color_print(index, ColorSpec::new().set_fg(Some(Color::Red)));
            print!("] ");
        } else {
            print!("{}", entry.prefix);
        }

        let spec = lscolors
            .style_for_path(&entry.path)
            .map(convert_to_color_spec)
            .unwrap_or_default();
        color_print(&entry.name, &spec);
        println!()
    }
}

fn convert_color(color: &lscolors::Color) -> Color {
    match color {
        lscolors::Color::RGB(r, g, b) => Color::Rgb(*r, *g, *b),
        lscolors::Color::Fixed(n) => Color::Ansi256(*n),
        lscolors::Color::Black => Color::Black,
        lscolors::Color::Red => Color::Red,
        lscolors::Color::Green => Color::Green,
        lscolors::Color::Yellow => Color::Yellow,
        lscolors::Color::Blue => Color::Blue,
        lscolors::Color::Magenta => Color::Magenta,
        lscolors::Color::Cyan => Color::Cyan,
        lscolors::Color::White => Color::White,
    }
}

fn convert_to_color_spec(style: &Style) -> ColorSpec {
    let mut spec = ColorSpec::new();

    if let Some(color) = &style.foreground {
        spec.set_fg(Some(convert_color(&color)));
    }

    if let Some(color) = &style.background {
        spec.set_bg(Some(convert_color(&color)));
    }

    spec.set_bold(style.font_style.bold);
    spec.set_italic(style.font_style.italic);
    spec.set_underline(style.font_style.underline);

    spec
}

#[cfg(target_os = "windows")]
fn open_alias_file_with_suffix(suffix: &str) -> io::Result<File> {
    let file_name = format!(
        "tre_aliases_{}.{}",
        env::var("USERNAME").unwrap_or("".to_string()),
        suffix
    );
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
            let editor = format!(
                "{}",
                if editor.is_empty() {
                    "Start-Process"
                } else {
                    editor
                }
            );
            let result = writeln!(
                &mut alias_file,
                "doskey /exename=pwsh.exe e{}={} {}\ndoskey /exename=powershell.exe e{}={} {}",
                index, editor, entry.path, index, editor, entry.path,
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
    let user = env::var("USER").unwrap_or_else(|_| "".to_string());
    let alias_file = format!("/tmp/tre_aliases_{}", &user);
    let path: PathBuf = [alias_file].iter().collect();
    let file = File::create(&path);
    if file.is_err() {
        eprintln!("[tre] failed to open {:?}", path);
    }

    file
}

#[cfg(not(target_os = "windows"))]
pub fn create_edit_aliases(editor: &str, entries: &[FormattedEntry]) {
    let alias = open_alias_file();
    if let Ok(mut alias_file) = alias {
        for (index, entry) in entries.iter().enumerate() {
            let result = writeln!(
                &mut alias_file,
                "alias e{}=\"eval '{} \\\"{}\\\"'\"",
                index,
                editor,
                entry.path.replace("'", "\\'")
            );

            if result.is_err() {
                eprintln!("[tre] failed to write to alias file.");
            }
        }
    }
}
