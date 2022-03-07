mod vars;
mod types;
mod conditional;
mod loops;
mod functions;
mod pointers;
mod structs;
mod enums;
mod cli;

fn main() {
    vars::run();
    types::run();
    conditional::run();
    loops::run();
    functions::run();
    pointers::run();
    structs::run();
    enums::run();
    cli::run();
}
