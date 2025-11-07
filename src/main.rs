use std::collections::VecDeque;

fn next(input: &str) -> Result<(usize, &str, &str), Option<usize>> {
    let mut start_idx = 0;
    for (i, c) in input.char_indices() {
        if c.is_whitespace() {
            start_idx = i + c.len_utf8();
        } else {
            break;
        }
    }

    let slice = &input[start_idx..];
    let mut chars = slice.char_indices();

    let (first_i, first_c) = match chars.next() {
        None => return Err(None),
        Some(c) => c,
    };

    if first_c.is_ascii_digit() {
        let end = slice
            .char_indices()
            .skip_while(|&(_, c)| c.is_ascii_digit())
            .next()
            .map(|(i, _)| i)
            .unwrap_or(slice.len());

        Ok((start_idx + first_i, &slice[first_i..end], &slice[end..]))
    } else if ['+', '-', '*', '/', '%', '(', ')'].contains(&first_c) {
        let end = first_i + first_c.len_utf8();
        Ok((start_idx + first_i, &slice[first_i..end], &slice[end..]))
    } else {
        Err(Some(start_idx + first_i))
    }
}

#[derive(Clone)]
enum Expression {
    Number(i64),
    Symbol(char),
    Exp {
        left: Box<Expression>,
        right: Box<Expression>,
        operation: Box<Expression>,
    },
}

impl Expression {
    fn evaluate(&self) -> Option<i64> {
        match self {
            Expression::Number(num) => Some(*num),
            Expression::Symbol(sym) => None,
            Expression::Exp {
                left,
                right,
                operation,
            } => {
                let l = left.evaluate();
                let r = right.evaluate();

                if let Some(v_l) = l {
                    if let Some(v_r) = r {
                        if let Expression::Symbol(sym) = **operation {
                            match sym {
                                '+' => Some(v_l + v_r),
                                '-' => Some(v_l - v_r),
                                '*' => Some(v_l * v_r),
                                '/' => {
                                    if v_r == 0 {
                                        println!("Div by 0 is not allowed");
                                        None
                                    } else {
                                        Some(v_l / v_r)
                                    }
                                }
                                '%' => Some(v_l % v_r),
                                _ => None,
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
    }

    fn print(&self) {}

    fn convert_input_to_exp<'a>(mut input: &str) -> Box<Option<Expression>> {
        let create_exp = |v: &mut VecDeque<Expression>| {
            let mut idx: Option<usize> = None;
            for (i, ex) in v.iter().enumerate() {
                if let Expression::Symbol(sym) = ex {
                    match sym {
                        '*' | '%' | '/' => {
                            idx = Some(i);
                            break;
                        }
                        '+' | '-' => {
                            idx = Some(i);
                        }
                        _ => {}
                    }
                }
            }

            let Some(mut i) = idx else { todo!() };

            let l = Box::new(v.remove(i - 1).unwrap());
            let op = Box::new(v.remove(i - 1).unwrap());
            i = if i > v.len() { v.len() } else { i };
            let r = Box::new(v.remove(i - 1).unwrap());

            Expression::Exp {
                left: l,
                right: r,
                operation: op,
            }
        };

        let mut tokens = Box::new(Vec::new());
        while !input.is_empty() {
            let res = next(&input);
            match res {
                Ok((_, s, st)) => {
                    tokens.push(s);
                    input = st;
                }
                Err(None) => {}
                Err(_) => panic!("Invalid expression was found!!"),
            }
        }

        let mut symbols: VecDeque<Expression> = VecDeque::new();
        let mut expressions: Vec<Expression> = Vec::new();

        for &token in tokens.iter() {
            match token {
                "+" | "-" | "%" | "/" | "*" => {
                    symbols.push_back(Expression::Symbol(token.parse().unwrap()))
                }
                ")" => {
                    let exp = create_exp(&mut symbols);
                    symbols.push_front(exp.clone());
                    expressions.push(exp);
                }
                num => match num.parse::<i64>() {
                    Ok(n) => {
                        symbols.push_back(Expression::Number(n));
                    }
                    Err(_) => {}
                },
            }
        }

        while symbols.len() != 1 {
            let exp = create_exp(&mut symbols);
            symbols.push_front(exp.clone());
            expressions.push(exp);
        }

        Box::new(expressions.pop())
    }

    fn tree(&self) {
        self.tree_rec("", true, true);
    }

    fn tree_rec<'a>(&self, tab: &str, last: bool, root: bool) {
        match self {
            Expression::Number(num) => {
                println!(
                    "{}{}{}",
                    tab,
                    if root {
                        ""
                    } else if last {
                        "└─ "
                    } else {
                        "├─ "
                    },
                    num
                );
            }
            Expression::Symbol(sym) => {
                println!(
                    "{}{}{}",
                    tab,
                    if root {
                        ""
                    } else if last {
                        "└─ "
                    } else {
                        "├─ "
                    },
                    sym
                );
            }
            Expression::Exp {
                left,
                right,
                operation,
            } => {
                operation.tree_rec(tab, false, root);
                let pre: &str = &format!("{}{}", tab, if last { "   " } else { "│  " });
                left.tree_rec(&pre, false, false);
                right.tree_rec(&pre, true, false);
            }
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(input: &str, output: Option<i64>) {
        let exp = Expression::convert_input_to_exp(&input).unwrap();
        exp.tree();
        let res = exp.evaluate();
        match res {
            None => {
                println!("Result: None")
            }
            Some(n) => {
                println!("Result: {}", n)
            }
        }
        assert_eq!(res, output);
    }

    #[test]
    fn exp1() {
        test("10 + 20", Some(30));
    }

    #[test]
    fn exp2() {
        test("10 / 0", None);
    }

    #[test]
    fn exp3() {
        test("(10 + 20) * 30", Some(900));
    }
    #[test]
    fn exp4() {
        test("10 + 20 * 30", Some(610));
    }
}
