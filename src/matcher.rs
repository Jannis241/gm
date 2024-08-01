use env::consts::{ARCH, EXE_SUFFIX};

use crate::*;

pub async fn identify_pattern(tokens: Vec<Arguement>, input: Input) -> Result<(), Box<dyn std::error::Error>>{
    // Initialisiere Konfiguration
    let mut user_config = config_manager::manage_config();

    // Holen aller Repositories
    let mut repo_list = git_commands::get_all_repositories(&user_config);
    let mut repo_names_list: Vec<String> = repo_list.iter().map(|repo| repo.Name.clone()).collect();
    let mut repo_path_list: Vec<String> = repo_list.iter().map(|repo| repo.Path.clone()).collect();

    // Hilfe-Nachrichten
    if tokens.is_empty() {
        helper::print_general_help();
        return Ok(())
    } else if tokens.len() == 1 && tokens[0] == Arguement::HELP {
        helper::print_general_help();
        return Ok(())
    }
    
    // Command-Verarbeitung
    match &tokens[..] {
        // help 
        [Arguement::CONFIG, Arguement::HELP] => print_config_help(),
        [Arguement::LIST, Arguement::HELP] => {
            helper::print_list_help();
        }
        [Arguement::UPLOAD, Arguement::HELP] => {
            helper::print_upload_help();
        }
        [Arguement::DOWNLOAD, Arguement::ALL, Arguement::HELP] => {
            print_download_all_help();
        }
        [Arguement::DOWNLOAD, Arguement::HELP] => {
            print_download_help();
        }
        [Arguement::UPLOAD, Arguement::ALL, Arguement::HELP] => {
            helper::print_upload_all_help();
        }
        [Arguement::CLEAR, Arguement::HELP] =>{
            print_clear_help();
        }
        [Arguement::SET, Arguement::CONFIG, Arguement::HELP] => {
            print_set_config_help();
       } 
        [Arguement::SHOW, Arguement::CONFIG, Arguement::HELP] => {
            print_show_config_help();
       }


        // commands
        [Arguement::SET, Arguement::CONFIG,  Arguement::NAME(ref what_to_change), Arguement::NAME(ref change)]=>{
            match what_to_change.as_str() {
                "username" => {
                    user_config.username = change.into();
                    config_manager::write_to_json("./config.json", &user_config);
                }
                "key" => {
                    user_config.api_key = change.to_string();
                    config_manager::write_to_json("./config.json", &user_config);
                }
                "path" => {
                    user_config.project_path = change.to_string();
                    config_manager::write_to_json("./config.json", &user_config);
                }
                _ => {
                    throw_error("invalid arguement. See 'gm set config --help'")
                }
            }
        }
        [Arguement::SHOW, Arguement::CONFIG] => {
            println!("{}: {}", "Username".blue().underline(),&user_config.username);
            println!("{}: {}", "Api-Key".blue().underline(),&user_config.api_key);
            println!("{}: {}", "Project path".blue().underline(), &user_config.project_path);
        }


        [Arguement::CLEAR] => terminal::clear_terminal(),
        [Arguement::LIST, Arguement::NAME(ref name)] => {
            let paths = git_commands::get_all_repos_of_user(&name, None).await;
            match paths {
                Ok(paths) => {
                    let mut path_list = Vec::new();
                    for path in paths{
                        path_list.push(path.clone_url);
                    }
                    git_commands::print_repo_list(&path_list);
                }
                Err(RepoError::Unauthorized) => {
                    println!("failed: you are not authorized to get these repositories. Make sure you have configured your api-key correctly");
                    println!("See 'gm config --help' for more info");                }
                Err(RepoError::NetworkError(ref value)) => {
                    println!("failed: network error -> '{}'", value)
                }
                Err(RepoError::UserNotFound) => {
                    println!("failed: user not found")
                }
                Err(RepoError::RateLimitExceeded) => {
                    println!("failed: rate limit exceeded")
                }
                Err(RepoError::ServerError(ref e)) => {
                    println!("failed: server error -> '{}'", e)
                }
                Err(RepoError::ParseError(ref e)) => {
                    println!("failed: parse error -> '{}'", e)
                }
            }

        }
        [Arguement::LIST, Arguement::OWN] => {
            let paths = git_commands::get_all_repos_of_user(&user_config.username, Some(user_config.api_key)).await;

            match paths {
                Ok(paths) => {
                    let mut path_list = Vec::new();
                    for path in paths{
                        path_list.push(path.clone_url);
                    }
                    git_commands::print_repo_list(&path_list);
                }
                Err(RepoError::Unauthorized) => {
                    println!("failed: you are not authorized to get these repositories. Make sure you have configured your api-key correctly");
                    println!("See 'gm config --help' for more info");                }
                Err(RepoError::NetworkError(ref value)) => {
                    println!("failed: network error -> '{}'", value)
                }
                Err(RepoError::UserNotFound) => {
                    println!("failed: user not found")
                }
                Err(RepoError::RateLimitExceeded) => {
                    println!("failed: rate limit exceeded")
                }
                Err(RepoError::ServerError(ref e)) => {
                    println!("failed: server error -> '{}'", e)
                }
                Err(RepoError::ParseError(ref e)) => {
                    println!("failed: parse error -> '{}'", e)
                }
            }

        }
        [Arguement::LIST, Arguement::DOWNLOADED] => {
            // update repo list, names, and path in case a repo got added or deleted
            git_commands::update_repos(&mut repo_list, &mut repo_names_list, &mut repo_path_list, &user_config);
            if repo_path_list.len() == 0 {
                terminal::throw_error("No git projects found")
            }
            else {
                git_commands::print_repo_list(&repo_path_list);
            }
        }
        [Arguement::DOWNLOAD, Arguement::NAME(ref reponame), Arguement::FROM, Arguement::NAME(ref username)]=>{
            git_commands::download(&reponame, &username, &user_config.project_path);
        }
        [Arguement::DOWNLOAD, Arguement::ALL, Arguement::FROM, Arguement::NAME(ref name)] => {
            let mut key = None;
            if &&user_config.username == &name {
                println!("Info: to download your own repositories you need to specify your api key in the config");
                key = Some(user_config.api_key);
            }
            let repos = git_commands::get_all_repos_of_user(&name, key).await;
            match repos {
                Ok(repos) => {
                    clone_all_repos(&repos, user_config.project_path); 
                }
                
                Err(RepoError::Unauthorized) => {
                    println!("failed: you are not authorized to get these repositories. Make sure you have configured your api-key correctly");
                    println!("See 'gm config --help'");
                }
                Err(RepoError::NetworkError(ref value)) => {
                    println!("failed: network error -> '{}'", value)
                }
                Err(RepoError::UserNotFound) => {
                    println!("failed: user not found")
                }
                Err(RepoError::RateLimitExceeded) => {
                    println!("failed: rate limit exceeded")
                }
                Err(RepoError::ServerError(ref e)) => {
                    println!("failed: server error -> '{}'", e)
                }
                Err(RepoError::ParseError(ref e)) => {
                    println!("failed: parse error -> '{}'", e)
                }
            }
        }
        [Arguement::UPLOAD, Arguement::ALL, rest @ ..] => {
            handle_upload_all_command(rest, &repo_list, &repo_names_list, &repo_path_list);
        }
        [Arguement::UPLOAD, rest @ ..] => {
            handle_upload_command(rest, &repo_list, &repo_names_list, &repo_path_list);},
        _ => {
            throw_error(format!("{:?} is not a valid gm command. See 'gm --help'.", input.raw_input.join(" ")).as_str());
        }
    }
    Ok(())
}


