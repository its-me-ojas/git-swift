use git2::Repository;

pub fn get_diff(repo_path: &str) -> Result<String, git2::Error> {
    let repo = Repository::open(repo_path)?;
    let head = match repo.head() {
        Ok(head) => head,
        Err(e) => {
            if e.code() == git2::ErrorCode::UnbornBranch {
                return Ok(String::from("No commits in the repository"));
            } else {
                return Err(e);
            }
        }
    };
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
