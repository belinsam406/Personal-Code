use std::fs;
use std::io;
use std::path;
//command for trash mv filename ~/.Trash/

fn main() -> io::Result<()> {
    let dir_path = "/Users/sambelin/Desktop"; // Specify the directory path you want to list files in

    // Read the directory and get a list of directory entries
    let entries = fs::read_dir(dir_path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        // Check if the entry is a file
        if path.is_file(){
            let metadata = fs::metadata(&path)?;

            // println!("File: {}", path.display());
            // println!("File Type: {:?}", metadata.file_type());

            println!(ImageCheck(&path));


            //need to get file type and check if image type
            //get file name
            //run trash mv filename ~/.Trash/ utill all image files are in trash


            // if metadata.is_file() {
            //     println!("File: {}", path.display());
            // }
        }

        // You can also check for directories, symlinks, etc., using similar conditions
    }

    Ok(())
}

fn ImageCheck(filePath) -> bool{
    let file_path = "filePath"; // Replace with your file's path

    let path = Path::new(file_path);
    if let Some(ext) = path.extension() {
        if ext == "jpg" {
            println!("The file is a JPEG image.");
            return true;
        } else {
            println!("The file is not a JPEG image.");
            return false;
        }
    } else {
        println!("The file has no extension.");
        return false;
    }
}