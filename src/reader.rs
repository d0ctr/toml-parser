pub mod char_supplier {
    use utf8_chars::BufReadCharsExt as _;

    use crate::{CharExt, NEWLINE_CRLF, NEWLINE_LF_STR};

    pub trait Supplier {
        fn get(&mut self) -> Option<char>;
    }

    pub struct Reader<R>
    where R: std::io::Read {
        inner: std::io::BufReader<R>,
    }

    impl<'a,R: std::io::Read> Reader<R> {

        pub fn new(inner: R) -> Reader<R> {
            Self {
                inner: std::io::BufReader::new(inner)
            }
        }

        pub fn iter_with_debug(&mut self) -> DebuggingIterator<R> {
            DebuggingIterator {
                end: false,
                inner: self.inner.chars_raw(),
                last_line: std::string::String::new(),
                line_end_buf: std::string::String::with_capacity(2)
            }
        }

        pub fn iter(&mut self) -> Iterator<R> {
            Iterator {
                end: false,
                inner: self.inner.chars_raw(),
                line_end_buf: std::string::String::with_capacity(2)
            }
        }
    }

    pub struct DebuggingIterator<'a, R: std::io::Read> {
        end: bool,
        inner: utf8_chars::CharsRaw<'a, std::io::BufReader<R>>,
        last_line: std::string::String,
        line_end_buf: std::string::String
    }

    impl<R: std::io::Read> DebuggingIterator<'_,R> {
        pub fn get_last_line(&mut self) -> &str {
            while !self.is_line_end() {
                self.next();
            }
            return &self.last_line;
        }

        pub fn is_end(&self) -> bool {
            self.end
        }

        pub fn is_line_end(&self) -> bool {
            match self.line_end_buf.as_str() {
                NEWLINE_CRLF | NEWLINE_LF_STR => true,
                _ => false,
            }
        }
    }

    impl<R: std::io::Read> std::iter::Iterator for DebuggingIterator<'_,R> {
        type Item = char;

        fn next(&mut self) -> Option<Self::Item> {
            if self.end {
                return None;
            }
            
            return match self.inner.next() {
                Some(next) => match next {
                    Ok(c) => {
                        if c.is_linebreak() && !self.is_line_end() {
                            self.line_end_buf.push(c);
                        } else {
                            if self.is_line_end() {
                                self.line_end_buf.clear();
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

    impl<R: std::io::Read> Supplier for DebuggingIterator<'_,R> {
        fn get(&mut self) -> Option<char> {
            self.next()
        }
    }

    pub struct Iterator<'a, R: std::io::Read> {
        end: bool,
        inner: utf8_chars::CharsRaw<'a, std::io::BufReader<R>>,
        line_end_buf: std::string::String
    }
    
    impl<R: std::io::Read> Iterator<'_,R> {
        pub fn is_end(&self) -> bool {
            self.end
        }

        pub fn is_line_end(&self) -> bool {
            match self.line_end_buf.as_str() {
                NEWLINE_CRLF | NEWLINE_LF_STR => true,
                _ => false,
            }
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
                        if c.is_linebreak() && !self.is_line_end() {
                            self.line_end_buf.push(c);
                        } else {
                            if self.is_line_end() {
                                self.line_end_buf.clear();
                            }
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
