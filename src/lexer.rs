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
            "set" => Arguement::SET,
            "--force" => Arguement::FORCE,
            "all" => Arguement::ALL,
            "public" => Arguement::PRIVACY("public".to_string()),
            "private" => Arguement::PRIVACY("private".to_string()),
            "save" => Arguement::SAVE,
            "from" => Arguement::FROM,
            "cls" => Arguement::CLEAR,
            "clear" => Arguement::CLEAR,
            other => {
                if other.starts_with("--branch=") {
                    Arguement::BRANCH(other["--branch=".len()..].to_string())
                } else if other.contains("/") {
                    Arguement::PATH(other.to_string())
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

