use std::convert::From;
use std::error::Error;
use std::fmt;
use std::string::FromUtf8Error;

include!(concat!(env!("OUT_DIR"), "/maps.rs"));

#[derive(Debug)]
pub struct TranslationError {
    pub why: String,
}

impl fmt::Display for TranslationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.why)
    }
}

impl Error for TranslationError {}
impl From<FromUtf8Error> for TranslationError {
    fn from(error: FromUtf8Error) -> Self {
        TranslationError {
            why: format!("FromUtf8Error {}", error),
        }
    }
}

pub fn encoded_len(input: &dyn AsRef<str>) -> usize {
    let mut l: usize = 0;
    for ch in input.as_ref().bytes() {
        l += BYTE_TO_EMOJI[ch as usize].len();
    }
    l
}

pub fn encode_string(input: &dyn AsRef<str>) -> String {
    let mut res = String::with_capacity(encoded_len(input));
    for v in input.as_ref().bytes() {
        res.push_str(BYTE_TO_EMOJI[v as usize])
    }
    res
}

pub fn decode_string(input: &dyn AsRef<str>) -> Result<String, TranslationError> {
    let mut iter = input.as_ref().bytes();
    let mut result: Vec<u8> = Vec::with_capacity(iter.clone().count());

    'm: loop {
        let mut sum: u8 = 0;
        'b: loop {
            let ch = iter.next();
            if ch.is_none() {
                break 'm;
            }

            let b = ch.unwrap();
            match b {
                240 => {
                    iter.next(); // 159
                    let b2 = iter.next().unwrap();
                    match b2 {
                        171 => {
                            // people
                            iter.next();
                            sum += 200;
                        }
                        146 => {
                            // heart
                            iter.next();
                            sum += 50;
                        }
                        165 => {
                            // bottom
                            iter.next();
                            sum += 5;
                        }
                        145 => {
                            // seperator
                            iter.nth(4);
                            break 'b;
                        }
                        _ => {
                            return Err(TranslationError {
                                why: format!("Attempt to decode byte {}", b2),
                            });
                        }
                    }
                }
                226 => {
                    let b2 = iter.next().unwrap();
                    match b2 {
                        156 => {
                            // star
                            iter.next();
                            sum += 10;
                        }
                        157 => {
                            // null heart
                            iter.nth(3);
                        }
                        _ => {
                            return Err(TranslationError {
                                why: format!("Attempt to decode byte {}", b2),
                            });
                        }
                    }
                }
                44 => {
                    // comma
                    sum += 1;
                }
                _ => {
                    return Err(TranslationError {
                        why: format!("Attempt to decode byte {}", b),
                    });
                }
            }
        }
        result.push(sum)
    }

    Ok(String::from_utf8(result)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_decode() {
        // ~~Test that we haven't killed backwards-compat~~
        // fuck backwards-compat
        assert_eq!(
            decode_string(&"💖✨✨✨,,,,👉👈💖💖,👉👈💖💖✨🥺👉👈💖💖✨🥺,👉👈").unwrap(),
            "Test"
        );
    }

    #[test]
    fn test_unicode_string_decode() {
        assert_eq!(
            decode_string(&"🫂✨✨✨✨👉👈💖💖💖🥺,,,,👉👈💖💖💖✨🥺👉👈💖💖💖✨✨✨🥺,👉👈")
                .unwrap(),
            "🥺",
        );
        assert_eq!(
            decode_string(
                &"🫂✨✨🥺,,👉👈💖💖✨✨🥺,,,,👉👈💖💖✨✨✨✨👉👈🫂✨✨🥺,,👉👈\
            💖💖✨✨✨👉👈💖💖✨✨✨✨🥺,,👉👈🫂✨✨🥺,,👉👈💖💖✨✨🥺,,,,👉👈\
            💖💖💖✨✨🥺,👉👈🫂✨✨🥺,,👉👈💖💖✨✨✨👉👈💖💖✨✨✨✨👉👈"
            )
            .unwrap(),
            "がんばれ",
        );
    }
}
