use crate::args::{
    check_args, get_args_cmd, get_cli_args, Args, ExecMode,
};
use crate::compile::compile_and_assemble;
use crate::error::io_error;
use crate::util::read_file;
use std::path::Path;

mod args;
mod ast;
mod compile;
mod context;
mod error;
mod log;
mod oxy_std;
mod parse;
mod position;
mod post_process;
mod symbols;
mod types;
mod util;

fn check_std(args: &Args) -> bool {
    if args.exec_mode != ExecMode::Lib
        && !Path::new(&args.std_path).exists()
    {
        io_error(format!(
            "STD file '{}' does not exist or is not accessible\n",
            args.std_path
        ))
            .print_stderr();
        false
    } else {
        true
    }
}

fn cli_exec(args: &Args) {
    let args_ = args.clone();
    let res = compile_and_assemble(
        args.eval.clone(),
        "CLI".to_owned(),
        &args_,
    );
    if res.is_err() {
        res.err().unwrap().pretty_print_stderr(
            args.eval.clone(),
            "CLI".to_string(),
        )
    }
}

fn import_exec(args: &Args) {
    if !Path::new(args.input.as_str()).exists() {
        io_error(format!(
            "Path '{}' doesn't exist\n",
            args.input
        ))
        .print_stderr();
        return;
    }

    let read_result = read_file(args.input.as_str());
    if read_result.is_err() {
        read_result.err().unwrap().print_stderr();
        return;
    }

    let source_code = read_result.unwrap();
    let res = compile_and_assemble(
        source_code.clone(),
        args.input.clone(),
        &args,
    );
    if res.is_err() {
        res.err().unwrap().pretty_print_stderr(
            source_code,
            args.input.clone(),
        )
    }
}

fn print_usage() {
    println!("{}", get_args_cmd().render_usage());
}

fn main() {
    let args = get_cli_args();

    if let Err(e) = check_args(&args) {
        e.print_stderr();
    }

    if !check_std(&args) {
        return;
    }

    if !args.eval.is_empty() {
        cli_exec(&args);
        return;
    }

    if !args.input.is_empty() {
        import_exec(&args);
        return;
    }

    print_usage()
}
