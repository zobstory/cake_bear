use std::env;
use std::fs;
use std::process::ExitCode;

const USAGE: &str = "\
cakec — cakebear compiler (Phase 1, in progress)

USAGE:
    cakec build <file.ts> [--dump-tokens]
    cakec --help

Phase 1 currently lexes the source. Parser / sema / codegen land in
follow-up commits.
";

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("--help") | Some("-h") | None => {
            print!("{USAGE}");
            ExitCode::SUCCESS
        }
        Some("build") => match run_build(args.collect()) {
            Ok(code) => code,
            Err(msg) => {
                eprintln!("cakec: {msg}");
                ExitCode::from(1)
            }
        },
        Some(other) => {
            eprintln!("cakec: unknown subcommand `{other}`\n\n{USAGE}");
            ExitCode::from(2)
        }
    }
}

fn run_build(args: Vec<String>) -> Result<ExitCode, String> {
    let mut path: Option<String> = None;
    let mut dump_tokens = false;
    for a in args {
        match a.as_str() {
            "--dump-tokens" => dump_tokens = true,
            flag if flag.starts_with("--") => return Err(format!("unknown flag `{flag}`")),
            _ => {
                if path.is_some() {
                    return Err("multiple input files passed; expected one".into());
                }
                path = Some(a);
            }
        }
    }
    let path = path.ok_or_else(|| "missing input file\n\nrun `cakec --help` for usage".to_string())?;

    let src = fs::read_to_string(&path).map_err(|e| format!("reading {path}: {e}"))?;
    let tokens = cakec_lexer::lex(&src).map_err(|e| format!("{path}: {e}"))?;

    if dump_tokens {
        for tok in &tokens {
            println!("{:>4}..{:<4} {:?}", tok.span.start, tok.span.end, tok.kind);
        }
    }

    eprintln!(
        "cakec: lexed {} tokens from {path}",
        tokens.len().saturating_sub(1) // don't count the EOF in the human-facing tally
    );
    eprintln!("cakec: parser not yet implemented — pipeline stops here for now");
    Ok(ExitCode::from(2))
}
