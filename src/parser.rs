use crate::ast::*;
use winnow::ascii::{alpha1, alphanumeric0};
use winnow::combinator::{opt, separated};
use winnow::prelude::*;
use winnow::token::take;
use winnow::{
    ascii::{float, multispace0 as multispaces},
    combinator::{alt, delimited, fold_repeat},
    token::one_of,
};
// pub struct Environment(HashMap<String, f32>);

pub fn expr(i: &mut &str) -> PResult<Expr> {
    delimited(
        multispaces,
        alt((assign_scalar, sum, product, function)),
        multispaces,
    )
    .parse_next(i)
}

fn sum(i: &mut &str) -> PResult<Expr> {
    use BinOp::*;
    let init = product.parse_next(i)?;

    fold_repeat(
        0..,
        (one_of(['+', '-']), product),
        move || init.clone(),
        |acc, (op, val): (char, Expr)| {
            if op == '+' {
                Expr::BinaryOp(Add(Box::new(acc), Box::new(val)))
            } else {
                Expr::BinaryOp(Sub(Box::new(acc), Box::new(val)))
            }
        },
    )
    .parse_next(i)
}

// fn unop(i: &mut &str) -> PResult<Expr> {
//     '-'.map(|_| Expr::UnaryOp).parse_next(i)
// }

fn product(i: &mut &str) -> PResult<Expr> {
    use BinOp::*;
    let init = scalar.parse_next(i)?;

    fold_repeat(
        0..,
        (one_of(['*', '/']), scalar),
        move || init.clone(),
        |acc, (op, val): (char, Expr)| {
            if op == '*' {
                Expr::BinaryOp(Mul(Box::new(acc), Box::new(val)))
            } else {
                Expr::BinaryOp(Div(Box::new(acc), Box::new(val)))
            }
        },
    )
    .parse_next(i)
}

fn scalar(i: &mut &str) -> PResult<Expr> {
    delimited(
        multispaces,
        alt((float.map(Expr::Scalar), function, parens)),
        multispaces,
    )
    .parse_next(i)
}

fn identifier(i: &mut &str) -> PResult<String> {
    let c1 = take(1u8).and_then(alpha1).parse_next(i)?;
    let c2 = opt(take(1u8).and_then(alphanumeric0)).parse_next(i)?;
    match c2 {
        Some(c) => Ok(format!("{}{}", c1, c)),
        None => Ok(c1.to_string()),
    }
}

fn assign_scalar(i: &mut &str) -> PResult<Expr> {
    let lhs = identifier.parse_next(i)?;
    delimited(multispaces, "=", multispaces).parse_next(i)?;
    let rhs = expr.parse_next(i)?;
    Ok(Expr::Assign {
        lhs,
        rhs: Box::new(rhs),
    })
}

// Math LN10 LN2 LOG10E LOG2E SQRT1_2 SQRT2 abs acos acosh asin asinh atan atan2 atanh cbrt ceil
// clz32 cos cosh exp expm1 floor fround hypot imul log log10 log1p log2 max min pow round sign
// sin sinh sqrt tan tanh trunc mix mod fract rmax rmin bx2 bx3 don sabs scl qcl rot Infinity map reduce

fn function_name(i: &mut &str) -> PResult<FunctionName> {
    use FunctionName::*;
    alt((
        alt((
            "sin".map(|_| Sin),
            "cos".map(|_| Cos),
            "tan".map(|_| Tan),
            "atan2".map(|_| Atan2),
            "exp".map(|_| Exp),
            "exp2".map(|_| Exp2),
            "log".map(|_| Log),
            "log2".map(|_| Log2),
            "pow".map(|_| Pow),
            "sqrt".map(|_| Sqrt),
            "abs".map(|_| Abs),
            "B".map(|_| Abs),
            "sign".map(|_| Sign),
            "floor".map(|_| Floor),
            "ceil".map(|_| Ceil),
            "fract".map(|_| Fract),
        )),
        alt((
            "mod".map(|_| Mod),
            "min".map(|_| Min),
            "max".map(|_| Max),
            "clamp".map(|_| Clamp),
            "mix".map(|_| Mix),
            "SM".map(|_| Smoothstep),
            "L".map(|_| Length),
            "distance".map(|_| Distance),
            "D".map(|_| Dot),
            "X".map(|_| Cross),
            "N".map(|_| Normalize),
        )),
    ))
    .parse_next(i)
}

