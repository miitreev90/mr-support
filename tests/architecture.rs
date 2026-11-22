// Uncle Bob fitness functions: architectural rules enforced in CI
// Rule 1: No unwrap()/expect() in production code (only tests)
// Rule 2: No panic! in production code
// Rule 3: Modules stay small (Clean Code: functions < 50 lines enforced via clippy)

#[test]
fn test_no_unwrap_in_production() {
    let src_dir = std::path::Path::new("src");
    let mut violations = Vec::new();
    for entry in walkdir(src_dir) {
        if entry.extension().map(|e| e == "rs").unwrap_or(false) {
            let content = std::fs::read_to_string(&entry).unwrap();
            // Strip test modules crudely: only check before #[cfg(test)]
            let prod = content.split("#[cfg(test)]").next().unwrap_or(&content);
            if prod.contains(".unwrap()") || prod.contains(".expect(") {
                violations.push(entry.display().to_string());
            }
        }
    }
    assert!(
        violations.is_empty(),
        "Uncle Bob rule: no unwrap/expect in production: {:?}",
        violations
    );
}

#[test]
fn test_no_panic_in_production() {
    let src_dir = std::path::Path::new("src");
    let mut violations = Vec::new();
    for entry in walkdir(src_dir) {
        if entry.extension().map(|e| e == "rs").unwrap_or(false) {
            let content = std::fs::read_to_string(&entry).unwrap();
            let prod = content.split("#[cfg(test)]").next().unwrap_or(&content);
            if prod.contains("panic!(")
                || prod.contains("unimplemented!(")
                || prod.contains("todo!(")
            {
                violations.push(entry.display().to_string());
            }
        }
    }
    assert!(
        violations.is_empty(),
        "Uncle Bob rule: no panic in production: {:?}",
        violations
    );
}

fn walkdir(dir: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(walkdir(&path));
            } else {
                files.push(path);
            }
        }
    }
    files
}
