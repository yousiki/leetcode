use clap::{Arg, Command};
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs;
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use toml_edit::{ArrayOfTables, Document, Item, Table};

/// Scan the solutions directory for all .rs files (excluding main.rs)
fn scan_solution_files(solutions_dir: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut solution_files = Vec::new();

    for entry in fs::read_dir(solutions_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
            if let Some(file_name) = path.file_stem().and_then(|s| s.to_str()) {
                // Skip main.rs if it exists
                if file_name != "main" {
                    solution_files.push(file_name.to_string());
                }
            }
        }
    }

    solution_files.sort();

    Ok(solution_files)
}

/// Parse the existing Cargo.toml file
fn parse_cargo_toml(cargo_toml_path: &Path) -> Result<Document, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(cargo_toml_path)?;
    let doc = content.parse::<Document>()?;
    Ok(doc)
}

/// Update the Cargo.toml file with test entries for solution files
/// Example:
/// ```toml
/// [[test]]
/// name = "two_sum"
/// path = "two_sum.rs"
/// ```
fn update_cargo_toml(
    doc: &mut Document,
    solution_files: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    // Remove existing [[test]] entries
    doc.as_table_mut().retain(|key, _| !key.starts_with("test"));

    // Add new [[test]] entries
    for file_name in solution_files {
        let mut table = Table::new();
        table["name"] = Item::Value(file_name.clone().into());
        table["path"] = Item::Value(format!("{}.rs", file_name).into());
        doc["test"].or_insert(Item::ArrayOfTables(ArrayOfTables::new()));
        doc["test"].as_array_of_tables_mut().unwrap().push(table);
    }

    Ok(())
}

/// Run in single mode, updating the Cargo.toml once
fn run_single_mode(
    solutions_dir: &Path,
    cargo_toml_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // Scan the solutions directory for .rs files
    let solution_files = scan_solution_files(solutions_dir)?;

    // Parse the existing Cargo.toml file
    let mut doc = parse_cargo_toml(cargo_toml_path)?;

    // Update the Cargo.toml with library entries
    update_cargo_toml(&mut doc, &solution_files)?;

    // Write the updated Cargo.toml back to the file
    fs::write(cargo_toml_path, doc.to_string())?;

    println!(
        "Successfully updated Cargo.toml with {} solution(s)",
        solution_files.len()
    );

    Ok(())
}

/// Run in watch mode, monitoring the solutions directory for changes
fn run_watch_mode(
    solutions_dir: &Path,
    cargo_toml_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Starting watch mode for directory: {}",
        solutions_dir.display()
    );
    println!("Press Ctrl+C to stop watching...");

    // Set up a channel to receive file system events
    let (tx, rx) = mpsc::channel();

    // Create a watcher
    let mut watcher: RecommendedWatcher = Watcher::new(
        tx,
        Config::default()
            .with_poll_interval(Duration::from_secs(5))
            .with_compare_contents(false),
    )?;

    // Watch the solutions directory recursively
    watcher.watch(solutions_dir, RecursiveMode::NonRecursive)?;

    // Set up a handler for Ctrl+C
    let (ctrl_c_tx, ctrl_c_rx) = mpsc::channel();
    thread::spawn(move || {
        // Wait for Ctrl+C
        ctrl_c_rx.recv().unwrap();
        println!("\nReceived Ctrl+C, stopping watch mode...");
        std::process::exit(0);
    });

    // Set up the Ctrl+C handler
    ctrlc::set_handler(move || {
        let _ = ctrl_c_tx.send(());
    })
    .expect("Error setting Ctrl+C handler");

    // Main event loop
    loop {
        match rx.recv() {
            Ok(Ok(event)) => match event.kind {
                EventKind::Create(_) | EventKind::Remove(_) => {
                    println!("Detected file change, updating Cargo.toml...");
                    run_single_mode(solutions_dir, cargo_toml_path)?;
                }
                _ => {}
            },
            Ok(Err(e)) => {
                eprintln!("Watch error: {:?}", e);
            }
            Err(e) => {
                eprintln!("Channel error: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let matches = Command::new("Cargo.toml Updater")
        .version("0.1.0")
        .about("Updates Cargo.toml with library entries for solution files")
        .arg(
            Arg::new("watch")
                .help("Run in watch mode")
                .long("watch")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("solutions-dir")
                .help("Path to the solutions directory")
                .long("solutions-dir")
                .default_value("rust/solutions"),
        )
        .arg(
            Arg::new("cargo-toml")
                .help("Path to the Cargo.toml file")
                .long("cargo-toml")
                .default_value("rust/solutions/Cargo.toml"),
        )
        .get_matches();

    // Get the path to the solutions directory
    let solutions_dir = Path::new(matches.get_one::<String>("solutions-dir").unwrap());
    let cargo_toml_path = Path::new(matches.get_one::<String>("cargo-toml").unwrap());

    // Check if watch mode is enabled
    if matches.get_flag("watch") {
        run_watch_mode(solutions_dir, cargo_toml_path)
    } else {
        run_single_mode(solutions_dir, cargo_toml_path)
    }
}
