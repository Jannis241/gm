use env::consts::ARCH;

use crate::*;
pub fn create_tokens(input: &Input) -> Vec<Arguement> {
    let mut token_list = Vec::new();
    for command in &input.raw_input {
        let token = match command.as_str() {
            "upload" => Arguement::UPLOAD,
            "update" => Arguement::UPDATE,
            "migrate" => Arguement::MIGRATE,
            "list" => Arguement::LIST,
            "ls" => Arguement::LIST,
            "--help" => Arguement::HELP,
            "download" => Arguement::DOWNLOAD, 
            "create" => Arguement::CREATE,
            "delete" => Arguement::DELETE,
            "show" => Arguement::SHOW,
            "edit" => Arguement::EDIT,
            "config" => Arguement::CONFIG,
            "set" => Arguement::SET,
            "--force" => Arguement::FORCE,
            "all" => Arguement::ALL,
            "public" => Arguement::PRIVACY("public".to_string()),
            "private" => Arguement::PRIVACY("private".to_string()),
            "save" => Arguement::SAVE,
            "from" => Arguement::FROM,
            "cls" => Arguement::CLEAR,
            "clear" => Arguement::CLEAR,
            "own" => Arguement::OWN,
            "." => Arguement::PUNKT,
            "downloaded" => Arguement::DOWNLOADED,
            "repo" => Arguement::REPO,
            other => {
                if other.starts_with("--branch=") {
                    Arguement::BRANCH(other["--branch=".len()..].to_string())
                }   else if other.starts_with("--path=") {
                    Arguement::PATH(other["--path=".len()..].to_string())
                } else if other.starts_with("--username=") {
                    Arguement::USERNAME(other["--username=".len()..].to_string())
                } else if other.starts_with("--key=") {
                    Arguement::API_KEY(other["--key=".len()..].to_string())
                } else if other.contains(" ") || other.contains("' '") {
                    Arguement::MSG(other.to_string())
                } else {
                    Arguement::NAME(other.to_string())
                }
            }
        };

        token_list.push(token);
    }
    token_list
}

