// Parse the problem solutions from 'src' directory and update Cargo.toml

use toml::Table;

fn main() {
    // Get project root
    let root = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
    // Parse source files from src
    let problems = std::fs::read_dir(root.clone() + "/src")
        .unwrap()
        .filter_map(|x| {
            // If x is a file named p*.rs then return its name
            x.ok().and_then(|x| {
                if x.path().is_file()
                    && x.path().extension() == Some(std::ffi::OsStr::new("rs"))
                    && x.file_name().to_string_lossy().starts_with("p")
                {
                    Some(
                        x.file_name()
                            .to_string_lossy()
                            .to_string()
                            .strip_suffix(".rs")
                            .unwrap()
                            .to_string(),
                    )
                } else {
                    None
                }
            })
        });
    // Update Cargo.toml
    let cargo_toml = std::fs::read_to_string(root.clone() + "/Cargo.toml").unwrap();
    let mut table = toml::from_str::<Table>(&cargo_toml).unwrap();
    let bins = table["bin"].as_array().unwrap().clone();
    let name_set: std::collections::HashSet<&str> =
        bins.iter().map(|x| x["name"].as_str().unwrap()).collect();
    // Add missing problems to Cargo.toml
    for problem in problems {
        // Add problem to Cargo.toml as a binary if not exists
        if !name_set.contains(&problem.as_str()) {
            println!("Adding {} to Cargo.toml", problem);
            let mut bin = Table::new();
            bin.insert("name".to_string(), toml::Value::String(problem.clone()));
            bin.insert(
                "path".to_string(),
                toml::Value::String(String::from("src/") + &problem + ".rs"),
            );
            table["bin"]
                .as_array_mut()
                .unwrap()
                .push(toml::Value::Table(bin));
        }
    }
    // Make sure bins are sorted
    table["bin"]
        .as_array_mut()
        .unwrap()
        .sort_by(|a, b| a["name"].as_str().unwrap().cmp(b["name"].as_str().unwrap()));
    // Write Cargo.toml
    std::fs::write(
        root.clone() + "/Cargo.toml",
        toml::to_string(&table).unwrap(),
    )
    .unwrap();
}
