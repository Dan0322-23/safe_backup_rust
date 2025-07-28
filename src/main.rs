use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use chrono::Local;

fn main() {
    println!("Please enter your file name: ");
    let filename = read_input();

    println!("Please enter your command (backup, restore, delete): ");
    let command = read_input();

    match command.trim() {
        "backup" => {
            match backup_file(&filename) {
                Ok(_) => {
                    println!("Backup created: {}.bak", filename);
                    log_action(&format!("Performed backup on {}", filename));
                },
                Err(e) => eprintln!("Backup failed: {}", e),
            }
        },
        "restore" => {
            match restore_file(&filename) {
                Ok(_) => {
                    println!("File restored from {}.bak", filename);
                    log_action(&format!("Performed restore on {}", filename));
                },
                Err(e) => eprintln!("Restore failed: {}", e),
            }
        },
        "delete" => {
            println!("Are you sure you want to delete {}? (yes/no): ", filename);
            let confirm = read_input();
            if confirm.trim().eq_ignore_ascii_case("yes") {
                match delete_file(&filename) {
                    Ok(_) => {
                        println!("File deleted.");
                        log_action(&format!("Deleted file {}", filename));
                    },
                    Err(e) => eprintln!("Delete failed: {}", e),
                }
            } else {
                println!("Deletion cancelled.");
            }
        },
        _ => println!("Unknown command."),
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn backup_file(filename: &str) -> io::Result<()> {
    let input_path = Path::new(filename);
    let output_path = input_path.with_extension("bak");

    let input = File::open(&input_path)?;
    let reader = BufReader::new(input);

    let output = File::create(&output_path)?;
    let mut writer = BufWriter::new(output);

    for line in reader.lines() {
        writeln!(writer, "{}", line?)?;
    }

    Ok(())
}

fn restore_file(filename: &str) -> io::Result<()> {
    let backup_filename = format!("{}.bak", filename); // long-lived String
    let backup_path = Path::new(&backup_filename);     // borrow from that string
    let original_path = Path::new(filename);           // original file path

    let input = File::open(backup_path)?;              // use the borrowed path
    let reader = BufReader::new(input);

    let output = File::create(original_path)?;
    let mut writer = BufWriter::new(output);

    for line in reader.lines() {
        writeln!(writer, "{}", line?)?;
    }

    Ok(())
}



fn delete_file(filename: &str) -> io::Result<()> {
    fs::remove_file(filename)
}

fn log_action(action: &str) {
    let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();
    let log_line = format!("{} {}\n", timestamp, action);

    let log_result = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logfile.txt")
        .and_then(|mut file| file.write_all(log_line.as_bytes()));

    if let Err(e) = log_result {
        eprintln!("Failed to write log: {}", e);
    }
}
