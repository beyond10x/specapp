//! Path containment: every path a manifest names stays inside the pack.

use std::path::{Component, Path, PathBuf};

/// Checks the text of a relative path or glob before anything is read: it is not empty, not
/// absolute, has no `..` component and no backslash.
pub fn relative_inside(text: &str) -> Result<(), String> {
    if text.is_empty() {
        return Err(String::from("the path is empty"));
    }
    if text.contains('\\') {
        return Err(format!("{text} has a backslash"));
    }
    let path = Path::new(text);
    for component in path.components() {
        match component {
            Component::Normal(_) | Component::CurDir => {}
            Component::ParentDir => return Err(format!("{text} goes up out of the pack")),
            Component::RootDir | Component::Prefix(_) => {
                return Err(format!("{text} is absolute"));
            }
        }
    }
    Ok(())
}

/// Resolves a path that exists, symbolic links included, and checks that it lies inside the
/// pack directory `root` (itself resolved).
pub fn resolved_inside(root: &Path, path: &Path) -> Result<PathBuf, String> {
    let resolved = path
        .canonicalize()
        .map_err(|error| format!("{} cannot be read: {error}", path.display()))?;
    if resolved.starts_with(root) {
        Ok(resolved)
    } else {
        Err(format!(
            "{} resolves to a place outside the pack",
            path.display()
        ))
    }
}

/// The path of `path` relative to `root`, with forward slashes, for a message.
pub fn shown(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .components()
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join("/")
}
