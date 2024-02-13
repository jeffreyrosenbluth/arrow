pub fn expand(input: &str) -> String {
    let mut in_chars = input.chars();
    let mut output = String::new();
    let mut current_char = in_chars.next();
    while let Some(c) = current_char {
        if c == '@' {
            let mut macro_type = String::new();
            while let Some(c) = in_chars.next() {
                if c == '{' {
                    break;
                }
                macro_type.push(c);
            }
            let mut content = String::new();
            let mut depth = 1;
            while let Some(c) = in_chars.next() {
                if c == '{' {
                    depth += 1;
                } else if c == '}' {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                content.push(c);
            }
            let expanded_content = expand(&content);
            output += &process_macro(&macro_type, &expanded_content);
        } else {
            output.push(c);
        }
        current_char = in_chars.next();
    }
    output
}

fn process_macro(macro_type: &String, content: &String) -> String {
    let mut result = String::new();
    if let Ok(repeats) = macro_type.parse::<usize>() {
        // Number-based macro
        for i in 1..=repeats {
            result += &content.replace("$", &i.to_string());
        }
    } else {
        // Character-based macro
        for ch in macro_type.chars() {
            result += &content.replace("$", &ch.to_string());
        }
    }
    result
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn expand_test() {
        let input =
            "s=1; @2{ [x,y]=r0(x,y), [x,z]=r1(x * $,z), @xyz{$=B($*2)-8,} s*=.5,} (L(x,y,z)-8)*s";
        dbg!(expand(input));
        let input = "s=10; @1{a=sin(y),b=sin(x),c=sin(z),d=x,e=s+1,}; SM(a,b,c,d,e)-5";
        dbg!(expand(input));
        let input = "@xyz{$=B($)-6,} L(x,y,z)-5";
        dbg!(expand(input));
        let input = "s=2.5,h=s/2,d=(s+h)/2,q=20,y-=10,[x,y]=r0(x,y),@xyz{$/=q,}c=1,t=0,@7{@xyz{$=mod($-h,s)-h,}t=d/D([x,y,z],[x,y,z]),@xyzc{$*=t,}}d=L(x,y,z)/c*2.-.025";
        dbg!(expand(input));
    }
}
