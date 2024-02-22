use std::{fs, io};
use svlang::lexer::Lexer;
use svlang::token::Token;

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
    let lexer_tests_dir = "tests/resources/lexer";
    let lexer_test_files = get_lexer_test_files(lexer_tests_dir).unwrap();
    for file in lexer_test_files {
        let source = fs::read_to_string(format!("{}/{}", lexer_tests_dir, file))
            .expect("Could not read the file.");

        println!("{}", file);
        println!("{:?}", source);

        let mut lexer = Lexer::new(source.as_bytes());
        let mut token: Token = Token::Eof;
        loop {
            lexer.get_token(&mut token);
            print!("{:?}  ", token);

            if token == Token::Eof {
                break;
            }
        }
        println!();

        break;
    }
}
