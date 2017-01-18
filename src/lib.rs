pub mod event {
    #[derive(PartialEq, Debug, Copy, Clone)]
    pub enum Event {
        Keypress(Key),
    }

    #[derive(PartialEq, Debug, Copy, Clone)]
    pub enum Key {
        Up,
        Down,
        Left,
        Right,
        Control,
        Alt,
        Tab,
        Esc,
        Delete,
        Enter,
        Char(char),
    }
}

pub mod cursor {
    pub struct Cursor {
        pub line: i64,
        pub column: i64,
    }

    impl Cursor {
        pub fn new(line: i64, column: i64) -> Cursor {
            Cursor { line: line, column: column }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
