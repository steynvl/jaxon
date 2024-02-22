use std::{fs, io};

fn get_lexer_test_files(path: &str) -> io::Result<Vec<String>> {
    let entries = fs::read_dir(path)?;

    let mut file_names: Vec<String> = entries
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.is_file() {
                path.file_name()?.to_str().map(|s| s.to_owned())
            } else {
                None
            }
        })
        .collect();

    file_names.sort();

    Ok(file_names)
}

#[test]
fn test_lexer() {
    let lexer_test_files = get_lexer_test_files("tests/resources/lexer").unwrap();
    for file in lexer_test_files {
        println!("{}", file)
        // let source = fs::read_to_string(format!("samples/{}", file)).expect("Could not read the file.");
    }
}
