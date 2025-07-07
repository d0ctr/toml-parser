pub mod char_supplier {
    use utf8_chars::BufReadCharsExt as _;

    use crate::{CharExt, NEWLINE_CR, NEWLINE_CRLF, NEWLINE_LF_STR};

    pub trait Supplier {
        fn get(&mut self) -> Option<char>;
        fn last(&self) -> Option<char>;
        fn as_iter(&mut self) -> &mut impl std::iter::Iterator<Item=char>;
    }

    pub struct Reader<R: std::io::Read> {
        inner: std::io::BufReader<R>,
    }

    impl<'a,R: std::io::Read> Reader<R> {
        pub fn new(inner: R) -> Reader<R> {
            Self {
                inner: std::io::BufReader::new(inner)
            }
        }

        pub fn iter_with_debug(&mut self) -> DebuggingIterator<R> {
            DebuggingIterator::new(self.inner.chars_raw())
        }

        pub fn iter(&mut self) -> Iterator<R> {
            Iterator::new(self.inner.chars_raw())
        }
    }

    pub struct DebuggingIterator<'a, R: std::io::Read + ?Sized> {
        end: bool,
        inner: utf8_chars::CharsRaw<'a, std::io::BufReader<R>>,
        last_line: std::string::String,
        line_end_buf: std::string::String,
        needle: (usize,usize),
        last: Option<char>,
    }

    impl<R: std::io::Read> DebuggingIterator<'_,R> {
        pub fn new<'a>(inner: utf8_chars::CharsRaw<'a, std::io::BufReader<R>>) -> DebuggingIterator<'a, R> {
            DebuggingIterator {
                inner,
                end: false,
                last_line: std::string::String::new(),
                line_end_buf: std::string::String::with_capacity(2),
                needle: (0,0),
                last: None,
            }
        }

        pub fn get_last_line(&mut self) -> &str {
            while !(self.is_line_end() || self.is_end()) {
                self.next();
            }
            return &self.last_line;
        }

        pub fn get_needle(&self) -> (usize,usize) {
            self.needle
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

        fn new_line(&mut self) {
            self.last_line.clear();
            self.needle.1 = 0;
            self.needle.0 += 1;
            self.line_end_buf.clear();
        }
    }

    impl<R: std::io::Read> std::iter::Iterator for DebuggingIterator<'_,R> {
        type Item = char;

        fn next(&mut self) -> Option<Self::Item> {
            if self.end {
                return None;
            }
            
            match self.inner.next() {
                Some(Ok(c)) => {
                    if c.is_linebreak() && !self.is_line_end() {
                        self.line_end_buf.push(c);

                        if c == NEWLINE_CR {
                            return self.next();
                        }
                    } else {
                        if self.is_line_end() {
                            self.new_line();
                        }
                        
                        self.last_line.push(c);
                        self.needle.1 += 1;
                    }

                    self.last = Some(c);
                },
                Some(Err(_)) | None => {
                    self.end = true;
                    self.last = None;
                }
            }

            self.last
        }
    }

    impl<R: std::io::Read> Supplier for DebuggingIterator<'_,R> {
        fn get(&mut self) -> Option<char> {
            self.next()
        }

        fn last(&self) -> Option<char> {
            self.last
        }
        
        fn as_iter(&mut self) -> &mut impl std::iter::Iterator<Item=char> {
            self
        }
    }

    pub struct Iterator<'a, R: std::io::Read> {
        end: bool,
        inner: utf8_chars::CharsRaw<'a, std::io::BufReader<R>>,
        line_end_buf: std::string::String,
        last: Option<char>,
    }
    
    impl<R: std::io::Read> Iterator<'_,R> {
        pub fn new<'a>(inner: utf8_chars::CharsRaw<'a, std::io::BufReader<R>>) -> Iterator<'a, R> {
            Iterator {
                inner,
                end: false,
                line_end_buf: std::string::String::with_capacity(2),
                last: None,
            }
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

    impl<R: std::io::Read> std::iter::Iterator for Iterator<'_,R> {
        type Item = char;

        fn next(&mut self) -> Option<Self::Item> {
            if self.end {
                return None;
            }
            
            match self.inner.next() {
                Some(Ok(c)) => {
                    if c.is_linebreak() && !self.is_line_end() {
                        self.line_end_buf.push(c);

                        if c == NEWLINE_CR {
                            return self.next();
                        }
                    } else if self.is_line_end() {
                        self.line_end_buf.clear();
                    }

                    self.last = Some(c);
                },
                Some(Err(_)) | None => {
                    self.end = true;
                    self.last = None;
                }
            }

            self.last
        }
    }
    
    impl<R: std::io::Read> Supplier for Iterator<'_,R> {
        fn get(&mut self) -> Option<char> {
            self.next()
        }

        fn last(&self) -> Option<char> {
            self.last
        }
        
        fn as_iter(&mut self) -> &mut impl std::iter::Iterator<Item=char> {
            self
        }
    }

    pub struct ToSupplier<'a> {
        iter: std::str::Chars<'a>,
        last: Option<char>
    }

    impl ToSupplier<'_> {
        pub fn from_string(string: &std::string::String) -> impl Supplier {
            ToSupplier {
                iter: string.chars(),
                last: None
            }
        }
    }

    impl Supplier for ToSupplier<'_> {
        fn get(&mut self) -> Option<char> {
            self.last = self.iter.next();

            return self.last
        }
    
        fn last(&self) -> Option<char> {
            self.last
        }
    
        fn as_iter(&mut self) -> &mut impl std::iter::Iterator<Item=char> {
            &mut self.iter
        }
    }
}
