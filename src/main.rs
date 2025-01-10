use git2::Repository;

fn get_diff(repo_path: &str) -> Result<String, git2::Error> {
    let repo = Repository::open(repo_path)?;
    let head = repo.head()?;
    let head_commit = head.peel_to_commit()?;
    let tree = head_commit.tree()?;

    let diff = repo.diff_tree_to_workdir_with_index(Some(&tree), None)?;

    let mut diff_str = String::new();

    diff.print(git2::DiffFormat::Patch, |_, _, line| {
        diff_str.push_str(std::str::from_utf8(line.content()).unwrap());
        true
    })?;

    Ok(diff_str)
}

fn main() {
    let repo_path = ".";
    match get_diff(repo_path) {
        Ok(diff) => println!("diff: {}", diff),
        Err(e) => println!("Error: {}", e),
    }
}
