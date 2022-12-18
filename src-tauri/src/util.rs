use std::path::PathBuf;

/**
Return name.ext if file does not exists
otherwise return name_N.ext
*/
pub fn new_filename(fullpath: &str) -> String {
    let mut path = PathBuf::from(fullpath);
    let mut filename = String::from(path.file_name().unwrap().to_string_lossy());
    let mut n = 1;
    
    while path.exists() {
        let parent = path.parent().unwrap();
        let stem = path.file_stem().unwrap().to_str().unwrap();
        let ext = match path.extension() {
            Some(e) => e.to_str().unwrap(),
            None => "",
        };
        if ext.len() > 0 {
            filename = format!("{stem}_{n}.{ext}"); 
        } else {
            filename = format!("{stem}_{n}"); 
        }
        

        if let Some(pos) = stem.rfind('_') {
            let s = &stem[..pos];
            let number = &stem[(pos+1)..];
            println!("has underscore: {stem}: {s}, {number}");
            if let Ok(_) = number.parse::<i32>() {
                filename = format!("{s}_{n}.{ext}");        
            } else {
                filename = format!("{stem}_{n}.{ext}");        
            }
        }
        println!("{:?},{},{},{}", parent, stem, ext, filename);
        n += 1;
        path = PathBuf::from(parent);
        path.push(&filename);
    }
    filename
}


#[cfg(test)]
mod tests {

    use std::fs::File;

    use super::*;

    #[test]
    fn test_new_filename() {
        let filename = "file.txt";
        let mut fullpath = std::env::current_dir().unwrap();
        fullpath.push(filename);
        let name = new_filename(fullpath.to_str().unwrap());
        assert_eq!(name, filename);

        File::create(filename).unwrap();
        assert!(PathBuf::from(filename).exists());
        let name = new_filename(filename);
        assert_eq!(name, "file_1.txt");

        File::create("file_1.txt").unwrap();
        assert!(PathBuf::from("file_1.txt").exists());
        let name = new_filename(filename);
        assert_eq!(name, "file_2.txt");

        std::fs::remove_file(filename).unwrap();
        std::fs::remove_file("file_1.txt").unwrap();
        assert!(!PathBuf::from(filename).exists());
        assert!(!PathBuf::from("file_1.txt").exists());
        
        let filename = "file_with_underscore.txt";
        File::create(filename).unwrap();
        assert!(PathBuf::from(filename).exists());
        let name = new_filename(filename);
        assert_eq!(name, "file_with_underscore_1.txt");
        std::fs::remove_file(filename).unwrap();
        assert!(!PathBuf::from(filename).exists());

        let filename = "file_3.txt";
        File::create(filename).unwrap();
        assert!(PathBuf::from(filename).exists());
        let name = new_filename(filename);
        assert_eq!(name, "file_1.txt");
        std::fs::remove_file(filename).unwrap();
        assert!(!PathBuf::from(filename).exists());

        let filename = "file";
        File::create(filename).unwrap();
        assert!(PathBuf::from(filename).exists());
        let name = new_filename(filename);
        assert_eq!(name, "file_1");
        std::fs::remove_file(filename).unwrap();
        assert!(!PathBuf::from(filename).exists());

    }

}