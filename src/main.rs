use std::{
    alloc::System,
    cmp::max,
    env::{self, Args},
    fs,
    path::{self, Path},
    string,
    time::{SystemTime, UNIX_EPOCH},
};

use byte_unit::{Byte, UnitType};
use filesize::{PathExt, file_real_size};
use walkdir::WalkDir;

// Test
#[derive(Clone)]
struct File {
    name: String,
    filesize: u64,
    last_accessed: std::time::SystemTime,
    age_minutes: u64,
    path: String,
    score: f64,
    normalized_filesize: f64,
    normalized_age: f64,
}

impl File {
    fn new(
        filename: &str,
        path_string: &str,
        filesize: u64,
        last_accessed: std::time::SystemTime,
    ) -> Self {
        Self {
            name: filename.clone().to_owned(),
            path: path_string.clone().to_owned(),
            filesize: filesize,
            last_accessed: last_accessed,
            age_minutes: 0,
            score: 0.0,
            normalized_filesize: 0.0,
            normalized_age: 0.0,
        }
    }

    fn score_file(&mut self) {
        // let duration = self.get_duration();
        // println!("duration {} * filsize {}", duration, self.filesize);
        self.score = self.normalized_filesize * self.normalized_age;
        // self.age = /;
        // self.age = duration;
        // todo!()
    }

    fn get_duration(&mut self) -> u64 {
        let now = SystemTime::now();

        let since_result = now.duration_since(self.last_accessed);
        if since_result.is_err() {
            panic!();
        }
        let duration_seconds = since_result.unwrap().as_secs() / 60;
        // println!("\nduration: {}\n", duration_seconds);
        self.age_minutes = duration_seconds;
        // println!(
        // "File: {} | Last Accessed: {:?} | Age: {}",
        // self.name, self.last_accessed, self.age_minutes
        // );
        duration_seconds
    }

    fn print(&self) {
        // todo!()
        println!("filename {}", self.name);
        println!("path {}", self.path);
        println!("filesize {}", self.filesize);
        println!("score: {}", self.score);
        println!("\n\n");

        // println!("last_accessed {}", self.last_accessed.to);
    }

    fn normalize_values(&mut self, max_age: u64, max_size: u64) {
        self.normalized_age = self.age_minutes as f64 / max_age as f64;
        self.normalized_filesize = self.filesize as f64 / max_size as f64;
    }

    fn preview_score(&self) {
        let filesize_byte = Byte::from_u64(self.filesize);
        let filesize = filesize_byte
            .get_appropriate_unit(UnitType::Decimal)
            .to_string();

        println!(
            "{:<40} | {:<30} | {:<20}  | {} minutes old",
            self.name, self.score, filesize, self.age_minutes
        );
    }

    // fn clone(&self) -> File {
    // File::new(&self.name, &self.path, self.filesize, self.last_accessed)
    // }
}

fn print_score_guide() {
    // todo!()
    println!(
        "{:<40} | {:<30} | {:<20} | {:<30} ",
        "Filename", "Score", "Filesize in Bytes", "Age"
    );
}

fn main() {
    let mut space_parced: u64 = 0;
    let working_dir = find_target_working_directory();

    let mut files = load_files(space_parced, working_dir);

    let mut max_age: u64 = 0;
    let mut max_size: u64 = 0;
    println!("getting duration..");
    for file in &mut files {
        file.get_duration();
    }
    println!("finding max size and age..");
    for file in &files {
        max_age = max(max_age, file.age_minutes);
        max_size = max(max_size, file.filesize);
    }
    println!("max age: {} \n max size: {}", max_age, max_size);
    println!("normalizing results..");
    for file in &mut files {
        file.normalize_values(max_age.clone(), max_size.clone())
    }
    println!("now scoring..");
    for file in &mut files {
        file.score_file();
    }

    println!("sorting..");
    files.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut preview_files: Vec<File> = Vec::new();
    // preview_files.append(files[0]);
    let mut index = 0;
    let number_to_preview = 200;
    while preview_files.len() < number_to_preview && index < files.len() {
        let file = &files[index];
        let file_path = Path::new(&file.path);
        // if file_path.is_dir() {
        // index += 1;
        // continue;
        // }
        preview_files.push(file.clone());
        // println!("index {}", index);
        index += 1;
    }
    print_score_guide();
    for file in preview_files {
        file.preview_score();
    }
}

fn load_files(mut space_parced: u64, working_dir: String) -> Vec<File> {
    let mut files: Vec<File> = Vec::new();
    let start_path = Path::new(&working_dir);
    for file_result in WalkDir::new(start_path) {
        if let Ok(ref file_result1) = file_result {
            let file = file_result.unwrap();
            let path = file.path();
            let filename = file.path().to_str().unwrap();

            let file_real_size_result = file_real_size(path);

            if file_real_size_result.is_err() {
                continue;
            }
            let filesize = file_real_size_result.unwrap();
            // println!("File size for {}: {}", filename, filesize);
            let last_accessed_result = fs::metadata(path);
            if last_accessed_result.is_err() {
                continue;
            }
            let last_accessed = last_accessed_result.unwrap().modified().unwrap();
            space_parced += filesize;
            // println!("file: {}", filename);
            files.push(File::new(
                filename,
                path.to_str().unwrap_or_default(),
                filesize,
                last_accessed,
            ));
        }
    }
    files
}

fn find_target_working_directory() -> String {
    let current_directory = env::current_dir().unwrap();

    let args: Vec<String> = env::args().collect();
    let working_dir: String;
    if args.len() <= 1 {
        working_dir = current_directory.to_str().unwrap().to_owned();
    } else {
        working_dir = args[1].to_string();
    }
    working_dir
}
