use crate::file;

pub fn get_diff_files(blob1_hash: &String, blob2_hash: &String) -> String {
    match file::cat_file(String::from(blob1_hash)) {
        Err(_) => String::from(""),
        Ok(blob1) => match file::cat_file(String::from(blob2_hash)) {
            Err(_) => String::from(""),
            Ok(blob2) => {
                let blob1_lines: Vec<&str> = blob1.split('\n').collect();
                let blob2_lines: Vec<&str> = blob2.split('\n').collect();
                let mut new_file: String = String::from("");
                let mut cnt = 0;
                for line in blob1_lines.clone() {
                    if cnt >= blob2_lines.len() {
                        new_file.push_str(&(String::from(line) + "\n"));
                    } else {
                        if line == blob2_lines[cnt] {
                        } else {
                            new_file.push_str(&(String::from(blob1_lines[cnt]) + "\n"));
                            new_file.push_str(&(String::from(blob2_lines[cnt]) + "\n"));
                        }
                    }
                    cnt += 1;
                }
                new_file
            }
        },
    }
}
