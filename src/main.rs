use clap::Parser;

mod cli;
mod diagram_formatting;
mod file_tree;
mod json_formatting;
mod output;
mod path_finders;
mod tre;

fn main() {
    let inputs = cli::Interface::parse();
    let options = inputs.as_options();
    tre::run(options)
}
