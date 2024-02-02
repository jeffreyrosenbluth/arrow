use winnow::ascii::{alpha1, dec_uint, multispace0};
use winnow::combinator::{alt, delimited, eof, preceded, repeat_till, separated};
use winnow::prelude::*;
use winnow::token::take_till;

#[derive(Debug)]
pub enum Syntax {
    Blob(String),
    ForNumeric { n: u8, block: Box<Syntax> },
    ForAlpha { a: String, block: Box<Syntax> },
    Seq(Vec<Syntax>),
    Empty,
}

// pub fn parse_syntax(i: &mut &str) -> PResult<Syntax> {
//     separated(1.., syntax, alt((",", ";")))
//         .map(|v| Syntax::Seq(v))
//         .parse_next(i)
// }

pub fn parse_macro(i: &mut &str) -> PResult<Syntax> {
    let p = repeat_till(0.., syntax, eof).parse_next(i)?;
    Ok(Syntax::Seq(p.0))
}

fn syntax(i: &mut &str) -> PResult<Syntax> {
    delimited(
        multispace0,
        alt((for_numeric, for_alpha, blob, rbrace.map(|_| Syntax::Empty))),
        multispace0,
    )
    .parse_next(i)
}

fn atsign<'a>(i: &mut &'a str) -> PResult<&'a str> {
    delimited(multispace0, "@", multispace0).parse_next(i)
}

fn lbrace<'a>(i: &mut &'a str) -> PResult<&'a str> {
    delimited(multispace0, "{", multispace0).parse_next(i)
}

fn rbrace<'a>(i: &mut &'a str) -> PResult<&'a str> {
    delimited(multispace0, "}", multispace0).parse_next(i)
}

fn blob(i: &mut &str) -> PResult<Syntax> {
    take_till(0.., |c| c == '@' || c == '}')
        .map(|b: &str| Syntax::Blob(b.to_string()))
        .parse_next(i)
}

fn block(i: &mut &str) -> PResult<Syntax> {
    preceded(lbrace, parse_macro).parse_next(i)
}

fn for_numeric(i: &mut &str) -> PResult<Syntax> {
    let n: u8 = preceded(atsign, dec_uint).parse_next(i)?;
    let block = block.parse_next(i)?;
    Ok(Syntax::ForNumeric {
        n,
        block: Box::new(block),
    })
}

fn for_alpha(i: &mut &str) -> PResult<Syntax> {
    let a = preceded(atsign, alpha1).parse_next(i)?;
    let block = block.parse_next(i)?;
    Ok(Syntax::ForAlpha {
        a: a.to_string(),
        block: Box::new(block),
    })
}

fn bytes_to_string(tape: &[u8]) -> String {
    String::from_utf8(tape.to_vec()).unwrap()
}

