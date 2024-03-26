use std::io::Write;
pub fn copy(source_path: std::path::PathBuf, destination_path: std::path::PathBuf) {
    if destination_path.is_file() {
        println!("Given path is not a directory");
    }
    if destination_path.to_str().unwrap().contains("..")
        || source_path.to_str().unwrap().contains("..")
    {
        println!("Please dont use .., this program would break lol");
        std::process::exit(0);
    }
    scaffold_directory(&source_path, &destination_path);
    std::io::stdout().flush().unwrap();
    println!("Copied {:?} to {:?}", source_path, destination_path);
}

fn scaffold_directory(source_path: &std::path::PathBuf, destination_path: &std::path::PathBuf) {
    std::fs::create_dir_all(destination_path.join(source_path)).unwrap();
    let paths = std::fs::read_dir(&source_path).unwrap();
    for p in paths {
        if p.as_ref().unwrap().path().is_dir() {
            std::fs::create_dir_all(destination_path.join(&source_path)).unwrap();

            scaffold_directory(&p.unwrap().path(), destination_path);
        } else if !p.as_ref().unwrap().file_name().is_empty() {
            std::fs::copy(
                p.as_ref().unwrap().path(),
                destination_path.join(&p.unwrap().path()),
            )
            .unwrap();
        }
    }
}
