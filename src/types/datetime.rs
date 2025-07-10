use chrono::{FixedOffset, NaiveDate, NaiveTime};

use crate::{errors::{FormatError, ParserError, UnallowedCharacterReason}, reader::char_supplier::Supplier, types::DateTimeType, CharExt, Counter};

pub struct DateTime;

fn read_digits<T: std::str::FromStr>(input: &mut impl Supplier, mut len: u8, suffixes: &[char], expect_end_of_line: bool) -> Option<T> {
    let mut buf = String::new();

    while let Some(c) = input.get() {
        if suffixes.contains(&c) || (expect_end_of_line && (c.is_whitespace() || c.is_comment_start())) {
            break;
        }
        if len == 0 || !c.is_digit(10) {
            return None;
        }

        buf.push(c);
        len -= 1;
    }

    if len != 0 {
        return None;
    }

    match buf.parse::<T>() {
        Ok(c) => Some(c),
        _ => None
    }
}

impl DateTime {
    pub fn parse_with_buf(buf: &mut impl Supplier, input: &mut impl Supplier) -> Result<DateTimeType, crate::errors::ParserError> {
        let mut _buf = String::new();
        let mut from_buf = true;

        let mut len = 0;

        let mut is_date = false;
        let mut is_time = false;
        
        loop {
            let next: Option<char>= if from_buf {
                let mut _next = buf.get(); 
                if _next.is_none() {
                    from_buf = false;
                    _next = input.get();
                }
                _next
            } else {
                input.get()
            };

            let c = if let Some(_c) = next {
                if _c.is_whitespace() {
                    return ParserError::from(FormatError::UnexpectedEnd);
                } else {
                    _c
                }
            } else {
                return ParserError::from(FormatError::UnexpectedEnd);
            };

            if c == '-' {
                is_date = true;
                break;
            } else if c == ':' {
                is_time = true;
                break;
            } else if !c.is_digit(10) {
                return ParserError::from(FormatError::UnallowedCharacter(c, UnallowedCharacterReason::InTypeDateTime));
            }
        
            _buf.push(c);
            len += 1;
        }
        
        if (is_date && len != 4) || (is_time && len != 2) {
            return ParserError::from(FormatError::UnexpectedEnd);
        }
        
        let date = if is_date {
            let year = _buf.parse::<i32>()?;
            
            let month: u32 = if let Some(_month) = read_digits(input, 2, &['-'], false) {
                _month
            } else {
                return ParserError::from(FormatError::ExpectedSequence("MM".to_string()))
            };
        
            let day: u32 = if let Some(_day) = read_digits(input, 2, &['T'], true) {
                _day
            } else {
                return ParserError::from(FormatError::ExpectedSequence("DD".to_string()))
            };
        
            _buf.clear();
        
            is_time = input.last() == Some('T');
        
            NaiveDate::from_ymd_opt(year, month, day)
        } else {
            None
        };
        
        if !is_time && date.is_some() && input.last() == Some(' ') {
            if let Some(c) = input.get() {
                if c.is_digit(10) {
                    _buf.push(c)
                } else if !c.is_whitespace() && !c.is_comment_start() {
                    return ParserError::from(FormatError::UnallowedCharacter(c, UnallowedCharacterReason::InTypeDate))
                }
            };
        
            is_time = if _buf.len() == 0 {
                false
            } else {
                loop {
                    if _buf.len() > 2 {
                        return ParserError::from(FormatError::ExpectedSequence("HH".to_string()))
                    } else if let Some(c) = input.get() {
                        if c.is_digit(10) {
                            _buf.push(c);
                        } else if c == ':' {
                            break true;
                        } else {
                            return ParserError::from(FormatError::UnallowedCharacter(c, UnallowedCharacterReason::InTypeTime))
                        }
                    }
                }
            }
        }
        
        let time = if is_time {
            let hour: u32 = if !_buf.is_empty() {
                _buf.parse::<u32>()?
            } else if let Some(_hour) = read_digits(input, 2, &[':'], false){
                _hour
            } else {
                return ParserError::from(FormatError::ExpectedSequence("hh:".to_string()));
            };
            _buf.clear();
        
        
            let minute: u32 = if let Some(_minute) = read_digits(input, 2, &[':'], false){
                _minute
            } else {
                return ParserError::from(FormatError::ExpectedSequence("mm:".to_string()));
            };
        
            let nanos: u32 = if let Some(_second) = read_digits::<u32>(input, 2, &['Z', '-', '+', '.'], true) {
                let mut sec = _second * 1_000_000_000;

                if input.last() == Some('.') {
                    _buf.push('.');
                    loop {
                        if let Some(c) = input.get() {
                            if c.is_digit(10) {
                                _buf.push(c);
                            } else if c.is_whitespace() || c.is_comment_start() || ['Z', '-', '+'].contains(&c) {
                                break;
                            } else {
                                return ParserError::from(FormatError::ExpectedSequence(".ffffff".to_string()));
                            }
                        }
                    }

                    let frac = _buf.parse::<f32>()?;
                    sec += (frac * 1_000_000.0) as u32;
                }

                sec
            } else {
                return ParserError::from(FormatError::ExpectedSequence("ss".to_string()));
            };
        
            let _time = NaiveTime::from_hms_nano_opt(hour, minute, 0, nanos);
        
            let offset = match input.last() {
                Some(_c) => match _c {
                    '-' | '+' => {
                        let shift_hour: i32 = if let Some(_hour) = read_digits(input, 2, &[':'], false) {
                            _hour
                        } else {
                            return ParserError::from(FormatError::ExpectedSequence("HH".to_string()));
                        };
        
                        let shift_minute: i32 = if let Some(_minute) = read_digits(input, 2, &[], true) {
                            _minute
                        } else {
                            return ParserError::from(FormatError::ExpectedSequence("mm".to_string()));
                        };
        
                        let mut shift = (shift_hour * 60 + shift_minute) * 60;
                        if _c == '-' {
                            shift *= -1;
                        }
        
                        FixedOffset::east_opt(shift)
                    },
                    'Z' => FixedOffset::east_opt(0),
                    _ => None
                },
                None => None,
            };
        
            (_time, offset)
        } else {
            (None, None)
        };
        
        return match (date, time) {
            (Some(date), (None, None)) => Ok(DateTimeType::Date(date)),
            (None, (Some(time), None)) => Ok(DateTimeType::Time(time)),
            (Some(date), (Some(time), None)) => Ok(DateTimeType::DateTime(date.and_time(time))),
            (Some(date), (Some(time), Some(offset))) => match date.and_time(time).checked_add_offset(offset) {
                Some(datetime) => Ok(DateTimeType::DateTime(datetime)),
                None => ParserError::from(FormatError::Unknown(format!("failed to contruct datetime from {date} {time} {offset}")))
            }
            _ => ParserError::from(FormatError::EmptyValue),
        };

    }
}

impl super::TypeParser<DateTimeType> for DateTime {
    fn parse(first: char, input: &mut impl Supplier) -> Result<DateTimeType, crate::errors::ParserError> {
        todo!()
    }

}