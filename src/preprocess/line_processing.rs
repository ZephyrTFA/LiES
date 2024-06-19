use crate::tokenize::token::Token;

#[test]
fn test() {
    let lines = [
        "/proc/main()",
        "\tworld.log << \"hello\";\tvar/xyz = 123",
        "\tvar/strr = {\"",
        "asdasdasd",
        "\"}",
        "\t\t\\",
        "asd",
    ];
    let mut expected = vec![
        vec!["/", "proc", "/", "main", "(", ")"],
        vec![
            "\t", "world", ".", "log", " ", "<<", " ", "\"", "hello", "\"",
        ],
        vec!["\t", "var", "/", "xyz", " ", "=", " ", "123"],
        vec!["\t", "var", "/", "strr", " ", "=", " ", "{", "\""],
        vec!["asdasdasd"],
        vec!["\"", "}"],
        vec!["\t\t", "asd"],
    ];

    let tokens = crate::tokenize::lib::tokenize_lines(
        lines.iter().cloned(),
        std::path::Path::new("test.test"),
    )
    .expect("failed to parse");
    let processed = process_lines(&tokens);

    assert_eq!(
        expected.len(),
        processed.len(),
        "final and output lens did not match"
    );
    for line in processed {
        let values: Vec<&str> = line.iter().map(|tok| tok.value().as_str()).collect();
        assert_eq!(values, expected.remove(0));
    }
}

fn is_line_escaped(final_token: &Token) -> bool {
    let value = final_token.value();
    if !value.ends_with('\\') {
        return false;
    }

    return value.chars().rev().take_while(|c| *c == '\\').count() % 2 == 1;
}

pub fn process_lines<'a>(line_tokens: impl Into<Vec<Vec<&'a Token>>>) -> Vec<Vec<&'a Token>> {
    let line_tokens = line_tokens.into();
    let mut final_lines = Vec::with_capacity(line_tokens.len());
    let mut current_line = vec![];

    macro_rules! next_line {
        () => {
            final_lines.push(std::mem::take(&mut current_line));
        };
    }

    for line in line_tokens {
        for token in line {
            if token.value() == ";" {
                next_line!();
                continue;
            }
            current_line.push(token);
        }
        if !current_line
            .last()
            .is_some_and(|token| is_line_escaped(token))
        {
            next_line!();
        } else {
            current_line.pop().unwrap();
        }
    }
    final_lines
}
