use std::env;

mod cli;
mod file_tree;
mod diagram_formatting;
mod json_formatting;
mod output;
mod path_finders;
mod tre;

fn main() {
    let args: Vec<String> = env::args().collect();
    let option = cli::get_run_option(&args);
    tre::run(option)
}
