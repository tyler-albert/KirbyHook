mod git_lib;

use std::io::{self, prelude::*, BufReader};
use std::io::Error;

fn main() -> Result<(), Error> {
    println!("Kirby Hook start");

    let stdin = io::stdin();
    let reader = BufReader::new(stdin);

    let mut git_commit_ids: Vec<String> = Vec::new();
    for line in reader.lines() {
        git_commit_ids.push(line?);
    }

    println!("Number of commits: {}", git_commit_ids.len());
    for commit_id in git_commit_ids {
        let commit_array = commit_id.split(" ");
        println!("Commit1");
        for commit_string in commit_array {
            println!("\t{}", commit_string);
        }
    }

    println!("Kirby Hook stop");

    Ok(())
}
