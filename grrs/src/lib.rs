use std::io::Write;

pub fn find_matches(content: &str, pattern: &str, mut writer: impl Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            if let Err(e) = writeln!(writer, "{}", line) {
                println!("Writing error: {}", e.to_string());
            }
        }
    }
}

// Init some tests
#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum/ndolor sit amet", "lorem", &mut result);

    assert_eq!(result, b"lorem ipsum")
}
