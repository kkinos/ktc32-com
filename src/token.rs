fn strtol(s: String) -> (String, Option<i64>) {
    let mut num = 0;
    let chars = s.chars();
    let mut has_digits = false;
    let mut index = s.len();

    for (i, c) in chars.enumerate() {
        if c.is_ascii_digit() {
            has_digits = true;
            num = num * 10 + (c as i64 - '0' as i64);
        } else {
            index = i;
            break;
        }
    }
    if !has_digits {
        return (s, None);
    }
    let (_, remainder) = s.split_at(index);
    (remainder.to_string(), Some(num))
}

fn extract_name(s: String) -> (String, String) {
    let mut index = 0;
    for (i, c) in s.chars().enumerate() {
        if c.is_ascii_alphabetic() {
            index = i + 1;
        } else {
            break;
        }
    }
    let name = s[..index].to_string();
    let remainder = s[index..].to_string();
    (remainder, name)
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Reserved,
    Ident,
    Num,
    Eof,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub val: i64,
    pub str: String,
}

pub fn tokenize(mut p: String) -> Vec<Token> {
    let mut token_list: Vec<Token> = Vec::new();
    while let Some(c) = p.chars().next() {
        if c.is_whitespace() {
            p = p.split_off(1);
            continue;
        }

        if c == '+' || c == '-' || c == '*' || c == '/' || c == '(' || c == ')' || c == ';' {
            p = p.split_off(1);
            token_list.push(Token {
                kind: TokenKind::Reserved,
                val: 0,
                str: c.to_string(),
            });
            continue;
        }

        if c == '=' || c == '!' {
            p = p.split_off(1);
            let h = p.chars().next().unwrap();
            if h == '=' {
                token_list.push(Token {
                    kind: TokenKind::Reserved,
                    val: 0,
                    str: c.to_string() + &h.to_string(),
                });
                p = p.split_off(1);
                continue;
            } else {
                token_list.push(Token {
                    kind: TokenKind::Reserved,
                    val: 0,
                    str: c.to_string(),
                });
                continue;
            }
        }

        if c == '>' || c == '<' {
            p = p.split_off(1);
            let h = p.chars().peekable().peek().cloned().unwrap();
            if h == '=' {
                token_list.push(Token {
                    kind: TokenKind::Reserved,
                    val: 0,
                    str: c.to_string() + &h.to_string(),
                });
                p = p.split_off(1);
                continue;
            } else {
                token_list.push(Token {
                    kind: TokenKind::Reserved,
                    val: 0,
                    str: c.to_string(),
                });
                continue;
            }
        }

        if c.is_ascii_digit() {
            let (r, num) = strtol(p);
            p = r;
            token_list.push(Token {
                kind: TokenKind::Num,
                val: num.unwrap(),
                str: c.to_string(),
            });
            continue;
        }

        if c.is_ascii_alphabetic() {
            let (r, name) = extract_name(p);
            p = r;
            match name.as_str() {
                "return" | "if" | "else" | "while" | "for" => token_list.push(Token {
                    kind: TokenKind::Reserved,
                    val: 0,
                    str: name,
                }),

                _ => {
                    token_list.push(Token {
                        kind: TokenKind::Ident,
                        val: 0,
                        str: name,
                    });
                }
            }
            continue;
        }
        panic!("could not tokenize {}", c);
    }

    token_list.push(Token {
        kind: TokenKind::Eof,
        val: 0,
        str: "".to_string(),
    });
    token_list
}