fn parens(i: &mut &str) -> PResult<Expr> {
    delimited("(", expr, ")")
        .map(|e| Expr::Paren(Box::new(e)))
        .parse_next(i)
}

fn args(i: &mut &str) -> PResult<Vec<Expr>> {
    separated(0.., expr, ",").parse_next(i)
}

fn function(i: &mut &str) -> PResult<Expr> {
    let name = function_name.parse_next(i)?;
    delimited(multispaces, "(", multispaces).parse_next(i)?;
    let args = args.parse_next(i)?;
    delimited(multispaces, ")", multispaces).parse_next(i)?;
    Ok(Expr::Function { name, args })
}

mod tests {
    use super::*;

    #[test]
    fn expr_test() {
        let input = " 1.1 +  2.12 ";
        let expected = Ok(("", String::from("BinaryOp(Add(Scalar(1.1), Scalar(2.12)))")));
        assert_eq!(expr.map(|e| format!("{e:?}")).parse_peek(input), expected);

        let input = " 12.4321 + 6.321 - 4.21+  3.";
        let expected = Ok((
        "",
        String::from("BinaryOp(Add(BinaryOp(Sub(BinaryOp(Add(Scalar(12.4321), Scalar(6.321))), Scalar(4.21))), Scalar(3.0)))"),
    ));
        assert_eq!(expr.map(|e| format!("{e:?}")).parse_peek(input), expected);

        let input = " 1.3 + 2.3*3.3 + 4.31";
        let expected = Ok((
        "",
        String::from("BinaryOp(Add(BinaryOp(Add(Scalar(1.3), BinaryOp(Mul(Scalar(2.3), Scalar(3.3))))), Scalar(4.31)))"),
    ));
        assert_eq!(expr.map(|e| format!("{e:?}")).parse_peek(input), expected);

        let input = "sin(3.14) / 2.0";
        let expected = Ok((
            "",
            String::from(
                "BinaryOp(Div(Function { name: Sin, args: [Scalar(3.14)] }, Scalar(2.0)))",
            ),
        ));
        assert_eq!(expr.map(|e| format!("{e:?}")).parse_peek(input), expected);

        let input = "aa = sin(1.0) / 2.0";
        let expected = Ok((
            "",
            String::from("Assign { lhs: \"aa\", rhs: BinaryOp(Div(Function { name: Sin, args: [Scalar(1.0)] }, Scalar(2.0))) }"),
        ));
        assert_eq!(expr.map(|e| format!("{e:?}")).parse_peek(input), expected);

        let input = "x3 = (1.0 + 2.0) * 3.0";
        let expected = Ok((
            "",
            String::from("Assign { lhs: \"x3\", rhs: BinaryOp(Mul(Paren(BinaryOp(Add(Scalar(1.0), Scalar(2.0)))), Scalar(3.0))) }"),
        ));
        assert_eq!(expr.map(|e| format!("{e:?}")).parse_peek(input), expected);
    }

    #[test]
    fn scalar_test() {
        let input = "3.0";
        let expected = Ok(("", String::from("Scalar(3.0)")));
        assert_eq!(scalar.map(|e| format!("{e:?}")).parse_peek(input), expected);

        let input = " 12.1";
        let expected = Ok(("", String::from("Scalar(12.1)")));
        assert_eq!(scalar.map(|e| format!("{e:?}")).parse_peek(input), expected);

        let input = "537.23 ";
        let expected = Ok(("", String::from("Scalar(537.23)")));
        assert_eq!(scalar.map(|e| format!("{e:?}")).parse_peek(input), expected);

        let input = "  24.456     ";
        let expected = Ok(("", String::from("Scalar(24.456)")));
        assert_eq!(scalar.map(|e| format!("{e:?}")).parse_peek(input), expected);
    }

