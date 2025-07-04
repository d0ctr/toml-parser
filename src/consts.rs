pub const COMMENT_START: char = '\u{0023}';

pub const WHITESPACE_TAB  : char = '\t';
pub const WHITESPACE_SPACE: char = ' ';

pub const NEWLINE_LF: char = '\n';
pub const NEWLINE_CR: char = '\r';
pub const NEWLINE_CRLF: &str = "\r\n";
pub const NEWLINE_CHARS: [char; 2] = ['\n','\r'];

const fn control_characters() -> [char; 32] {
    let mut arr: [char; 32] = ['\0'; 32];
    let mut i = 0;

    let mut code: u32 = 0x0000;
    while code <= 0x0008 {
        arr[i] = char::from_u32(code).unwrap();
        i += 1;
        code += 1;
    }

    code = 0x000A;
    while code <= 0x001F {
        arr[i] = char::from_u32(code).unwrap();
        i += 1;
        code += 1;
    }

    arr[i] = char::from_u32(0x007f).unwrap();

    arr
}
pub const ALLOWED_CTRL_CHARACTERS: [char; 32] = control_characters();

pub const UTF_8_ESCAPE_SHORT: &str = r"\u([0-9A-Fa-f]{4})";
pub const UTF_8_ESCAPE_LONG:  &str = r"\U([0-9A-Fa-f]{8})";

pub const STRING_REPLACEMENTS: [(&str,char); 7] = [
    (r"\b",  '\u{0008}'),
    (r"\t",  '\u{0009}'),
    (r"\n",  '\u{000A}'),
    (r"\f",  '\u{000C}'),
    (r"\r",  '\u{000D}'),
    ("\\\"", '\u{0022}'),
    (r"\\",  '\u{005C}')
];

const fn unicode_sequence_chars() -> [char; 22] {
    let mut arr: [char; 22] = ['\0'; 22];
    let mut i = 0;

    let mut code = '0' as u8;
    while code <= '9' as u8 {
        arr[i] = code as char;
        i += 1;
        code += 1;
    }

    let mut code = 'a' as u8;
    while code <= 'f' as u8 {
        arr[i] = code as char;
        i += 1;
        code += 1;
    }

    let mut code = 'A' as u8;
    while code <= 'F' as u8 {
        arr[i] = code as char;
        i += 1;
        code += 1;
    }

    arr
}

pub const UNICODE_SEQUENCE_CHARS: [char; 22] = unicode_sequence_chars();
