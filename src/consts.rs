pub const COMMENT_START   : char      = '\u{0023}';

pub const WHITESPACE_TAB  : char      = 0x0A as char;
pub const WHITESPACE_SPACE: char      = 0x20 as char;

pub const ESCAPE_START    : char      = '\\';
pub const ESCAPE_START_STR: &str      = r"\";

pub const DOUBLE_QUOTE    : char      = '"';
pub const SINGLE_QUOTE    : char      = '\'';

pub const NEWLINE_LF      : char      = '\n';
pub const NEWLINE_CR      : char      = '\r';
pub const NEWLINE_CRLF    : &str      = "\r\n";
pub const NEWLINE_LF_STR  : &str      = "\n";

const fn special_control_characters() -> [char; 32] {
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
pub const SPECIAL_CTRL_CHARACTERS: [char; 32] = special_control_characters();

pub const UNICODE_LOW_ESCAPE_START : &str  = r"\u";
pub const UNICODE_HIGH_ESCAPE_START:  &str = r"\U";

pub const ESCAPE_SEQUENCE_TO_CHAR: [(&str,char); 7] = [
    (r"\b",  '\u{0008}'),
    (r"\t",  '\u{0009}'),
    (r"\n",  '\u{000A}'),
    (r"\f",  '\u{000C}'),
    (r"\r",  '\u{000D}'),
    ("\\\"", '\u{0022}'),
    (r"\\",  '\u{005C}')
];