    #[test]
    fn product_test() {
        let input = " 12.7 *2.7 /  3.7";
        let expected = Ok((
            "",
            String::from("BinaryOp(Div(BinaryOp(Mul(Scalar(12.7), Scalar(2.7))), Scalar(3.7)))"),
        ));
        assert_eq!(
            product.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );

        let input = " 2.3* 3.3  *2.3 *2.3 /  3.3";
        let expected = Ok((
            "",
            String::from(
                "BinaryOp(Div(BinaryOp(Mul(BinaryOp(Mul(BinaryOp(Mul(Scalar(2.3), Scalar(3.3))), Scalar(2.3))), Scalar(2.3))), Scalar(3.3)))",
            ),
        ));
        assert_eq!(
            product.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );

        let input = " 48.4 /  3.4/2.5";
        let expected = Ok((
            "",
            String::from("BinaryOp(Div(BinaryOp(Div(Scalar(48.4), Scalar(3.4))), Scalar(2.5)))"),
        ));
        assert_eq!(
            product.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );
    }

    #[test]
    fn parens_test() {
        let input = " (  2.0 )";
        let expected = Ok(("", String::from("Paren(Scalar(2.0))")));
        assert_eq!(expr.map(|e| format!("{e:?}")).parse_peek(input), expected);

        let input = " 2.1 * (  3.23 + 4.456 ) ";
        let expected = Ok((
            "",
            String::from(
                "BinaryOp(Mul(Scalar(2.1), Paren(BinaryOp(Add(Scalar(3.23), Scalar(4.456))))))",
            ),
        ));
        assert_eq!(expr.map(|e| format!("{e:?}")).parse_peek(input), expected);

        let input = "  2.5 * 2.5  / ( 5.0 - 1.0) + 3.25";
        let expected = Ok((
            "",
            String::from(
                "BinaryOp(Add(BinaryOp(Div(BinaryOp(Mul(Scalar(2.5), Scalar(2.5))), Paren(BinaryOp(Sub(Scalar(5.0), Scalar(1.0)))))), Scalar(3.25)))",
            )),
        );
        assert_eq!(expr.map(|e| format!("{e:?}")).parse_peek(input), expected);
    }

    #[test]
    fn function_test() {
        let input = "sin(1.0)";
        let expected = Ok((
            "",
            String::from("Function { name: Sin, args: [Scalar(1.0)] }"),
        ));
        assert_eq!(
            function.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );

        let input = "clamp(-2.0 * 5.1, 0.0, 10.0)";
        let expected = Ok((
            "",
            String::from("Function { name: Clamp, args: [BinaryOp(Mul(Scalar(-2.0), Scalar(5.1))), Scalar(0.0), Scalar(10.0)] }"),
        ));
        assert_eq!(
            function.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );

        let input = "cos( sin(3.14* 2.0) )";
        let expected = Ok((
            "",
            String::from("Function { name: Cos, args: [Function { name: Sin, args: [BinaryOp(Mul(Scalar(3.14), Scalar(2.0)))] }] }"),
        ));
        assert_eq!(
            function.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );
    }

    #[test]
    fn identifier_test() {
        let input = "a";
        let expected = Ok(("", String::from("a")));
        assert_eq!(identifier.parse_peek(input), expected);

        let input = "a2c";
        let expected = Ok(("c", String::from("a2")));
        assert_eq!(identifier.parse_peek(input), expected);
    }

    #[test]
    fn assign_scalar_test() {
        let input = "ab = 1.57";
        let expected = Ok((
            "",
            String::from("Assign { lhs: \"ab\", rhs: Scalar(1.57) }"),
        ));
        assert_eq!(
            assign_scalar.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );

        let input = "a = sin(1.0) / 2.0";
        let expected = Ok((
            "",
            String::from("Assign { lhs: \"a\", rhs: BinaryOp(Div(Function { name: Sin, args: [Scalar(1.0)] }, Scalar(2.0))) }"),
        ));
        assert_eq!(
            assign_scalar.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );
    }
}
