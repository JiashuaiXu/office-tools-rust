use std::path::Path;

/// 检查文件是否为 Python 脚本
pub fn is_python_script(path: &Path) -> bool {
    path.extension()
        .map(|ext| ext == "py")
        .unwrap_or(false)
}

/// 清理脚本名称，移除不安全字符
pub fn sanitize_script_name(name: &str) -> String {
    name.replace(['/', '\\'], "_")
        .replace(['<', '>', ':', '"', '|', '?', '*'], "")
}

/// 获取脚本的相对路径
pub fn get_script_path(name: &str) -> String {
    format!("scripts/{}.py", sanitize_script_name(name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_is_python_script() {
        assert!(is_python_script(&PathBuf::from("test.py")));
        assert!(!is_python_script(&PathBuf::from("test.txt")));
        assert!(!is_python_script(&PathBuf::from("test")));
    }

    #[test]
    fn test_sanitize_script_name() {
        assert_eq!(sanitize_script_name("test/script"), "test_script");
        assert_eq!(sanitize_script_name("test<>:script"), "testscript");
        assert_eq!(sanitize_script_name("normal_script"), "normal_script");
    }

    #[test]
    fn test_get_script_path() {
        assert_eq!(get_script_path("test"), "scripts/test.py");
        assert_eq!(get_script_path("test/script"), "scripts/test_script.py");
    }
} 