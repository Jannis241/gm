pub fn print_config_help(){
    println!("Config options:

    set config      Change your configuration settings
    show config     Display the current configuration settings 
    
    See 'gm <option> --help' for more info
    ")
}

pub fn print_list_help() {
    println!("");
    println!("Usage: gm list [<options>]

List options:
    list downloaded     List all downloaded git repositories in your project directory 
    list <user>         List all repositories owned by the specified user
    list own            List all of your own repositories (including private)

Examples:
    gm list downloaded
    gm list my_friend
    gm list own

Extra Info:
    - To list your own repositories, ensure your API key is configured in the config. 
    - Check with 'gm show config' or see 'gm set config --help' for more info
");
}

pub fn print_show_config_help() {
    println!("");
    println!("Usage: gm show config

Show config options:
    show config     Display the current configuration settings

Examples:
    gm show config

Extra Info:
    - This command displays the current configuration, including your username, API key, and project path.
");
}

pub fn print_set_config_help() {
    println!("");
    println!("Usage: gm set config <option> <value>

Set config options:
    set config <option> <value>  Modify a specific configuration setting

Options:
    username    Set the username for Git operations
    key         Update the API key for accessing repositories
    path        Define the path where repositories will be cloned

Examples:
    gm set config username new_user
    gm set config key new_api_key
    gm set config path /new/project/path

Extra Info:
    - Make sure to use the correct option names and provide valid values
    - check by executing 'gm show config' or see 'gm set config --help' for more details.
");
}

pub fn print_clear_help() {
    println!("");
    println!("Usage: gm clear / cls

Clear your current terminal screen.");
}

pub fn print_general_help() {
    println!("");
    println!("Usage: gm <command> [<args>]

Available commands:
    upload          Upload changes to a specified git repository
    upload all      Upload all known repositories to git
    download        Download a specific repository from a user
    download all    Download all repositories from a user
    clear           Clear the terminal screen
    show config     Display your current configuration settings
    set config      Modify your configuration settings
    list            List all downloaded repositories in your project directory
    --help          Show help information for a specific command

    For detailed usage, see 'gm <command> --help'
"); 
}

pub fn print_upload_all_help() {
    println!("");
    println!("Usage: gm upload all [--force]
    
Upload options:
    --force             Forcefully upload changes, overwriting if necessary
    
Examples:
    gm upload all --force
    gm upload all
    
Extra Info:
    - This command uploads changes to all repositories that are known and tracked by the system.
    - Check your repository list with 'gm list downloaded' or see 'gm list --help' for more info
");
}

pub fn print_upload_help() {
    println!("");
    println!("Usage: gm upload <repo_name> [<commit_message>] [--force] [--branch=<branch_name>]
    
Upload options:
    <commit_message>        Optional commit message (default: 'committed by Git-Manager')
    --force                 Forcefully upload changes, overwriting if necessary
    --branch=<branch_name>  Specify the branch to upload changes to (default = 'main')
    
Examples:
    gm upload my_repo 'Initial commit' --force --branch=main
    gm upload my_repo --force
    gm upload my_repo 'Initial commit'
    gm upload my_repo --branch=feature_branch
    gm upload my_repo

Extra Info:
    If you are in a git repository directory, you can also use:
    
    Usage: gm upload . [<commit message>] [--force] [--branch=<branch_name>]

    Examples:
        gm upload . 
        gm upload . --force
        gm upload . 'commit message'
        gm upload . --branch=test_branch

    Note: 
    - Ensure you are in a valid git repository directory or specify a repository name.
    - Make sure to you have configured your project path correctly. See 'gm show config' or 'gm set config --help'
    - You can check your downloaded repositories with 'gm list downloaded'
");
}

pub fn print_download_all_help() {
    println!("");
    println!("Usage: gm download all from <name>

Examples:
    gm download all from my_friend

Extra Info:
    - Ensure your API key is configured if you are downloading your own repositories. 
    - Check by executing 'gm show config'
    - For more info see 'gm set config --help'
");
}

pub fn print_download_help() {
    println!("");
    println!("Usage: gm download <repo_name> from <user_name>

Examples:
    gm download random_repo from my_friend
    
Extra Info:
    - This command downloads a specific repository from the specified user.
");
}
