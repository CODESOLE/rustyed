use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::PathBuf,
};

#[derive(Debug, Default)]
pub struct Buffer {
    pub name: String,
    pub buf: String,
}

impl Buffer {
    pub fn new(bufname: &PathBuf) -> Self {
        Buffer {
            name: bufname.display().to_string(),
            buf: String::new(),
        }
    }
    pub fn write_to_file(&self) {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.name)
            .expect("Error occured while opening or creating file!");
        file.write_all(&self.buf.as_bytes())
            .expect("Error occured while writing to file!");
    }
    pub fn read_to_buffer(&mut self, p: &PathBuf) {
        let mut buf: String = String::new();
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(p)
            .expect("Error occured while opening or creating file!");
        match file.read_to_string(&mut buf) {
            Ok(read_bytes) => {
                println!("{read_bytes} Bytes read from file!");
                if read_bytes == 0 {
                    self.buf.push('\n');
                }
            }
            Err(e) => {
                eprintln!("Error occured while opening and reading file with error code: {e}")
            }
        }
        buf = buf.replace("\r\n", "\n");
        buf.push('\n');
        self.buf = buf;
    }
}
