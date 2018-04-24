pub fn match_str<'a>(data: &'a str, pattern: &'a str) -> Option<&'a str> {
    let pattern_len = pattern.len();

    if data.len() >= pattern_len {
        let (head, body) = data.split_at(pattern_len);
        if head == pattern {
            return Some(body);
        }
    }

    None
}

/*
8894bf287965630718070411aa284b2f6ed7b974/dir
*/

#[test]
fn match_str_test() {
    let aaa = "abcdef";

    assert_eq!(match_str(aaa, ""), Some("abcdef"));
    assert_eq!(match_str(aaa, "abc"), Some("def"));
    assert_eq!(match_str(aaa, "abcde"), Some("f"));
    assert_eq!(match_str(aaa, "abcdef"), Some(""));
    assert_eq!(match_str(aaa, "abd"), None);
    assert_eq!(match_str(aaa, "abdeffffff"), None);
    assert_eq!(match_str(aaa, "ffff"), None);
}
