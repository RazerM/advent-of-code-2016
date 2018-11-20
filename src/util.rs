use std::io::{self, Read};

pub(crate) fn stdin_as_string() -> String {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    buffer.trim().to_string()
}

//pub fn stdin_lines() -> Vec<String> {
//    let stdin = io::stdin();
//    let lines = stdin.lock().lines().map(|l| l.unwrap()).collect();
//    lines
//}
