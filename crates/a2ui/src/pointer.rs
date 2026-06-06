//! JSON Pointer resolution (RFC 6901) with A2UI relative path extension.
//!
//! A2UI paths can be:
//! - Absolute: `/foo/bar` — resolved from the data model root.
//! - Relative: `foo/bar` — resolved from the current data context base path.

use serde_json::Value;

/// Resolve a JSON Pointer path against a JSON value.
///
/// Follows RFC 6901: tokens are separated by `/`, with `~0` for `~` and `~1`
/// for `/` escaping.
///
/// Returns `None` if the path does not exist in the value.
pub fn resolve(root: &Value, pointer: &str) -> Option<Value> {
    if pointer.is_empty() || pointer == "/" {
        return Some(root.clone());
    }

    // serde_json's built-in pointer expects a leading `/`
    let normalized = if pointer.starts_with('/') {
        pointer.to_string()
    } else {
        format!("/{pointer}")
    };

    root.pointer(&normalized).cloned()
}

/// Set a value at the given JSON Pointer path, creating intermediate
/// objects/arrays as needed (auto-vivification).
///
/// Returns the mutated root value.
pub fn set_at_path(root: &mut Value, path: &str, value: Value) {
    let path = if path.is_empty() || path == "/" {
        // Replace entire root
        *root = value;
        return;
    } else if path.starts_with('/') {
        &path[1..]
    } else {
        path
    };

    let tokens: Vec<&str> = path.split('/').collect();
    set_recursive(root, &tokens, value);
}

fn set_recursive(current: &mut Value, tokens: &[&str], value: Value) {
    if tokens.is_empty() {
        *current = value;
        return;
    }

    let token = unescape_token(tokens[0]);
    let rest = &tokens[1..];

    // Check if token is an array index
    if let Ok(index) = token.parse::<usize>() {
        // Ensure current is an array
        if !current.is_array() {
            *current = Value::Array(Vec::new());
        }
        let arr = current.as_array_mut().unwrap();
        // Extend array if needed
        while arr.len() <= index {
            arr.push(Value::Null);
        }
        set_recursive(&mut arr[index], rest, value);
    } else {
        // Ensure current is an object
        if !current.is_object() {
            *current = Value::Object(serde_json::Map::new());
        }
        let obj = current.as_object_mut().unwrap();
        let entry = obj
            .entry(token.clone())
            .or_insert_with(|| Value::Null);
        set_recursive(entry, rest, value);
    }
}

/// Remove a value at the given JSON Pointer path.
///
/// Returns the removed value, if any.
pub fn remove_at_path(root: &mut Value, path: &str) -> Option<Value> {
    if path.is_empty() || path == "/" {
        let old = root.clone();
        *root = Value::Object(serde_json::Map::new());
        return Some(old);
    }

    let path = if path.starts_with('/') {
        &path[1..]
    } else {
        path
    };

    let tokens: Vec<&str> = path.split('/').collect();
    if tokens.is_empty() {
        return None;
    }

    remove_recursive(root, &tokens)
}

fn remove_recursive(current: &mut Value, tokens: &[&str]) -> Option<Value> {
    if tokens.len() == 1 {
        let token = unescape_token(tokens[0]);
        if let Ok(index) = token.parse::<usize>() {
            if let Some(arr) = current.as_array_mut() {
                if index < arr.len() {
                    return Some(arr.remove(index));
                }
            }
        } else if let Some(obj) = current.as_object_mut() {
            return obj.remove(&token);
        }
        return None;
    }

    let token = unescape_token(tokens[0]);
    let rest = &tokens[1..];

    if let Ok(index) = token.parse::<usize>() {
        current
            .as_array_mut()
            .and_then(|arr| arr.get_mut(index))
            .and_then(|child| remove_recursive(child, rest))
    } else {
        current
            .as_object_mut()
            .and_then(|obj| obj.get_mut(&token))
            .and_then(|child| remove_recursive(child, rest))
    }
}

/// Make an absolute path from a possibly-relative path and a base path.
///
/// - If `path` starts with `/`, it's already absolute — return as-is.
/// - Otherwise, join with `base_path`.
pub fn make_absolute(path: &str, base_path: &str) -> String {
    if path.starts_with('/') {
        path.to_string()
    } else if base_path.is_empty() {
        format!("/{path}")
    } else {
        let base = base_path.trim_end_matches('/');
        format!("{base}/{path}")
    }
}

/// Unescape a single JSON Pointer token per RFC 6901.
fn unescape_token(token: &str) -> String {
    token.replace("~1", "/").replace("~0", "~")
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn resolve_root() {
        let data = json!({"a": 1});
        assert_eq!(resolve(&data, ""), Some(data.clone()));
        assert_eq!(resolve(&data, "/"), Some(data.clone()));
    }

    #[test]
    fn resolve_absolute() {
        let data = json!({"user": {"name": "Alice", "age": 30}});
        assert_eq!(resolve(&data, "/user/name"), Some(json!("Alice")));
        assert_eq!(resolve(&data, "/user/age"), Some(json!(30)));
    }

    #[test]
    fn resolve_relative() {
        let data = json!({"name": "Bob"});
        // Relative paths get a leading `/` prepended
        assert_eq!(resolve(&data, "name"), Some(json!("Bob")));
    }

    #[test]
    fn resolve_array_index() {
        let data = json!({"items": ["a", "b", "c"]});
        assert_eq!(resolve(&data, "/items/1"), Some(json!("b")));
    }

    #[test]
    fn resolve_missing() {
        let data = json!({"a": 1});
        assert_eq!(resolve(&data, "/b"), None);
        assert_eq!(resolve(&data, "/a/b"), None);
    }

    #[test]
    fn set_simple() {
        let mut data = json!({});
        set_at_path(&mut data, "/name", json!("Alice"));
        assert_eq!(data, json!({"name": "Alice"}));
    }

    #[test]
    fn set_nested() {
        let mut data = json!({});
        set_at_path(&mut data, "/user/name", json!("Bob"));
        assert_eq!(data, json!({"user": {"name": "Bob"}}));
    }

    #[test]
    fn set_array_index() {
        let mut data = json!({"items": ["a", "b"]});
        set_at_path(&mut data, "/items/2", json!("c"));
        assert_eq!(data, json!({"items": ["a", "b", "c"]}));
    }

    #[test]
    fn set_root() {
        let mut data = json!({});
        set_at_path(&mut data, "/", json!({"replaced": true}));
        assert_eq!(data, json!({"replaced": true}));
    }

    #[test]
    fn remove_key() {
        let mut data = json!({"a": 1, "b": 2});
        let removed = remove_at_path(&mut data, "/a");
        assert_eq!(removed, Some(json!(1)));
        assert_eq!(data, json!({"b": 2}));
    }

    #[test]
    fn remove_array_element() {
        let mut data = json!({"items": ["a", "b", "c"]});
        let removed = remove_at_path(&mut data, "/items/1");
        assert_eq!(removed, Some(json!("b")));
        assert_eq!(data, json!({"items": ["a", "c"]}));
    }

    #[test]
    fn make_absolute_already_absolute() {
        assert_eq!(make_absolute("/foo/bar", "/base"), "/foo/bar");
    }

    #[test]
    fn make_absolute_relative_with_base() {
        assert_eq!(make_absolute("name", "/items/0"), "/items/0/name");
    }

    #[test]
    fn make_absolute_relative_empty_base() {
        assert_eq!(make_absolute("name", ""), "/name");
    }
}
