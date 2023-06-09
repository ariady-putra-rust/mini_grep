use minigrep_lib::*;
use std::{env, io::Result, process::exit};

// CASE_SENSITIVE=1 cargo run -- [QUERY] [FILE_PATH]

type Query = String;
type Path = String;
type CaseSensitive = bool;

const CASE_SENSITIVE: &str = "CASE_SENSITIVE";

fn run_app(query: &Query, file_path: &Path, case_sensitive: CaseSensitive) -> Result<()> {
    Ok({
        let search_result = mini_grep(&query, &file_path, case_sensitive)?;
        for hit in search_result {
            println!("{hit}");
        }
    })
}

fn main() -> Result<()> {
    Ok({
        let args: Vec<String> = env::args().collect();
        if let Some((query, file_path)) = process_args(&args) {
            let case_sensitive = env_var_case_sensitive();
            run_app(&query, &file_path, case_sensitive)?;
        } else {
            show_usage();
        }
    })
}

fn env_var_case_sensitive() -> bool {
    match env::var(CASE_SENSITIVE) {
        Ok(case_sensitive) => case_sensitive == "1",
        Err(x) => {
            eprintln!(
                r#"{CASE_SENSITIVE} {x}, defaulting to CASE_INSENSITIVE
run with `{CASE_SENSITIVE}=1` to search with case sensitivity enabled"#
            ); // print to stderr
            return false;
        }
    }
}

fn process_args(args: &Vec<String>) -> Option<(Query, Path)> {
    let query = args.get(1)?;
    let file_path = args.get(2)?;

    return Some((query.to_string(), file_path.to_string()));
}

fn show_usage() {
    eprintln!(r#"Valid arguments are "Query" "File Path""#);
    exit(-1);
}
