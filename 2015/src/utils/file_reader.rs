use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::process;
use utf8_read::{Char, Reader};

pub struct FileReader(Reader<File>);

impl FileReader {
    fn open_file(file_path: &str) -> File {
        match File::open(file_path) {
            Ok(f) => f,
            Err(_) => {
                eprintln!("Could not open file {}, aborting", file_path);
                process::exit(1);
            }
        }
    }

    pub fn new(file_path: &str) -> FileReader {
        let file = Self::open_file(file_path);

        FileReader(Reader::new(file))
    }

    pub fn read_next(&mut self) -> Option<char> {
        if let Char::Char(ch) = self
            .0
            .next_char()
            .expect("Could not read a character from the file")
        {
            return Some(ch);
        }

        None
    }

    pub fn process_file<F>(file_path: &str, cb: &mut F)
    where
        F: FnMut(char),
    {
        let mut reader = FileReader::new(file_path);
        reader.process_all(cb);
    }

    pub fn process_all<F>(&mut self, cb: &mut F)
    where
        F: FnMut(char),
    {
        while let Some(ch) = self.read_next() {
            cb(ch);
        }
    }

    pub fn process_until<F>(&mut self, cb: &mut F)
    where
        F: FnMut(char) -> bool,
    {
        while let Some(ch) = self.read_next() {
            if cb(ch) {
                break;
            }
        }
    }

    pub fn process_lines<F>(file_path: &str, cb: &mut F)
    where
        F: FnMut(&str),
    {
        let file = Self::open_file(file_path);
        let reader = BufReader::new(file);

        for line in reader.lines() {
            cb(&line.expect("Could not read a line"));
        }
    }
}
