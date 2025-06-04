use std::{
    alloc::System,
    env::{self, Args},
    fs,
    path::{self, Path},
    string,
    time::{SystemTime, UNIX_EPOCH},
};

use filesize::{PathExt, file_real_size};
use walkdir::WalkDir;

// Test
struct File {
    name: String,
    filesize: u64,
    last_accessed: std::time::SystemTime,
    path: String,
    score: u64,
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
            score: 0,
        }
    }

    fn score_file(&mut self) {
        let now = SystemTime::now();

        let since_result = now.duration_since(UNIX_EPOCH);
        if since_result.is_err() {
            return;
        }
        let duration = since_result.unwrap().as_secs() / 86_400;
        println!("duration {} * filsize {}", duration, self.filesize);
        self.score = self.filesize * duration;

        // todo!()
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
}

fn main() {
    let mut files: Vec<File> = Vec::new();
    let mut space_parced: u64 = 0;

    let args: Vec<String> = env::args().collect();
    let working_dir: String;
    if args.len() <= 1 {
        working_dir = "/".to_string();
    } else {
        working_dir = args[1].to_string();
    }

    let start_path = Path::new(&working_dir);
    for file_result in WalkDir::new(start_path) {
        if let Ok(ref file_result1) = file_result {
            let file = file_result.unwrap();
            let path = file.path();
            let filename = file.file_name().to_str().unwrap();
            let file_real_size_result = file_real_size(path);
            if file_real_size_result.is_err() {
                continue;
            }
            let filesize = file_real_size_result.unwrap();
            let last_accessed_result = fs::metadata(path);
            if last_accessed_result.is_err() {
                continue;
            }
            let last_accessed = last_accessed_result.unwrap().accessed().unwrap();
            space_parced += filesize;
            println!("file: {}", filename);
            files.push(File::new(
                filename,
                path.to_str().unwrap_or_default(),
                filesize,
                last_accessed,
            ));
        }
    }

    println!("now scoring..");
    for file in &mut files {
        // file.print();
        file.score_file();
    }
    files.sort_by(|a, b| b.score.cmp(&a.score));
    for file in files.iter().take(10) {
        file.print();
    }
}
