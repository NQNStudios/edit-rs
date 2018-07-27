use std::env;

// TODO port the code from this file of the Git source:
// https://github.com/git/git/blob/936d1b989416a95f593bf81ccae8ac62cd83f279/editor.c

fn is_terminal_dumb() -> bool {
    match env::var("TERM") {
        Err(_) => true,
        Ok("dumb") => true,
        Ok(_) => false,
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
