// SPDX-License-Identifier: MPL-2.0

use std::fs;
use std::path::Path;

const MAIN_TEMPLATE: &str = include_str!("../templates/main.c");
const MAKEFILE_TEMPLATE: &str = include_str!("../templates/Makefile");
const GITIGNORE_TEMPLATE: &str = include_str!("../templates/gitignore");

pub fn create_project(name: &str) -> std::io::Result<()> {
    let root = Path::new(name);

    fs::create_dir(root)?;
    fs::create_dir(root.join("src"))?;
    fs::create_dir(root.join("include"))?;

    fs::write(root.join("src/main.c"), MAIN_TEMPLATE)?;
    fs::write(root.join("README.md"), format!("# {name}\n"))?;

    // We make sure to substitute our placeholder variable in the files
    // with the actual supplyed project name from the user.
    let makefile = MAKEFILE_TEMPLATE.replace("{{PROJECT_NAME}}", name);

    fs::write(root.join("Makefile"), makefile)?;

    let gitignore = GITIGNORE_TEMPLATE.replace("{{PROJECT_NAME}}", name);

    fs::write(root.join(".gitignore"), gitignore)?;

    Ok(())
}
