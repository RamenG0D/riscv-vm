use anyhow::Result;

fn main() -> Result<()> {
    // let output = Command::new("git")
    // 	.arg("rev-parse")
    // 	.arg("HEAD")
    // 	.output()
    // 	.expect("failed to execute git");
    // let commit_hash = String::from_utf8(output.stdout).unwrap();

    let repo = git2::Repository::open(env!("CARGO_MANIFEST_DIR"))?;
    // get the current commit of the repository
    // should be the same as the output of
    // `git log --format="%H" -n 1`
    let commit = repo.head()?.peel_to_commit()?.id().to_string();
    // get the current branch of the repository
    let binding = repo.head()?;
    let branch = binding.shorthand().unwrap_or("unknown");
    // get the current tag of the repository
    let binding = repo.tag_names(None)?;
    let tag = binding
        .iter()
        .flatten()
        .find(|tag| tag == &branch)
        .unwrap_or("unknown");

    println!("cargo:rustc-env=GIT_COMMIT={}", commit);
    println!("cargo:rustc-env=GIT_BRANCH={}", branch);
    println!("cargo:rustc-env=GIT_TAG={}", tag);

    let version = rustc_version::version().unwrap();
    println!("cargo:rustc-env=RUST_VERSION={}", version);

    Ok(())
}
