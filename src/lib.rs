use std::env;
use std::ffi::OsString;

// TODO port the code from this file of the Git source:
// https://github.com/git/git/blob/936d1b989416a95f593bf81ccae8ac62cd83f279/editor.c

pub enum Error {
    NoEditor(&'static str)
}

fn is_terminal_dumb() -> bool {
    match env::var("TERM") {
        Err(_) => true,
        Ok(term) => term == "dumb".to_string(),
    }
}

fn text_editor() -> Result<OsString, Error> {
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

    let editor = editor.unwrap_or(OsString::from("vi".to_string()));
    Ok(editor)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
