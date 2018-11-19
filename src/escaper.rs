
use std::borrow::Cow;



const BACKSLASH: char = '\\';
const UNSAFES: [char;6] = [BACKSLASH, ' ', '\t', '\r', '\n', '\''];
const DOLLARS: [char;2] = ['\r', '\n'];


pub struct Escaper();



impl Escaper {
    pub fn escape(&self, s: &str) -> String {
        if s.contains(should_dollar) {
            return self.dollar_escape(s);
        }

        let mut result = String::with_capacity(s.len() + 2);
        let mut first = true;

        for it in s.split('\'') {
            if first {
                first = false;
            } else {
                result.push_str("\\'");
            }
            result.push_str(&quote_escape(it));
        }

        result
    }

    pub fn dollar_escape(&self, s: &str) -> String {
        let mut result = String::with_capacity(s.len() + 2);

        for c in s.chars() {
            match c {
                '\r' => result.push_str("\\r"),
                '\n' => result.push_str("\\n"),
                '\\' => result.push_str("\\\\"),
                '\'' => result.push_str("\\'"),
                _ => result.push(c),
            }
        }

        format!("$'{}'", result)
    }
}


fn is_unsafe(c: char) -> bool {
    UNSAFES.contains(&c)
}

fn should_dollar(c: char) -> bool {
    DOLLARS.contains(&c)
}

fn quote_escape<'a>(s: &'a str) -> Cow<'a, str> {
    if !s.contains(is_unsafe) {
        return s.into();
    }

    let mut result = String::with_capacity(s.len() + 2);

    for c in s.chars() {
        result.push(c);
    }

    format!("'{}'", result).into()
}




#[cfg(test)]
mod tests {
    use escaper::*;

    #[test]
    fn test_escape() {
        let e: Escaper = Escaper();

        assert_eq!(e.escape("foo"), "foo");
        assert_eq!(e.escape("ねこ"), "ねこ");
        assert_eq!(e.escape("ね こ"), "'ね こ'");

        assert_eq!(e.escape("ね'こ"), "ね\\'こ");
        assert_eq!(e.escape("ねこ'きゃっと'です"), "ねこ\\'きゃっと\\'です");
        assert_eq!(e.escape("ね こ'きゃっと'です"), "'ね こ'\\'きゃっと\\'です");

        assert_eq!(e.escape("ね\nこ"), "$'ね\\nこ'");
        assert_eq!(e.escape("ね\n'こ"), "$'ね\\n\\'こ'");
    }
}
