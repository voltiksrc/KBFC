// SPDX-License-Identifier: MPL-2.0

pub fn name_is_valid(name: &str) -> bool {
    if name.is_empty() {
        eprintln!("[KBFC]: Empty project name!");
        return false;
    }

    // Deny path traversal otherwise we could make a project
    // in a unexpected place.
    if name == "." || name == ".." {
        return false;
    }

    if name.contains('/') {
        return false;
    }

    true
}
