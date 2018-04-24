use filesystem::utils::hash::Hash;

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

pub fn match_hash<'a>(data: &'a str) -> Option<(Hash, &'a str)> {
    let len = data.len();

    if len >= 40 {
        let (head, rest) = data.split_at(40);

        for char_item in head.as_bytes() {
            if char_item.is_ascii_hexdigit() == false {
                return None;
            }
        }

        return Some(
            (Hash::from_string(head), rest)
        );
    }

    None
}

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
