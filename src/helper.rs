pub fn print_list_help() {
    println!("");
    println!("Usage: gm list

List all of your downloaded directory from your project directory.");
}

pub fn print_general_help() {
    println!("");
    println!("Usage: gm <command> [<args>]

Available commands:
    upload       Upload changes to a git repository
    upload all   Upload all known repositories to git
    list         List all downloaded repositories in your project directory
    --help       Show help about a specific command");
}
pub fn print_upload_all_help() {
    println!("");
    println!("Usage: gm upload all [--force]
    
Upload options:
    --force             Optional force upload changes
    
Examples:
    gm upload all --force
    gm upload my_repo");
}

pub fn print_upload_help() {
    println!("");
    println!("Usage: gm upload <repo_name> [<commit_message>] [--force] [--branch=<branch_name>]
    
Upload options:
    <commit_message>        Optional commit message (default: 'committed by Git-Manager')
    --force                 Optional force upload changes
    --branch=<branch_name>  Optional branch name (default: 'main')
    
Examples:
    gm upload my_repo 'Initial commit' --force --branch=main
    gm upload my_repo --force
    gm upload my_repo 'Initial commit'
    gm upload my_repo --branch=feature_branch
    gm upload my_repo

Extra Info:
    Upload can also be used while you are in a repository:
    
    Usage: gm upload [<commit message>] [--force] [--branch=<branch_name>]

    Examples:
        gm upload
        gm upload --force
        gm upload 'commit message'

    Note: 
    - You have to be in a git repository otherwise you well get an error");



}

pub fn print_download_all_help() {
    println!("");
    println!("Usage: gm download all from <name>

Examples:
    gm download all from user_name

Extra Info:
    - Make sure you have your API key configured if you are downloading your own repositories.");
}


pub fn print_download_help() {
    println!("");
    println!("Usage: gm download <repo_name> from <user_name>

Examples:
    gm download random_repo from my_friend");
}