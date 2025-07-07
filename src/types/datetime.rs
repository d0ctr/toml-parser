use chrono::{FixedOffset, NaiveDate, NaiveTime};

use crate::{errors::{FormatError, ParserError, UnallowedCharacterReason}, reader::char_supplier::Supplier, types::DateTimeType, CharExt, Counter};

pub struct DateTime;

fn read_digits<T: std::str::FromStr>(input: &mut impl Supplier, mut len: u8, suffixes: &[char]) -> Option<T> {
    let mut buf = String::new();

    while let Some(c) = input.get() {
        if suffixes.contains(&c) {
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

impl super::TypeParser<DateTimeType> for DateTime {
    fn parse(first: char, input: &mut impl Supplier) -> Result<DateTimeType, crate::errors::ParserError> {
        let mut buf = String::new();

        let mut len = 1;
        let mut c = first;

        let mut is_date = false;
        let mut is_time = false;

        loop {
            if c == '-' || c == ':' {
                is_date = c == '-';
                is_time = c == ':';
                break;
            } else if !c.is_digit(10) {
                return ParserError::from(FormatError::UnallowedCharacter(c, UnallowedCharacterReason::InTypeDateTime));
            }

            buf.push(c);
            len += 1;

            c = if let Some(_c) = input.get() {
                if _c.is_whitespace() {
                    return ParserError::from(FormatError::UnexpectedEnd);
                } else {
                    _c
                }
            } else {
                return ParserError::from(FormatError::UnexpectedEnd);
            };
        }

        if (is_date && len != 4) || (is_time && len != 2) {
            return ParserError::from(FormatError::UnexpectedEnd);
        }

        let date = if is_date {
            let year = buf.parse::<i32>()?;
            
            let month: u32 = if let Some(_month) = read_digits(input, 2, &['-']) {
                _month
            } else {
                return ParserError::from(FormatError::ExpectedSequence("MM".to_string()))
            };

            let day: u32 = if let Some(_day) = read_digits(input, 2, &[' ', 'T', '\n', '#']) {
                _day
            } else {
                return ParserError::from(FormatError::ExpectedSequence("DD".to_string()))
            };

            buf.clear();

            is_time = input.last() == Some('T');

            NaiveDate::from_ymd_opt(year, month, day)
        } else {
            None
        };
        
        if !is_time && date.is_some() && input.last() == Some(' ') {
            if let Some(_c) = input.get() {
                if c.is_digit(10) {
                    buf.push(c)
                } else if !_c.is_whitespace() && !_c.is_comment_start() {
                    return ParserError::from(FormatError::UnallowedCharacter(c, UnallowedCharacterReason::InTypeDate))
                }
            };

            is_time = if buf.len() == 0 {
                false
            } else {
                loop {
                    if buf.len() > 2 {
                        return ParserError::from(FormatError::ExpectedSequence("HH".to_string()))
                    } else if let Some(_c) = input.get() {
                        if _c.is_digit(10) {
                            buf.push(_c);
                        } else if _c == ':' {
                            break true;
                        } else {
                            return ParserError::from(FormatError::UnallowedCharacter(c, UnallowedCharacterReason::InTypeTime))
                        }
                    }
                }
            }
        }

        let time = if is_time {
            let hour = buf.parse::<u32>()?;

            buf.clear();

            let minute: u32 = if let Some(_minute) = read_digits(input, 2, &[':']){
                _minute
            } else {
                return ParserError::from(FormatError::ExpectedSequence("mm".to_string()));
            };

            let millis: u32 = if let Some(_second) = read_digits::<f64>(input, 2, &['Z', '-', '+', ' ', '\n', '#']) {
                (_second * 1_000_000.0).floor() as u32
            } else {
                return ParserError::from(FormatError::ExpectedSequence("hh".to_string()));
            };

            let time = NaiveTime::from_hms_milli_opt(hour, minute, 0, millis);

            let offset = match input.last() {
                Some(_c) => match _c {
                    '-' | '+' => {
                        buf.push(_c);
                        let shift_hour: i32 = if let Some(_hour) = read_digits(input, 2, &[':']) {
                            _hour
                        } else {
                            return ParserError::from(FormatError::ExpectedSequence("HH".to_string()));
                        };

                        let shift_minute: i32 = if let Some(_minute) = read_digits(input, 2, &[]) {
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

            (time, offset)
        } else {
            (None, None)
        };

        return match (date, time) {
            (None, (None, None)) => ParserError::from(FormatError::EmptyValue),
            (Some(date), (None, None)) => Ok(DateTimeType::Date(date)),
            (None, (Some(time), None)) => Ok(DateTimeType::Time(time)),
            (Some(date), (Some(time), None)) => Ok(DateTimeType::DateTime(date.and_time(time))),
            (Some(date), (Some(time), Some(offset))) => match date.and_time(time).checked_add_offset(offset) {
                Some(datetime) => Ok(DateTimeType::DateTime(datetime)),
                None => ParserError::from(FormatError::EmptyValue)
            }
            _ => ParserError::from(FormatError::EmptyValue)
        };
    }

}