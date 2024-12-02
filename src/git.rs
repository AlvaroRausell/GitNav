use crate::macros::msg;
use colored::Colorize;
use git2::Repository;
pub fn get_current_branch(repo: &Repository) -> String {
    repo.head().unwrap().name().unwrap().to_string()
}

pub fn checkout_to(repo: &Repository, target: &str) {
    msg("Checking out to".yellow(), target);
    let (obj, reference) = repo.revparse_ext(&(target)).unwrap();
    repo.checkout_tree(&obj, None).unwrap();
    match reference {
        // gref is an actual reference like branches or tags
        Some(gref) => repo.set_head(gref.name().unwrap()),
        // this is a commit, not a reference
        None => repo.set_head_detached(obj.id()),
    }
    .expect("Failed to set HEAD");
}
