mod git_lib;

use std::io::{self, prelude::*, BufReader};
use std::io::Error;
use crate::git_lib::git_lib::GitPostReceiveInput;

fn main() -> Result<(), Error> {
    println!("Kirby Hook start");

    let input_vec: Vec<GitPostReceiveInput> = parse_input_from_stdin()?;

    for git_input in &input_vec {
        println!("{}", git_input);
    }

    println!("Kirby Hook stop");

    Ok(())
}

fn parse_input_from_stdin() -> Result<Vec<GitPostReceiveInput>, Error> {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin);

    let mut git_commit_ids: Vec<String> = Vec::new();
    // Read commit refs and branch from standard input
    for line in reader.lines() {
        git_commit_ids.push(line?);
    }

    let mut input_vec: Vec<GitPostReceiveInput> = Vec::new();
    println!("Number of commits: {}", git_commit_ids.len());
    for commit_id in git_commit_ids {

        let commit: GitPostReceiveInput =
            GitPostReceiveInput::from_command_line_string(commit_id);

        input_vec.push(commit);
    }

    return Ok(input_vec);
}
