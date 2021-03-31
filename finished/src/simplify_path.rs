pub fn simplify_path(path: String) -> String {
    let mut stack = Vec::new();
    for part in path.split('/') {
        if part == ".." {
            let _ = stack.pop();
        } else if part == "." || part.is_empty() {
            continue;
        } else {
            stack.push(part);
        }
    }

    if stack.is_empty() {
        "/".into()
    } else {
        let mut result = String::new();
        for part in stack {
            result.push('/');
            result.push_str(part);
        }
        result
    }
}

#[test]
fn test() {
    let cases = vec![
        ("/home/", "/home"),
        ("/../", "/"),
        ("/home//foo/", "/home/foo"),
        ("/a/./b/../../c/", "/c"),
        ("/a/../../b/../c//.//", "/c"),
        ("/a//b////c/d//././/..", "/a/b/c"),
        ("/...", "/..."),
    ];
    for (path, expected) in cases {
        assert_eq!(simplify_path(path.to_string()), expected);
    }
}
