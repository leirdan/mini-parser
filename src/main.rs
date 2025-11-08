#[derive(Clone)]
enum Expression {
    Number(i64),
    Unary {
        operand: Box<Expression>,
        operation: char,
    },
    Binary {
        left: Box<Expression>,
        right: Box<Expression>,
        operation: char,
    },
}

impl Expression {
    fn evaluate(&self) -> Option<i64> {
        match self {
            Expression::Number(num) => Some(*num),
            Expression::Unary { operand, operation } => {
                if let Some(num) = operand.evaluate() {
                    match operation {
                        '-' => Some(-num),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            Expression::Binary {
                left,
                right,
                operation,
            } => {
                let l = left.evaluate();
                let r = right.evaluate();

                if let Some(v_l) = l {
                    if let Some(v_r) = r {
                        match *operation {
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
                            _ => panic!("Invalid operation!"),
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

    fn tree(&self) {
        self.tree_rec(0, true);
    }

    fn tree_rec<'a>(&self, level: usize, last: bool) {
        if level > 0 {
            for _ in 1..level {
                print!("| ");
            }

            if last {
                print!("└─ ");
            } else {
                print!("├─ ");
            }
        }

        match self {
            Expression::Number(num) => {
                println!("{}", num);
            }
            Expression::Unary { operand, operation } => {
                println!("{}", *operation);
                operand.tree_rec(level + 1, true);
            }
            Expression::Binary {
                left,
                right,
                operation,
            } => {
                println!("{}", *operation);
                left.tree_rec(level + 1, false);
                right.tree_rec(level + 1, true);
            }
        }
    }
}

fn main() {
    let exp = Expression::Binary {
        left: Box::new(Expression::Binary {
            left: Box::new(Expression::Binary {
                left: Box::new(Expression::Binary {
                    left: Box::new(Expression::Unary {
                        operand: Box::new(Expression::Binary {
                            left: Box::new(Expression::Number(10)),
                            right: Box::new(Expression::Number(20)),
                            operation: '+',
                        }),
                        operation: '-',
                    }),
                    right: Box::new(Expression::Number(30)),
                    operation: '+',
                }),
                right: Box::new(Expression::Number(40)),
                operation: '+',
            }),
            right: Box::new(Expression::Binary {
                left: Box::new(Expression::Number(50)),
                right: Box::new(Expression::Number(60)),
                operation: '+',
            }),
            operation: '+',
        }),
        right: Box::new(Expression::Unary {
            operand: Box::new(Expression::Number(5)),
            operation: '-',
        }),
        operation: '*',
    };
    exp.tree();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(exp: &Expression, output: Option<i64>) {
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
        let exp = Expression::Binary {
            left: Box::new(Expression::Number(10)),
            right: Box::new(Expression::Number(20)),
            operation: '+',
        };
        test(&exp, Some(30));
    }

    #[test]
    fn exp2() {
        let exp = Expression::Binary {
            left: Box::new(Expression::Number(10)),
            right: Box::new(Expression::Number(0)),
            operation: '/',
        };
        test(&exp, None);
    }

    #[test]
    fn exp3() {
        let exp = Expression::Binary {
            left: Box::new(Expression::Binary {
                left: Box::new(Expression::Number(10)),
                right: Box::new(Expression::Number(20)),
                operation: '+',
            }),
            right: Box::new(Expression::Number(30)),
            operation: '*',
        };
        test(&exp, Some(900));
    }

    #[test]
    fn exp4() {
        let exp = Expression::Binary {
            left: Box::new(Expression::Number(10)),
            right: Box::new(Expression::Binary {
                left: Box::new(Expression::Number(20)),
                right: Box::new(Expression::Number(30)),
                operation: '*',
            }),
            operation: '+',
        };
        test(&exp, Some(610));
    }

    #[test]
    fn exp5() {
        let exp = Expression::Binary {
            left: Box::new(Expression::Unary {
                operand: Box::new(Expression::Number(10)),
                operation: '-',
            }),
            right: Box::new(Expression::Number(2)),
            operation: '/',
        };

        test(&exp, Some(-5));
    }

    #[test]
    fn exp6() {
        let exp = Expression::Binary {
            left: Box::new(Expression::Binary {
                left: Box::new(Expression::Binary {
                    left: Box::new(Expression::Binary {
                        left: Box::new(Expression::Unary {
                            operand: Box::new(Expression::Binary {
                                left: Box::new(Expression::Number(10)),
                                right: Box::new(Expression::Number(20)),
                                operation: '+',
                            }),
                            operation: '-',
                        }),
                        right: Box::new(Expression::Number(30)),
                        operation: '+',
                    }),
                    right: Box::new(Expression::Number(40)),
                    operation: '+',
                }),
                right: Box::new(Expression::Binary {
                    left: Box::new(Expression::Number(50)),
                    right: Box::new(Expression::Number(60)),
                    operation: '+',
                }),
                operation: '+',
            }),
            right: Box::new(Expression::Unary {
                operand: Box::new(Expression::Number(5)),
                operation: '-',
            }),
            operation: '*',
        };
        test(&exp, Some(-750));
    }
}
