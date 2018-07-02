use language;
use language::FindResult;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;
use std::vec::Vec;
use utils::path::filename;

pub struct Java {
    pub maybe_file: Result<File, Error>,
    pub file_name: String,
}

pub fn source(p: &Path) -> Java {
    Java {
        maybe_file: File::open(p),
        file_name: filename(p).unwrap(),
    }
}

impl language::Language for Java {
    #[inline]
    fn find(&self) -> Result<language::FindResult, Error> {
        let mut counter = 1; // Lines begin on index 1
        let mut comments = Vec::<u32>::new();

        match self.maybe_file {
            Ok(ref file) => {
                for line in BufReader::new(file).lines() {
                    match line {
                        Ok(l) => {
                            if is_comment(l) {
                                comments.push(counter);
                            }
                        }
                        Err(_) => panic!("Could not read line"),
                    }
                    counter = counter + 1;
                }

                Ok(FindResult {
                    file_name: self.file_name.to_owned(),
                    lines: comments,
                })
            }
            Err(_) => Err(Error::new(ErrorKind::InvalidInput, "Could not parse file")),
        }
    }
}

fn is_comment(line: String) -> bool {
    // TODO: Make code prettier
    let first_two_chars = &line.trim().get(0..2);
    let first_three_chars = &line.trim().get(0..3);
    let three_char_result = match first_three_chars {
        &None => false,
        &Some(chars) => match chars {
            "/**" => true,
            _ => false,
        },
    };
    if three_char_result == true {
        return true;
    }
    match first_two_chars {
        &None => false,
        &Some(chars) => match chars {
            "//" => true,
            "/*" => true,
            _ => {
                return match &chars.get(0..1) {
                    None => false,
                    Some(first_char) => {
                        return match first_char {
                            &"*" => true,
                            _ => false,
                        };
                    }
                };
            }
        },
    }
}