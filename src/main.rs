use clap::{Parser, Subcommand};
use octocrab::{models::Repository, Octocrab};
use serde_json::json;

#[derive(Subcommand)]
enum GhpmCommand {
    /// Turns all your repositories private, except starred ones
    #[clap(name = "thanos_snap")]
    ThanosSnap,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(name = "ghpm-rs")]
#[command(version = "v0.1.0")]
#[command(about = "Manages your github privacy", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: GhpmCommand,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let output = std::process::Command::new("gh")
    .arg("auth")
    .arg("token")
    .arg("--secure-storage")
    .arg("--hostname")
    .arg("github.com")
    .output()
    .expect("expected gh cli to be installed under the path 'gh'. Do you have the gh cli under another path or name ? if yes please complain to the developer");

    let token = std::str::from_utf8(&output.stdout)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Non-UTF8 output"))?
        .trim()
        .to_string();

    if token.is_empty() {
        panic!("can't work unless authenticated to github with 'gh login' ")
    }

    let octocrab = Octocrab::builder().personal_token(token).build()?;

    let user: octocrab::models::UserProfile = octocrab.get("/user", None::<&()>).await?;

    let username = &user.login;

    println!("running as {}", username);

    let readme_repository = format!("{}/{}", username, username);

    let payload = json!({
        "private": true,
    });

    let public_repositories_github_endpoint = format!(
        "https://api.github.com/users/{}/repos?visibility=public&per_page=100",
        username
    );

    match &cli.command {
        GhpmCommand::ThanosSnap => {
            loop {
                let public_repositories: Vec<Repository> = octocrab
                    .get(&public_repositories_github_endpoint, None::<&()>)
                    .await?;

                let public_repositories_names: Vec<&String> = public_repositories
                    .iter()
                    .filter_map(|repo| repo.full_name.as_ref().or(None))
                    .collect();

                println!(
                    "your public repositories : {:#?}",
                    public_repositories_names
                );

                for repo in public_repositories.clone() {
                    let Some(repository_name) = repo.full_name else {
                        println!("skipped a repository without a full_name");
                        continue;
                    };

                    if repo.stargazers_count != Some(0) {
                        println!(
                            "repository {} cannot be switched to private by ghpm-rs because it has stars.",
                            &repository_name,
                        );

                        continue;
                    }

                    if repo.stargazers_count.is_none() {
                        println!(
                            "skipped {}. ghpm-rs could not read its number of stars",
                            &repository_name,
                        );

                        continue;
                    }

                    if repository_name == readme_repository {
                        println!(
                            "dodging the README repository {} because it's a special repository",
                            readme_repository
                        );

                        continue;
                    }

                    let current_endpoint =
                        format!("https://api.github.com/repos/{}", &repository_name);

                    let result = octocrab
                        .patch::<Repository, _, _>(current_endpoint, Some(&payload))
                        .await;

                    match result {
                        Ok(_) => {
                            println!("{} was switched to private", &repository_name)
                        }
                        Err(_) => {
                            println!("failed to switch {} to private. Manually switch it to private (see README) and please complain to the developer", &repository_name)
                        }
                    }
                }

                if public_repositories.len() != 100 {
                    break;
                }
            }

            Ok(())
        }
    }
}
