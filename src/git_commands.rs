

use crate::*;

pub fn delete_repo(repoName: &String, api_key: &String){
    println!("The user wants to delete the repo {}", repoName);
}

pub fn get_repo_path_by_name(name: &String, repoList: &[Repository]) -> Option<String> {
    let mut repoPath = None;
    for repo in repoList {
        if &repo.Name == name {
            repoPath = Some(repo.Path.clone());
        }
    }
    repoPath
}

pub fn delete_branch(repoName: &String, branchName: &String, api_key: &String){
    println!("the user wants to delete the branch {} from the repo {}", branchName, repoName);
}

pub fn create_repo(repoName: &String, public: &bool, path: &String, api_key: &String){
    println!("the user wants to create a repo:");
    println!("Name: {}", repoName);
    println!("Path (wo das repo ist, Optional): {}", path);
    println!("Public: {}", public);
    println!("API-Key: {}", api_key);
}

pub fn create_branch(repoName: &String, branchName: &String, api_key: &String){
    println!("the user wants to create a branch in {} with the name {} -> api key: {}", repoName, branchName, api_key);
}

pub fn upload(repoPath: &String, commitMessage: &String, force: bool, branch_name: String){
    println!("the user wants to upload: {}", repoPath);
    println!("commit msg: {}", commitMessage);
    println!("Force: {}", force);
    println!("branch: {}", branch_name)
    
}

pub fn update(repoPath: &String, force: bool, branch_name: String){
    println!("the user wants to update: {}", repoPath);
    println!("force: {}", force);
}

pub fn download(repo_name: &String, username: &String, path: &str) {
    let clone_url = format!("https://github.com/{}/{}.git", username, repo_name);
    let target_path = path;
    let output = Command::new("git")
    .arg("clone")
    .arg(&clone_url)
    .arg(format!("{}/{}", target_path, extract_repo_name(&clone_url)))
    .output()
    .expect("Failed to execute git command");

    if !output.status.success() {
        eprintln!("Failed to clone repo: {}", clone_url);
    } else {
        println!("Successfully cloned: {}", clone_url);
    }
}

pub fn migrate(projectPath: &String, repoName: &String, public: bool, api_key: &String){
    println!("the user wants to migrate a project:");
    println!("repo name: {}", repoName);
    println!("current project path: {}", projectPath);
    println!("Public: {}", public);
    println!("API-Key: {}", api_key);
}

pub fn find_git_repos(path: &Path) -> Vec<PathBuf> {
    let mut git_repos = Vec::new();

    // Rekursiv alle Einträge im Pfad durchgehen
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    // Wenn das Verzeichnis noch nicht als Git-Repo gefunden wurde
                    if !git_repos.iter().any(|repo: &PathBuf | repo.starts_with(&entry_path)) {
                        // Nach .git-Dateien suchen
                        let git_dir = entry_path.join(".git");
                        if git_dir.exists() {
                            git_repos.push(entry_path);
                        } else {
                            // Rekursiv in Unterverzeichnisse suchen
                            git_repos.extend(find_git_repos(&entry_path));
                        }
                    }
                }
            }
        }
    }

    git_repos
}


pub fn clone_all_repos(repos: &[CloneData], target_path: String){
    for repo in repos {
        let output = Command::new("git")
            .arg("clone")
            .arg(&repo.clone_url)
            .arg(format!("{}/{}", target_path, extract_repo_name(&repo.clone_url)))
            .output()
            .expect("Failed to execute git command");

        if !output.status.success() {
            eprintln!("Failed to clone repo: {}", repo.clone_url);
        } else {
            println!("Successfully cloned: {}", repo.clone_url);
        }
    }
}





#[derive(Debug, Error)]
pub enum RepoError {
    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Invalid API key or unauthorized access")]
    Unauthorized,

    #[error("User not found")]
    UserNotFound,

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Unexpected server error: {0}")]
    ServerError(String),

    #[error("Failed to parse response: {0}")]
    ParseError(String),
}

