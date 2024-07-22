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
        [Arguement::LIST, Arguement::HELP] => {
            helper::print_list_help();
        }
        [Arguement::UPLOAD, Arguement::HELP] => {
            helper::print_upload_help();},
        [Arguement::DOWNLOAD, Arguement::ALL, Arguement::HELP] => {
            print_download_all_help();
        }
        [Arguement::DOWNLOAD, Arguement::HELP] => {
            print_download_help();
        }
        [Arguement::UPLOAD, Arguement::ALL, Arguement::HELP] => {
            helper::print_upload_all_help();
        }
        
        // commands
        [Arguement::CLEAR] => terminal::clear_terminal(),
        [Arguement::LIST] => {
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
            git_commands::clone_all_repos(&name, key, &user_config.project_path).await?;
        }
        [Arguement::UPLOAD, rest @ ..] => {
            handle_upload_command(rest, &repo_list, &repo_names_list, &repo_path_list);},
        _ => {
            throw_error(format!("{:?} is not a valid gm command. See 'gm --help'.", input.raw_input.join(" ")).as_str());
        }
    }
    Ok(())
}




fn handle_upload_command(rest: &[Arguement], repo_list: &[Repository], repo_names_list: &[String], repo_path_list: &[String]) {
    let current_dir = env::current_dir().expect("error while getting current directory in: (matcher, handle upload command)");
    let current_dir_str = current_dir.to_str().expect("error while converting to string").to_string();
    let is_current_dir_git_repository = repo_path_list.contains(&current_dir_str);

    let mut commit_message = "committed by Git-Manager".to_string();
    let mut force = false;
    let mut branch_name = "main".to_string();
    let mut invalid_args = vec![];

    // Verarbeite Argumente vor dem eigentlichen Match
    for token in rest {
        match token {
            Arguement::FORCE => force = true,
            Arguement::MSG(msg) => commit_message = msg.clone(),
            Arguement::BRANCH(branch) => branch_name = branch.clone(),
            _ => invalid_args.push(format!("{:?}", token)),
        }
    }

    if is_current_dir_git_repository && invalid_args.is_empty() {
        // Wenn im aktuellen Verzeichnis und keine ungültigen Argumente

        // Find the repository name based on the current directory
        let repo_name = repo_list.iter()
            .find(|repo| repo.Path == current_dir_str)
            .map(|repo| &repo.Name)
            .expect("Repository not found for the current directory");

        git_commands::upload(&current_dir_str, &commit_message, force, branch_name);
        return;
    }

    if rest.is_empty() && !is_current_dir_git_repository {
        throw_error("Missing repository name. See 'gm upload --help'.");
        exit(0);
    }

    match rest {
        [Arguement::NAME(ref name), rest @ ..] => {
            let mut commit_message = "committed by Git-Manager".to_string();
            let mut force = false;
            let mut branch_name = "main".to_string();
            let mut invalid_args = vec![];

            for token in rest {
                match token {
                    Arguement::FORCE => force = true,
                    Arguement::MSG(msg) => commit_message = msg.clone(),
                    Arguement::BRANCH(branch) => branch_name = branch.clone(),
                    _ => invalid_args.push(format!("{:?}", token)),
                }
            }

            if !invalid_args.is_empty() {
                throw_error(format!("Invalid Arguments. See 'gm upload --help'.").as_str());
                exit(0);
            }

            if !repo_names_list.contains(&name.to_string()) {
                throw_error(format!("Repository '{}' not found. See 'gm list'.", &name).as_str());
                exit(0);
            }

            for repo in repo_list {
                if &repo.Name == name {
                    git_commands::upload(&repo.Path, &commit_message, force, branch_name.clone());
                }
            }
        },

        [Arguement::ALL, rest @ ..] => {
            let mut commit_message = "committed by Git-Manager".to_string();
            let mut force = false;
            let mut branch_name = "main".to_string();
            let mut invalid_args = vec![];

            for token in rest {
                match token {
                    Arguement::FORCE => force = true,
                    Arguement::MSG(msg) => commit_message = msg.clone(),
                    Arguement::BRANCH(branch) => branch_name = branch.clone(),
                    _ => invalid_args.push(format!("{:?}", token)),
                }
            }

            if !invalid_args.is_empty() {
                throw_error(format!("Invalid Arguments. See 'gm upload all --help'.").as_str());
                exit(0);
            }

            for repo in repo_list {
                git_commands::upload(&repo.Path, &commit_message, force, branch_name.clone());
            }
        },

        _ => {
            throw_error("Invalid upload arguements. See 'gm upload --help'.");
            exit(0);
        }
    }
}
// Weitere Helper-Funktionen und Typen hier ...
