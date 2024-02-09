pub fn expand_macro(input: &str) -> String {
    let mut output = String::new();
    let mut i = 0;
    while i < input.len() {
        let mut current_char = input.chars().nth(i).unwrap();
        if current_char == '@' {
            i += 1; // Move past '@'
            let mut macro_type = String::new();
            while i < input.len() && input.chars().nth(i).unwrap() != '{' {
                macro_type.push(input.chars().nth(i).unwrap());
                i += 1;
            }
            i += 1; // Move past '{'
            let mut content = String::new();
            let mut depth = 1;
            while i < input.len() && depth > 0 {
                current_char = input.chars().nth(i).unwrap();
                if current_char == '{' {
                    depth += 1;
                } else if current_char == '}' {
                    depth -= 1;
                    if depth == 0 {
                        break; // Don't include the closing '}' in content
                    }
                }
                content.push(current_char);
                i += 1;
            }
            let expanded_content = expand_macro(&content); // Handle nested macros
            output += &process_macro(&macro_type, &expanded_content);
        } else {
            output.push(current_char);
        }
        i += 1;
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
        dbg!(expand_macro(input));
        let input = "s=10; @1{a=sin(y),b=sin(x),c=sin(z),d=x,e=s+1,}; SM(a,b,c,d,e)-5";
        dbg!(expand_macro(input));
        let input = "@xyz{$=B($)-6,} L(x,y,z)-5";
        dbg!(expand_macro(input));
    }
}
