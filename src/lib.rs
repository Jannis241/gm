#![allow(non_snake_case)]
#![allow(unused)]

use std::env;
use std::collections::btree_map::Range;
use std::io::{self, ErrorKind, Write}; 
use std::fs::{self, File, OpenOptions}; 
use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use colored::*; 
use reqwest::header::ValueDrain;
use serde::{Serialize, Deserialize};
use reqwest::{header, Error};
use std::process::{exit, Command};
use colored::*; 



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