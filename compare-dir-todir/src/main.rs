use std::env;
use std::fs::{self, File};
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};
use std::collections::HashSet;
use md5::{Md5, Digest};

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: dir_compare <dir1> <dir2> <output_dir>");
        return;
    }
    let dir1 = Path::new(&args[1]);
    let dir2 = Path::new(&args[2]);
    let output_dir = Path::new(&args[3]);
    let is_in_dir1 = dir_exists_in_dir(dir1, output_dir);
    let is_in_dir2 = dir_exists_in_dir(dir2, output_dir);
    if is_in_dir1 || is_in_dir2 {
        println!("Output directory can not in compared directory!");
        return;
    }
    // Get list of files in dir1 and dir2
    let files1 = get_files(&dir1);
    let files2 = get_files(&dir2);

    // Compare MD5 checksums of all files
    let mut unique_files: HashSet<PathBuf> = HashSet::new();
    for file1 in files1.iter() {
        match files2.get(file1.file_name().unwrap()) {
            Some(file2) => {
                if md5_checksum(file1) != md5_checksum(file2) {
                    println!("MD5 checksums do not match for file {:?}", file1);
                    unique_files.insert(file1.to_path_buf());
                    unique_files.insert(file2.to_path_buf());
                }
            }
            None => {
                println!("File {:?} does not exist in {:?}", file1.file_name().unwrap(), dir2);
                unique_files.insert(file1.to_path_buf());
            }
        }
    }
    for file2 in files2.iter() {
        if !files1.contains_key(file2.file_name().unwrap()) {
            println!("File {:?} does not exist in {:?}", file2.file_name().unwrap(), dir1);
            unique_files.insert(file2.to_path_buf());
        }
    }

    // Copy all unique files to output directory
    for file in unique_files.iter() {
        let dest_file = output_dir.join(file.file_name().unwrap());
        fs::copy(file, dest_file).expect("Failed to copy file");
    }
}

// Returns a hash map of file names to file paths in the given directory
fn get_files(dir: &Path) -> std::collections::HashMap<&str, PathBuf> {
    let mut files = std::collections::HashMap::new();
    for entry in fs::read_dir(dir).expect("Failed to read directory") {
        let path = entry.expect("Failed to get directory entry").path();
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            files.insert(file_name, path);
        }
    }
    return files
}

// Calculates the MD5 checksum of the given file
fn md5_checksum(file_path: &Path) -> String {
    let mut file = BufReader::new(File::open(file_path).expect("Failed to open file"));
    let mut hasher = md5::Md5::new();
    io::copy(&mut file, &mut hasher).expect("Failed to read file");
    format!("{:x}", hasher.finalize())
}



fn dir_exists_in_dir(parent_dir: &Path, child_dir: &Path) -> bool {
    let parent_metadata = fs::metadata(parent_dir).unwrap();
    if !parent_metadata.is_dir() {
        return false;
    }

    let child_path = PathBuf::from(parent_dir).join(child_dir);
    if let Err(_) = fs::metadata(&child_path) {
        return false;
    }

    child_path.starts_with(parent_dir)
}