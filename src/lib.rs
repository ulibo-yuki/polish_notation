use regex::Regex;
use std::fmt;

/// Custom Error type
/// 
/// ## list Explanation
/// 
/// - FailedCalculate,
///     - when the calculation failed because of something.
/// - NotEnoughOperands,
///     - when there are not enough operands.
/// - UseUnavailableCharacter,
///     - when you use unavailable character.
/// - NotEnteredExoression,
///     - when you not entered exoression.
/// 
/// # example
/// 
/// because PolishError implemented fmt::Display, you can print errorcode easy.
/// 
/// ```
/// use polish_notation::PolishError;
/// use polish_notation::pn;
/// 
/// match pn("+ 5 1") {
///     Ok(result) => println!("{}", result),
///     Err(e) => eprintln!("{}", e),
/// };
/// ```
#[derive(Debug, PartialEq)]
pub enum PolishError {
    FailedCalculate,
    NotEnoughOperands,
    UseUnavailableCharacter,
    NotEnteredExoression,
}
impl fmt::Display for PolishError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PolishError::FailedCalculate => write!(f, "failed calculate"),
            PolishError::NotEnoughOperands => write!(f, "not enough operands"),
            PolishError::UseUnavailableCharacter => write!(f, "use unavailable character"),
            PolishError::NotEnteredExoression => write!(f, "not entered exoression"),
        }
    }
}

enum TokenKind {
    Operator(String),
    Operand(f64),
}

fn is_exoression(exoression: &str) -> bool {
    !exoression.is_empty()
}

fn is_unavailable_character(checked_string: &str) -> bool {
    // reは不正値
    let re = Regex::new(r"[^+\-*/%^1234567890 ]").unwrap();
    re.is_match(checked_string)
}

fn syntax_check(exoression: &str) -> Result<(), PolishError> {
    if !is_exoression(exoression) {
        Err(PolishError::NotEnteredExoression)
    } else if is_unavailable_character(exoression) {
        return Err(PolishError::UseUnavailableCharacter);
    } else {
        Ok(())
    }
}

fn parse_token(word: &str) -> Result<TokenKind, PolishError> {
    match word.parse::<f64>() {
        // opnd
        Ok(i) => Ok(TokenKind::Operand(i)),
        // ops
        Err(_) => {
            if !is_unavailable_character(word) {
                Ok(TokenKind::Operator(word.to_string()))
            } else {
                Err(PolishError::UseUnavailableCharacter)
            }
        }
    }
}

fn calculate(a: f64, b: f64, ops: &str) -> Result<f64, PolishError> {
    match ops {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => Ok(a / b),
        "%" => Ok(a % b),
        _ => Err(PolishError::FailedCalculate),
    }
}

/// receive an exoression in Polish notation as `&str`, then `pn` return ans as `f64` or `PolishError`.
/// 
/// ## example
/// 
/// ```
/// use polish_notation::PolishError;
/// use polish_notation::pn;
/// 
/// match pn("+ 5 1") {
///     Ok(result) => println!("{}", result),
///     Err(e) => eprintln!("{}", e),
/// };
/// ```
pub fn pn(expression: &str) -> Result<f64, PolishError> {
    match syntax_check(expression) {
        Ok(_) => {}
        Err(e) => return Err(e),
    }

    let split_expression = expression.split_whitespace();
    let mut operands: Vec<f64> = vec![];

    for token in split_expression.rev() {
        if cfg!(debug_assertions) {
            println!("token(Only displayed when debug): {:?}", token);
        }
        match parse_token(token) {
            Ok(kind) => {
                let result = match kind {
                    // 被演算子の場合
                    TokenKind::Operand(opnd) => opnd,
                    // 演算子の場合
                    TokenKind::Operator(ops) => {
                        if operands.len() < 2 {
                            return Err(PolishError::NotEnoughOperands);
                        }
                        match calculate(
                            operands[operands.len() - 1],
                            operands[operands.len() - 2],
                            &ops,
                        ) {
                            Ok(result) => {
                                operands.drain(operands.len() - 2..operands.len());
                                result
                            }
                            Err(e) => return Err(e),
                        }
                    }
                };
                operands.push(result);
            }
            Err(e) => return Err(e),
        };
    }

    if operands.len() == 1 {
        Ok(operands[operands.len() - 1])
    } else {
        Err(PolishError::FailedCalculate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let exoressions = [
            ("+ 5 2 ", Ok(7.0)),
            ("- 5 2", Ok(3.0)),
            ("* 5 2 ", Ok(10.0)),
            ("/ 5 2", Ok(2.5)),
            ("% 5 2 ", Ok(1.0)),
            ("1", Ok(1.0)),
            ("-1", Ok(-1.0)),
            // 以下エラーテスト
            ("* [ 5 1 = 7  1", Err(PolishError::UseUnavailableCharacter)),
            ("* + 5 1 - 7", Err(PolishError::NotEnoughOperands)),
            ("", Err(PolishError::NotEnteredExoression)),
        ];
        for exoression in exoressions {
            println!("{:?}", exoression);
            assert_eq!(pn(exoression.0), exoression.1);
        }
    }

    #[test]
    fn use_test() {
        match pn("+ 5 1") {
            Ok(result) => println!("{}", result),
            Err(e) => eprintln!("{}", e),
        };
    }
}
