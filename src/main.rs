use std::env;
use std::fs::File;
use std::io;
use std::path::Path;

const INVALID_CHARS: &[char] = &[
    '/', '\\', ':', '*', '+', ',', ';', '<', '>', '?', '=', '|', '"', '\'', '[', ']', '{', '}',
];

fn is_valid_filename(filename: &str) -> bool {
    !filename.chars().any(|c| INVALID_CHARS.contains(&c))
}

fn file_exists(filename: &str) -> bool {
    Path::new(filename).exists()
}

fn create_empty_file(filename: &str) -> io::Result<()> {
    if !file_exists(filename) {
        File::create(filename)?;
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    for arg in &args {
        if is_valid_filename(arg) {
            if let Err(e) = create_empty_file(arg) {
                eprintln!("Erro ao criar arquivo {}: {}", arg, e);
            }
        }
    }
}
