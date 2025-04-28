// Parse the problem solutions from 'rust/solutions' directory and update Cargo.toml

use toml::{Table, Value};
use std::collections::HashSet;
use indicatif::{ProgressBar, ProgressStyle};

// Helper function to extract problem number from problem name (e.g., "p123" -> 123)
fn extract_problem_number(problem_name: &str) -> Option<u32> {
    if problem_name.starts_with('p') {
        problem_name[1..].parse::<u32>().ok()
    } else {
        None
    }
}

fn main() {
    println!("Updating Cargo.toml...");

    // Get project root
    let root = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");

    // Parse source files from src
    let solutions_dir = root.clone() + "/rust/solutions";
    let entries = std::fs::read_dir(&solutions_dir).unwrap().collect::<Result<Vec<_>, _>>().unwrap();

    // Create a progress bar for scanning files
    let scan_pb = ProgressBar::new(entries.len() as u64);
    scan_pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} files scanned ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    // Filter problem files
    let problems = entries.into_iter()
        .filter_map(|entry| {
            scan_pb.inc(1);

            // If entry is a file named p*.rs then return its name
            if entry.path().is_file()
                && entry.path().extension() == Some(std::ffi::OsStr::new("rs"))
                && entry.file_name().to_string_lossy().starts_with("p")
            {
                Some(
                    entry.file_name()
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
        .collect::<Vec<_>>();

    scan_pb.finish_with_message("Files scanned");

    // Update Cargo.toml
    let cargo_toml = std::fs::read_to_string(root.clone() + "/Cargo.toml").unwrap();
    let mut table = toml::from_str::<Table>(&cargo_toml).unwrap();

    // Get existing bins
    let bins = table["bin"].as_array().unwrap().clone();
    let name_set: HashSet<&str> = bins.iter().map(|x| x["name"].as_str().unwrap()).collect();

    // Create a progress bar for processing problems
    let process_pb = ProgressBar::new(problems.len() as u64);
    process_pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} problems processed ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    // Add missing problems to Cargo.toml
    for problem in problems {
        process_pb.inc(1);

        // Add problem to Cargo.toml as a binary if not exists
        if !name_set.contains(&problem.as_str()) {
            // Only print a message for newly added problems
            println!("Adding new problem: {} to Cargo.toml", problem);

            let mut bin = Table::new();
            bin.insert("name".to_string(), Value::String(problem.clone()));
            bin.insert(
                "path".to_string(),
                Value::String(String::from("rust/solutions/") + &problem + ".rs"),
            );
            table["bin"]
                .as_array_mut()
                .unwrap()
                .push(Value::Table(bin));
        }
    }

    process_pb.finish_with_message("Problems processed");

    // Create a progress bar for organizing Cargo.toml
    let organize_pb = ProgressBar::new(3);
    organize_pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] {msg}")
        .unwrap());
    organize_pb.set_message("Organizing Cargo.toml sections...");

    // Separate the update bin from problem bins
    let bins = table["bin"].as_array().unwrap().clone();
    let mut update_bin = None;
    let mut problem_bins = Vec::new();

    for bin in bins {
        let name = bin["name"].as_str().unwrap();
        if name == "update" {
            update_bin = Some(bin.clone());
        } else {
            problem_bins.push(bin.clone());
        }
    }

    organize_pb.inc(1);
    organize_pb.set_message("Sorting problem bins by ID...");

    // Sort problem bins by problem number
    problem_bins.sort_by(|a, b| {
        let a_name = a["name"].as_str().unwrap();
        let b_name = b["name"].as_str().unwrap();

        let a_num = extract_problem_number(a_name).unwrap_or(u32::MAX);
        let b_num = extract_problem_number(b_name).unwrap_or(u32::MAX);

        a_num.cmp(&b_num)
    });

    organize_pb.inc(1);
    organize_pb.set_message("Rebuilding Cargo.toml with proper ordering...");

    // Create a new ordered content string - we'll build this manually to ensure correct order
    let mut ordered_content = String::new();

    // 1. Add package section first (if it exists)
    if table.contains_key("package") {
        ordered_content.push_str(&toml::to_string(&Table::from_iter([("package".to_string(), table["package"].clone())])).unwrap());
        ordered_content.push_str("\n\n");
    }

    // 2. Add dependencies section second (if it exists)
    if table.contains_key("dependencies") {
        ordered_content.push_str(&toml::to_string(&Table::from_iter([("dependencies".to_string(), table["dependencies"].clone())])).unwrap());
        ordered_content.push_str("\n\n");
    }

    // 3. Add update bin third (if it exists)
    if let Some(update) = update_bin {
        ordered_content.push_str("[[bin]]\n");
        ordered_content.push_str(&format!("name = \"{}\"\n", update["name"].as_str().unwrap()));
        ordered_content.push_str(&format!("path = \"{}\"\n\n", update["path"].as_str().unwrap()));
    }

    // 4. Add all problem bins
    for bin in &problem_bins {
        ordered_content.push_str("[[bin]]\n");
        ordered_content.push_str(&format!("name = \"{}\"\n", bin["name"].as_str().unwrap()));
        ordered_content.push_str(&format!("path = \"{}\"\n\n", bin["path"].as_str().unwrap()));
    }

    // Write the ordered Cargo.toml
    std::fs::write(root.clone() + "/Cargo.toml", ordered_content).unwrap();

    organize_pb.inc(1);
    organize_pb.finish_with_message("Cargo.toml updated successfully!");
}