fn handle_upload_all_command(rest: &[Arguement], repo_list: &[Repository], repo_names_list: &[String], repo_path_list: &[String]){
    let mut force = false;
    if !rest.is_empty(){
        match rest[..] { 
            [Arguement::FORCE] => force = true,
            _ => {
                throw_error("invalid arguement. See 'gm upload all --help'");
                exit(0)
            }
        }

    }

    for repo in repo_list {
        git_commands::upload(&repo.Path, &"committed by Git-Manager".to_string(), force, "main".to_string())
    }
}


fn handle_upload_command(rest: &[Arguement], repo_list: &[Repository], repo_names_list: &[String], repo_path_list: &[String]) {
    let current_dir = env::current_dir().expect("error while getting current directory in: (matcher, handle upload command)");
    let current_dir_str = current_dir.to_str().expect("error while converting to string").to_string();
    let is_current_dir_git_repository = repo_path_list.contains(&current_dir_str);

    let mut commit_message = "committed by Git-Manager".to_string();
    let mut force = false;
    let mut branch_name = "main".to_string();
    let mut repo_path = String::new();
    
    if rest.is_empty() {
        throw_error("missing arguements. See 'gm upload --help'");
        exit(0)
    }

    // <name> <msg> <branch> <force>

    // get path
    match rest[0] {
        Arguement::NAME(ref repo_name) => {
            let repo = get_repo_path_by_name(repo_name, repo_list);
            match repo {
                Some(path) => repo_path = path,
                None => {
                    throw_error("Repository not found. See 'gm list downloaded'");
                    println!("Note: make sure you have configured your project path correctly. See 'gm config --help'");
                    exit(0)
                }
            }

        }
        Arguement::PUNKT => {
            if repo_path_list.contains(&current_dir_str){
                repo_path = current_dir_str;
            }
            else {
                throw_error("You are currently not in a known repository. See 'gm list downloaded'");
                println!("Note: make sure you have configured your project path correctly. See 'gm config --help'");
                exit(0)
            }
        }
        _ => {
            throw_error("missing repository name. See 'gm upload --help'")
        }
    }
    for arg in &rest[1..]{
        match arg{
            Arguement::FORCE => force = true,
            Arguement::NAME(msg) => {
                if commit_message == "committed by Git-Manager".to_string() {
                    commit_message = msg.to_string();
                }
                else {
                    throw_error("cant assign commit message twice. See 'gm upload --help'");
                    exit(0);
                }
            }
            Arguement::BRANCH(name) => branch_name = name.to_string(),
            _ => {
                throw_error("Invalid arguement. See 'gm upload --help'");
                exit(0)
            }
        }
    }
    git_commands::upload(&repo_path, &commit_message, force, branch_name);

}

