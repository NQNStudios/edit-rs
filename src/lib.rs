use std::env;
use std::ffi::OsString;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;


extern crate subprocess;
use subprocess::{Exec, Redirection};

// This crate tries to get multi-line string input from your user's favorite
// text editor. Its logic is based on how Git command-line chooses an editor:
// https://github.com/git/git/blob/936d1b989416a95f593bf81ccae8ac62cd83f279/editor.c

pub enum Error {
    NoEditor(&'static str),
    IOError(io::Error),
    SubprocessError(subprocess::PopenError),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IOError(e)
    }
}

impl From<subprocess::PopenError> for Error {
    fn from(e: subprocess::PopenError) -> Self {
        Error::SubprocessError(e)
    }
}

fn is_terminal_dumb() -> bool {
    match env::var("TERM") {
        Err(_) => true,
        Ok(term) => term == "dumb".to_string(),
    }
}

pub fn text_editor() -> Result<OsString, Error> {
    let mut editor = env::var_os("EDITRS_EDITOR");
    let terminal_is_dumb = is_terminal_dumb();

    if editor == None && !terminal_is_dumb {
        editor = env::var_os("VISUAL");
    }
    if editor == None {
        editor = env::var_os("EDITOR");
    }

    if editor == None && terminal_is_dumb {
        return Err(Error::NoEditor("Terminal is dumb, but EDITOR unset"));
    }

    let editor = editor.unwrap_or_else(|| {
        println!("Using vi as default text editor. To change this behavior, set the EDITRS_EDITOR environment variable.");
        OsString::from("vi".to_string())
    });
    Ok(editor)
}

pub fn get_input(default_value: &str) -> Result<String, Error> {
    let file = OpenOptions::new().write(true).create(true).open(".EDITRS_EDITOR_INPUT")?;
    // If a default value is given, write it to the file before opening it
    file.set_len(0)?;
    Exec::cmd("echo").arg(default_value).stdout(Redirection::File(file)).join()?;


    // Open the user's text editor and wait for them to close it
    Exec::cmd(text_editor()?).arg(".EDITRS_EDITOR_INPUT").join()?;
    let mut file = OpenOptions::new().read(true).open(".EDITRS_EDITOR_INPUT")?;
    let mut file_buffer = String::new();
    file.read_to_string(&mut file_buffer)?;
    Ok(file_buffer.trim().to_string())
}
