extern crate detector;

#[cfg(test)]
mod tests {
    use detector;
    use detector::language as l;
    use detector::language::Language;
    use std::ffi::OsStr;
    use std::fs::File;
    use std::path::Path;

    #[test]
    fn js_find_with_comments() {
        let path = Path::new("tests/resources/js-with-comments.js");
        let result = l::javascript::Struct {
            file: File::open(path),
        }.find();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 8);
    }

    #[test]
    fn js_find_without_comments() {
        let path = Path::new("tests/resources/js-without-comments.js");
        let result = l::javascript::Struct {
            file: File::open(path),
        }.find();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn resolve_type_with_supported_file() {
        let path = Path::new("tests/resources/js-with-comments.js");
        let result = detector::resolve_type(path);
        assert!(result.is_ok());
    }

    #[test]
    fn resolve_type_with_unsupported_file() {
        let path = Path::new("tests/resources/unsupported.arb");
        let result = detector::resolve_type(path);
        assert!(result.is_err());
    }

    #[test]
    fn exists_on_filesystem_true() {
        let path = OsStr::new("tests/resources/unsupported.arb");
        let result = detector::exists_on_filesystem(path);
        assert!(result.is_ok());
    }

    #[test]
    fn exists_on_filesystem_false() {
        let path = OsStr::new("tests/resources/does_not_exist");
        let result = detector::exists_on_filesystem(path);
        assert!(result.is_err());
    }
}
