#![allow(non_snake_case)]
#![allow(unused)]

use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, ErrorKind, Write};
use std::path::{MAIN_SEPARATOR, Path, PathBuf};
use std::process::{exit, Command};

use colored::*;
use reqwest::header::{self, ValueDrain};
use reqwest::Error;
use serde::{Deserialize, Serialize};

// Declare the modules
pub mod config_manager;
pub mod git_commands;
pub mod helper;
pub mod lexer;
pub mod matcher;
pub mod terminal;

// Optionally, re-export items for easier access in other parts of the crate
pub use config_manager::*;
pub use git_commands::*;
pub use helper::*;
pub use lexer::*;
pub use matcher::*;
pub use terminal::*;



#[derive(Deserialize)]
pub struct Repository {
    pub Name: String,
    pub Path: String,
    pub clone_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
    pub username: String,
    pub project_path: String,
}

#[derive(Debug)]
pub struct Input {
    pub commands: Vec<String>,
    pub raw_input: Vec<String>,
    pub command_length: usize,
}


#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Arguement {
    UPLOAD,
    MIGRATE,
    DOWNLOAD,
    LIST,
    HELP,
    CREATE,
    DELETE,
    UPDATE,
    ALL,
    NAME(String),
    SET,
    SHOW,
    MSG(String),
    FORCE,
    PRIVACY(String),
    PATH(String),
    SAVE,
    FROM,
    BRANCH(String),
    CLEAR,
    OWN,
    DOWNLOADED,
    REPO,
    PUNKT,
}

#[derive(Deserialize)]
struct CloneData {
    clone_url: String
}


impl Input{
    pub fn get() -> Input{
        let mut raw_input: Vec<String> = env::args().collect();
        raw_input.remove(0);
        let commands: Vec<String> = raw_input.iter().map(|arg| arg.trim().to_lowercase()).collect();
        let command_length = raw_input.len();

        Input {
            commands,
            raw_input,
            command_length,
        }
    }
}