pub fn expand_macros(tape: &[u8], mut idx_stack: Vec<usize>) -> Vec<u8> {
    let mut cs: Vec<u8> = Vec::new();
    let mut idx = idx_stack[idx_stack.len() - 1];
    // let mut idx = 0;
    // let mut idx_stack: Vec<usize> = Vec::new();
    while idx < tape.len() {
        let c = tape[idx];
        if c == b'@' {
            idx += 1;
            let mut iters = Vec::new();
            while tape[idx] != b'{' {
                iters.push(tape[idx]);
                idx += 1;
            }
            idx += 1;
            idx_stack.push(idx);
            let iters_num = bytes_to_string(&iters).parse::<u8>();
            match iters_num {
                Ok(n) => {
                    for i in 0..n {
                        idx = idx_stack[idx_stack.len() - 1];
                        while tape[idx] != b'}' {
                            if tape[idx] == b'$' {
                                cs.push(i);
                            } else if tape[idx] == b'@' {
                                idx_stack.push(idx);
                                // let s = &tape[idx..];
                                // dbg!(bytes_to_string(&s));
                                cs.extend(expand_macros(&tape, idx_stack.clone()));
                            } else {
                                cs.push(tape[idx]);
                            }
                            idx += 1;
                        }
                        idx += 1;
                    }
                    // idx_stack.pop();
                }
                Err(_) => {
                    for v in iters {
                        idx = idx_stack[idx_stack.len() - 1];
                        while tape[idx] != b'}' {
                            if tape[idx] == b'$' {
                                cs.push(v);
                            } else if tape[idx] == b'@' {
                                idx_stack.push(idx);
                                // let s = &tape[idx..];
                                cs.extend(expand_macros(&tape, idx_stack.clone()));
                            } else {
                                cs.push(tape[idx]);
                            }
                            idx += 1;
                        }
                        idx += 1;
                    }
                    // idx_stack.pop();
                }
            }
            // let idx = idx_stack.pop();
        } else {
            cs.push(c);
            idx += 1;
        }
    }
    cs
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn blob_test() {
        let input =
            "s=1; @2{ [x,y]=r0(x,y), [x,z]=r1(x,z), @xyz{$=B($*2)-8,} s*=.5,} (L(x,y,z)-8)*s";
        // let expected = Ok((
        //     "@2{ [x,y]=r0(x,y), [x,z]=r1(x,z), @xyz{$=B($*2)-8,} s*=.5,} (L(x,y,z)-8)*s",
        //     String::from("Seq([Blob(\"s=1; \")])"),
        // ));
        dbg!(parse_macro.parse_peek(input));
        // assert_eq!(
        //     parse_macro.map(|e| format!("{e:?}")).parse_peek(input),
        //     expected
        // );
    }

    #[test]
    fn nested() {
        let input = "@2{ [x,y]=r0(x,y), [x,z]=r1(x,z), @xyz{$=B($*2)-8,} s*=.5,} (L(x,y,z)-8)*s";
        let expected = Ok((
            "@2{ [x,y]=r0(x,y), [x,z]=r1(x,z), @xyz{$=B($*2)-8,} s*=.5,} (L(x,y,z)-8)*s",
            String::from("Blob(\"s=1; \")"),
        ));
        assert_eq!(
            parse_macro.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );
    }

    #[test]
    fn for_numeric_test() {
        let input = "@2{ [x,y]=r0(x,y), [x,z]=r1(x,z)}";
        let expected = Ok((
            "}",
            String::from(
                "Seq([ForNumeric { n: 2, block: Seq([Blob(\"[x,y]=r0(x,y), [x,z]=r1(x,z)\")]) }])",
            ),
        ));
        assert_eq!(
            parse_macro.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );
    }

    #[test]
    fn for_alpha_test() {
        let input = "@xyz{$=B($*2)-8,}";
        let expected = Ok((
            "}",
            String::from("Seq([ForAlpha { a: \"xyz\", block: Seq([Blob(\"$=B($*2)-8,\")]) }])"),
        ));
        assert_eq!(
            parse_macro.map(|e| format!("{e:?}")).parse_peek(input),
            expected
        );
    }

    // #[test]
    // fn expand_1() {
    //     let input = "@xyz{$=B($)-6,} L(x,y,z)";
    //     let expected = "x=B(x)-6,y=B(y)-6,z=B(z)-6, L(x,y,z)";
    //     let vs = expand_macros(input.as_bytes(), vec![0]);
    //     assert_eq!(bytes_to_string(&vs), expected,);
    // }
    // #[test]
    // fn expand_2() {
    //     let input =
    //         "s=1; @2{ [x,y]=r0(x,y), [x,z]=r1(x,z), @xyz{$=B($*2)-8,} s*=.5,} (L(x,y,z)-8)*s";
    //     // let expected = "x=B(x)-6,y=B(y)-6,z=B(z)-6, L(x,y,z)";
    //     let vs = expand_macros(input.as_bytes(), vec![0]);
    //     assert_eq!(bytes_to_string(&vs), "",);
    // }
}
