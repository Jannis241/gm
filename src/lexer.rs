use std::env::consts::ARCH;

use gm::*;

pub fn create_tokens(input: &Input) -> Vec<Arguement> {
    let mut token_list = Vec::new();
    for command in &input.raw_input {
        let token = match command.as_str() {
            "upload" => Arguement::UPLOAD,
            "update" => Arguement::UPDATE,
            "migrate" => Arguement::MIGRATE,
            "list" => Arguement::LIST,
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
            other => {
                if command.contains("/") {
                    Arguement::PATH(other.to_string())
                } 
                else if command.contains(" ") || command.contains("' '"){
                    Arguement::MSG(other.to_string())
                }
                
                else {
                    Arguement::NAME(other.to_string())
                }
            }
        };

        token_list.push(token);
    }
    token_list
}
