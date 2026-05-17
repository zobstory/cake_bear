use std::env;
use std::process::ExitCode;

const USAGE: &str = "\
cakec — cakebear compiler (Phase 1 bootstrap)

USAGE:
    cakec build <file.ts> [-o <out>]
    cakec --help

Phase 1 supports primitive types, typed functions, and `console.log` only.
";

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("--help") | Some("-h") | None => {
            print!("{USAGE}");
            ExitCode::SUCCESS
        }
        Some("build") => {
            eprintln!("cakec build: not yet implemented (Phase 1 pipeline lands in follow-up commits)");
            ExitCode::from(2)
        }
        Some(other) => {
            eprintln!("cakec: unknown subcommand `{other}`\n\n{USAGE}");
            ExitCode::from(2)
        }
    }
}
