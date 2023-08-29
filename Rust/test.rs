// This is a comment, and is ignored by the compiler.
// You can test this code by clicking the "Run" button over there ->
// or if you prefer to use your keyboard, you can use the "Ctrl + Enter"
// shortcut.

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

// This is the main function.
fn main() {
    // Statements here are executed when the compiled binary is called.

    // Print text to the console.
    println!("Hello World!");
}

/*
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let dir_path = "."; // Specify the directory path you want to list files in

    // Read the directory and get a list of directory entries
    let entries = fs::read_dir(dir_path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        // Check if the entry is a file
        if path.is_file() {
            println!("File: {}", path.display());
        }

        // You can also check for directories, symlinks, etc., using similar conditions
    }

    Ok(())
}
*/