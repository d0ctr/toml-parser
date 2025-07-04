pub mod char_supplier {
    use utf8_chars::BufReadCharsExt as _;

    use crate::{NEWLINE_CHARS, NEWLINE_CR, NEWLINE_LF};

    pub trait Supplier {
        fn get(&mut self) -> Option<char>;
    }

    pub struct Reader<R>
    where R: std::io::Read {
        inner: std::io::BufReader<R>,
    }

    impl<'a,R: std::io::Read> Reader<R> {
        pub fn from_bufreader(inner: std::io::BufReader<R>) -> Reader<R> {
            Self {
                inner
            }
        }

        pub fn new(inner: R) -> Reader<R> {
            Self {
                inner: std::io::BufReader::new(inner)
            }
        }

        pub fn iter(&mut self) -> Iterator<R> {
            Iterator {
                end: false,
                inner: self.inner.chars_raw(),
                last_line: std::string::String::new(),
                line_end: false
            }
        }
    }

    pub struct Iterator<'a, R: std::io::Read> {
        end: bool,
        inner: utf8_chars::CharsRaw<'a, std::io::BufReader<R>>,
        last_line: std::string::String,
        line_end: bool
    }

    impl<R: std::io::Read> Iterator<'_,R> {
        pub fn get_last_line(&mut self) -> &str {
            while !self.line_end {
                self.next();
            }
            return &self.last_line;
        }

        pub fn is_end(&self) -> bool {
            self.end
        }
    }

    impl<R: std::io::Read> std::iter::Iterator for Iterator<'_,R> {
        type Item = char;

        fn next(&mut self) -> Option<Self::Item> {
            if self.end {
                return None;
            }
            
            return match self.inner.next() {
                Some(next) => match next {
                    Ok(c) => {
                        if c == NEWLINE_LF {
                            self.line_end = true;
                        } else if self.line_end && c == NEWLINE_CR {
                            self.line_end = false;
                            return self.next();
                        } else {
                            if self.line_end {
                                self.line_end = false;
                                self.last_line.clear();
                            }
                            self.last_line.push(c);
                        }

                        return Some(c);
                    },
                    Err(_) => {
                        self.end = true;
                        None
                    }
                },
                None => {
                    self.end = true;
                    None
                }
            }
        }
    }

    impl<R: std::io::Read> Supplier for Iterator<'_,R> {
        fn get(&mut self) -> Option<char> {
            self.next()
        }
    }
}
