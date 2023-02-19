use cmd_sys::EnumCommandLine;

mod cl;
mod commands;

fn main() {
    cl::CommandLine::command_line("Terminal Recipes> ")
}
