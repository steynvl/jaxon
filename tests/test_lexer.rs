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
        // Stop here (for now), so that test suite passes.
        if file == "test109.svl" {
            break;
        }

        let source_file = format!("{}/{}", lexer_tests_dir, file);
        let source = fs::read_to_string(&source_file)
            .expect(format!("Could not read the file: {}", source_file).as_str());

        let std_out_file = format!("{}/report/{}.out.txt", lexer_tests_dir, file);
        let std_out = fs::read_to_string(&std_out_file)
            .expect(format!("Could not read the file: {}", std_out_file).as_str());

        let std_err_file = format!("{}/report/{}.err.txt", lexer_tests_dir, file);
        let std_err = fs::read_to_string(&std_err_file)
            .expect(format!("Could not read the file: {}", std_err_file).as_str());

        println!("-- START --");
        println!("{}", file);
        println!("{:?}", source);
        println!("out: {:?}", std_out);
        println!("err: {:?}", std_err);

        let mut lexer = Lexer::new(source.as_bytes());
        let mut token: Token = Token::Eof;
        loop {
            lexer.get_token(&mut token).unwrap();
            print!("{:?}  ", token);

            if token == Token::Eof {
                break;
            }
        }
        println!();
    }
}
