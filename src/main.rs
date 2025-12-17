use std::fs;
use walkdir::WalkDir;

fn main() {
    let folder = "/home/user/Downloads";

    println!("Sorting files in: {}", folder);

    for entry in WalkDir::new(folder).min_depth(1).max_depth(1) {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            let ext = path
                .extension()
                .unwrap_or_default()
                .to_str()
                .unwrap()
                .to_lowercase();

            let target = match ext.as_str() {
                "jpg" | "jpeg" | "png" | "gif" => "Images",
                "pdf" | "docx" | "txt"        => "Documents",
                "zip" | "tar" | "gz"          => "Archives",
                "mp3" | "wav"                 => "Music",
                "mp4" | "mov"                 => "Videos",
                "rs" | "py" | "js"            => "Code",
                _ => "Other"
            };

            let new_dir = format!("{}/{}", folder, target);
            fs::create_dir_all(&new_dir).unwrap();

            let file_name = path.file_name().unwrap().to_str().unwrap();
            let new_path = format!("{}/{}", new_dir, file_name);

            println!("Moving: {} → {}", file_name, target);
            fs::rename(path, new_path).unwrap();
        }
    }

    println!("Done!");
}
