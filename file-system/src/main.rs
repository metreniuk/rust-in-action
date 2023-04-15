use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq)]

enum FileState {
    Open,
    Closed,
}

trait Read {
    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Closed => write!(f, "CLOSED"),
            FileState::Open => write!(f, "OPEN"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl File {
    fn new(name: &str) -> Self {
        File {
            name: String::from(name),
            data: vec![],
            state: FileState::Closed,
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> Self {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }
}

impl Read for File {
    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        let mut tmp = self.data.clone();
        let read_len = tmp.len();

        save_to.reserve(read_len);
        save_to.append(&mut tmp);
        Ok(read_len)
    }
}

fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;
    Ok(f)
}

fn main() {
    let f1_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let f1 = File::new_with_data("f1.txt", &f1_data);
    let mut buffer: Vec<u8> = vec![];
    let f1 = open(f1).unwrap();
    let f1_length = f1.read(&mut buffer).unwrap();
    let f1 = close(f1).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f1);
    println!("{} is {} bytes long", f1.name, f1_length);
    println!("{}", text);
}
