#[allow(dead_code)]
pub fn simplify_path(path: String) -> String {
    let mut s_parts = vec![];
    s_parts.push("");

    for part in path.split("/") {
        match part {
            "" => continue,
            "." => continue,
            ".." => {
                if s_parts.len() > 1 {
                    s_parts.pop();
                }
            }
            _ => s_parts.push(part),
        }
    }

    if s_parts.len() <= 1 {
        return String::from("/");
    }

    s_parts.join("/").to_string()
}

#[cfg(test)]
mod simplify_path_tests {
    use crate::simplify_path::simplify_path;

    #[test]
    fn lc_case_1() {
        assert_eq!("/home", simplify_path("/home/".to_owned()));
    }

    #[test]
    fn lc_case_2() {
        assert_eq!("/home/foo", simplify_path("/home//foo".to_owned()));
    }

    #[test]
    fn lc_case_3() {
        assert_eq!(
            "/home/user/Pictures",
            simplify_path("/home/user/Documents/../Pictures".to_owned())
        );
    }

    #[test]
    fn lc_case_4() {
        assert_eq!("/", simplify_path("/../".to_owned()));
    }

    #[test]
    fn lc_case_5() {
        assert_eq!(
            "/.../b/d",
            simplify_path("/.../a/../b/c/../d/./".to_owned())
        );
    }

    #[test]
    fn beav_case_1() {
        assert_eq!(
            "/a/b",
            simplify_path("////////////////////////////////a////////////////////b".to_owned())
        );
    }

    #[test]
    fn does_handle_nested_() {
        assert_eq!(
            "/a/b",
            simplify_path("////////////////////////////////a////////////////////b".to_owned())
        );
    }

    #[test]
    fn does_handle_clear_to_root() {
        assert_eq!("/a", simplify_path("/../a".to_owned()));
    }

    #[test]
    fn does_handle_clear_to_root_2() {
        assert_eq!("/", simplify_path("/a/..".to_owned()));
    }

    #[test]
    fn does_handle_current_dir_child() {
        assert_eq!("/a/a", simplify_path("/a/./a".to_owned()));
    }

    #[test]
    fn does_handle_current_dir_child_2() {
        assert_eq!("/a/b/c/d", simplify_path("/a/./b/./c/./d".to_owned()));
    }

    #[test]
    fn does_handle_nested_back_on_root() {
        assert_eq!("/a", simplify_path("/../../../../../../../../../../../../../../../../../../../../../../../../../../../../../../../../../../../../../../../../../../../../../../../../../../../../../../../../../../../../../../../../a/".to_owned()));
    }

    #[test]
    fn does_handle_slash_as_dir_name() {
        assert_eq!("/a/_", simplify_path("/a/_/.".to_owned()));
    }
}
