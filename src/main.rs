mod git;
#[macro_use]
mod macros;
use crate::macros::err_out;

use clap::{Parser, ValueEnum};
use git::{checkout_to, get_current_branch};
use git2::Repository;
use semver::{Version, VersionReq};
use serde::Deserialize;
use serde_json;
use std::{
    fs::{self, File},
    io::BufReader,
    path::PathBuf,
};

#[derive(ValueEnum, Clone)]
enum SupportedFrameworks {
    /// For Node.js packages (default)
    Npm,
}

#[derive(Parser)]
#[command(name = "GitNav")]
#[command(version, about)]
struct Args {
    repository_path: PathBuf,
    #[clap(short = 'v', required = true, name = "target version", help="Semver-compatible version pattern to find")]
    find_version: String,
    #[clap(
        short = 'c',
        required = false,
        name = "create branch",
        default_value_t = false,
        help="Create and checkout to a branch containing the last valid version"
    )]
    create_branch: bool,
    #[clap(short='t', required=false, name="package framework", help="Package framework. Will affect which files to look at for versions." , default_value_t=SupportedFrameworks::Npm)]
    #[arg(value_enum)]
    framework_type: SupportedFrameworks,
}

#[derive(Debug, Deserialize)]
struct PackageJson {
    version: String,
}
fn read_package_json(repo_directory: &PathBuf) -> PackageJson {
    println!("Repo location: {:?}", repo_directory);
    let package_json_path = repo_directory.join("package.json");
    if !package_json_path.exists() {
        err!("Package.json not found");
    }
    let package_json = File::open(package_json_path).unwrap();
    let json: PackageJson = serde_json::from_reader(BufReader::new(package_json)).unwrap();
    json
}
fn main() {
    let cli_args = Args::parse();
    let version_parsed = VersionReq::parse(&cli_args.find_version).unwrap();
    let repo_path_raw = cli_args.repository_path;
    let repo_directory = fs::canonicalize(&repo_path_raw).unwrap();
    let repo = Repository::open(&repo_directory).unwrap();
    // TODO: Make default branch a CLI argument
    if !get_current_branch(&repo).ends_with("refs/heads/main") {
        checkout_to(&repo, "refs/heads/main");
    }
    let mut walker = repo.revwalk().unwrap();
    walker.push_head().unwrap();
    let found: Option<(String, String)> = walker
        .map(|commit| {
            let commit_oid = commit.as_ref().unwrap();
            checkout_to(&repo, commit_oid.to_string().as_str());
            let json = read_package_json(&repo_directory);
            println!(
                "Version {} found at commit {}",
                json.version,
                commit_oid.to_string()
            );
            if version_parsed.matches(&Version::parse(&json.version).unwrap()) {
                return Some((commit_oid.to_string(), json.version));
            }
            return None;
        })
        .filter(|commit| commit.is_some())
        .nth(0)
        .unwrap();
    match found {
        Some((commit, version)) => {
            println!(
                "Last commit with version {} ({}) found in commit: {}",
                cli_args.find_version, version, commit
            );
            if cli_args.create_branch {
                let branch_name = format!("service-branch-{}", version);
                repo.branch(
                    &branch_name,
                    &repo.head().unwrap().peel_to_commit().unwrap(),
                    true,
                )
                .unwrap();
                checkout_to(&repo, &branch_name);
            }
        }
        None => {
            println!(
                "No commit found satisfying version pattern: {}",
                cli_args.find_version
            )
        }
    }
}