pub async fn get_all_repos_of_user(username: &str, token: Option<String>) -> Result<Vec<CloneData>, RepoError> {
    // Erstelle den Client    
    let client = reqwest::Client::new();

    // Erstelle die URL
    let url = format!("https://api.github.com/users/{}/repos?per_page=100", username);

    // Füge optional den Authorization Header hinzu
    let mut request = client.get(&url).header("User-Agent", "rust-github-client");

    if let Some(token) = token {
        println!("Getting authorization with API key...");
        let auth_value = format!("token {}", token);
        request = request.header(header::AUTHORIZATION, auth_value);
    }

    // Sende die Anfrage
    let response = request.send().await.map_err(|e| RepoError::NetworkError(e.to_string()))?;
    
    // Überprüfe den Status der Antwort
    match response.status() {
        reqwest::StatusCode::OK => {
            // Parste die JSON Antwort
            let repos: Vec<CloneData> = response.json().await.map_err(|e| RepoError::ParseError(e.to_string()))?;
            Ok(repos)
        },
        reqwest::StatusCode::UNAUTHORIZED => Err(RepoError::Unauthorized),
        reqwest::StatusCode::NOT_FOUND => Err(RepoError::UserNotFound),
        reqwest::StatusCode::FORBIDDEN if response.headers().get("X-RateLimit-Remaining").map_or(false, |v| v == "0") => {
            Err(RepoError::RateLimitExceeded)
        },
        status if status.is_server_error() => Err(RepoError::ServerError(status.to_string())),
        _ => Err(RepoError::NetworkError(response.status().to_string())),
    }
}

pub fn extract_repo_name(clone_url: &str) -> &str {
    Path::new(clone_url)
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown-repo")
}

pub fn get_all_repositories(user_config: &Config) -> Vec<Repository>{
    let project_path_str = user_config.project_path.as_str();
    let project_path = Path::new(project_path_str);
    let git_repos_paths: Vec<PathBuf> = git_commands::find_git_repos(project_path);

    let mut repo_list = Vec::new();
    for path in git_repos_paths.clone() {
        let cleanPath = path.to_str().unwrap_or("error -> path is none");
        let cleanPathSplit = cleanPath.split_inclusive("/").collect::<Vec<&str>>();
        let name = cleanPathSplit.last().unwrap().to_string();
        let repo = Repository {
            Name: name.clone(),
            Path: cleanPath.to_string(),
            clone_url: "https://github.com/".to_string() + &user_config.username+ "/" + &name + ".git",
        };
        repo_list.push(repo);
        
    }

    repo_list
}

pub fn print_repo_list(repo_paths: &Vec<String>) {
    for path in repo_paths.clone() {
        let cleanPathSplit = path.split_inclusive("/").collect::<Vec<&str>>();
        
        println!("{}{}", cleanPathSplit[0..cleanPathSplit.len() - 2].join("").italic(), cleanPathSplit.last().unwrap().blue().italic().bold());
    }
    println!("");
    println!("Found {} repositories", repo_paths.len().to_string().blue());
    
}


#[derive(Debug)]
pub enum SearchError {
    NotFound,
    MultipleFound,
    IoError(io::Error),
}

impl From<io::Error> for SearchError {
    fn from(error: io::Error) -> Self {
        SearchError::IoError(error)
    }
}

pub fn find_file_in_path(path: &str, name: &str) -> Result<String, SearchError> {
    let mut found_files = vec![];

    fn search_directory(path: &PathBuf, name: &str, found_files: &mut Vec<PathBuf>) -> io::Result<()> {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                if entry_path.file_name() == Some(std::ffi::OsStr::new(name)) {
                    found_files.push(entry_path.clone());
                }
                search_directory(&entry_path, name, found_files)?;
            }
        }
        Ok(())
    }

    let path = PathBuf::from(path);
    search_directory(&path, name, &mut found_files)?;

    match found_files.len() {
        0 => Err(SearchError::NotFound),
        1 => Ok(found_files.pop().unwrap().to_string_lossy().into_owned()),
        _ => Err(SearchError::MultipleFound),
    }
}

pub fn update_repos(repo_list: &mut Vec<Repository>, repo_names_list: &mut Vec<String>, repo_path_list: &mut Vec<String>, user_config: &Config) {
    // Aktualisiere die Repo-Liste, Namen und Pfade im Falle, dass ein Repo hinzugefügt oder gelöscht wurde
    
    let new_repo_list = git_commands::get_all_repositories(user_config);
    *repo_list = new_repo_list;  // Übernehme die neuen Repos in die ursprüngliche Liste
    
    repo_names_list.clear();
    repo_path_list.clear();
    for repo in repo_list.iter() {
        repo_names_list.push(repo.Name.clone());
        repo_path_list.push(repo.Path.clone());
    }
}
