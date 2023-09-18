use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::PathBuf,
    // str::FromStr,
};

#[derive(Debug, Default)]
pub struct Buffer {
    pub name: String,
    pub buf: String,
    // pub vec_str: Vec<String>,
}

impl Buffer {
    pub fn new(bufname: &PathBuf) -> Self {
        Buffer {
            name: bufname.display().to_string(),
            buf: String::new(),
            // vec_str: Vec::<String>::new(),
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
                    // self.vec_str.push(String::from_str("\n").unwrap());
                    self.buf.push('\n');
                }
            }
            Err(e) => {
                eprintln!("Error occured while opening and reading file with error code: {e}")
            }
        }
        buf = buf.replace("\r\n", "\n");
        // for s in buf.lines() {
        //     let mut ss = s.to_string();
        //     ss.push('\n');
        //     self.vec_str.push(ss);
        // }
        self.buf = buf;
    }
}
