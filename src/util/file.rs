use std::fs::{self, File, OpenOptions};
use std::io::prelude::*;
use std::io::Result;
use std::path::Path;

// 获取某个文件的内容，若文件为空，则创建
pub fn get_content(filename: &str) -> Result<String> {
    let filename = handle_path(filename);
    let mut file = match OpenOptions::new().read(true).open(&filename) {
        Err(e) => {
            // 文件不存在,则创建
            if e.kind() == std::io::ErrorKind::NotFound {
                create_file(&filename)?;
                return Ok(String::from(""));
            }
            return Err(e);
        }
        Ok(f) => f,
    };

    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

// 将要写的内容添加到文件中，以覆盖的方式
pub fn save(filename: &str, content: &str) -> Result<String> {
    let filename = handle_path(filename);
    create_file(&filename)?
        .write_all(content.as_bytes())
        .map(|_| "write success".to_string())
}

// 将要写的内容添加到文件中，以追加的方式
pub fn append(filename: &str, content: &str) -> Result<String> {
    let filename = handle_path(filename);
    open_file(&filename, true)?
        .write_all(content.as_bytes())
        .map(|_| "write success".to_string())
}

// 创建文件，文件包括目录也可以创建
pub fn create_file(filename: &str) -> Result<File> {
    let filename = handle_path(filename);
    let path = Path::new(&filename);
    // 创建parent
    path.parent().map(|parent| fs::create_dir_all(parent));

    // parent创建失败时，此处直接失败
    File::create(filename)
}

// 打开文件，若不存在该文件，则创建,且文件为append类型的
pub fn open_file(filename: &str, append: bool) -> Result<File> {
    let filename = handle_path(filename);
    let path = Path::new(&filename);
    let res = OpenOptions::new().read(true).append(append).open(path);
    if res.is_ok() {
        return res;
    }
    // 创建parent
    path.parent().map(|parent| fs::create_dir_all(parent));

    // parent创建失败时，此处直接失败
    File::create(filename)
}

// 将path中的波浪线替换为 home
fn handle_path(filename: &str) -> String {
    if !filename.starts_with("~/") {
        // 获取home目录
        return String::from(filename);
    }
    let home = std::env::var("HOME").unwrap();
    return filename.replacen("~", &home, 1);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_get_content() {
        let content = get_content("~/.bashrc");
        println!("content is :{:?}", content);
    }

    #[test]
    pub fn test_save_content() {
        println!("{:?}", save("tmpfile/qxg", "gagha"));
    }

    #[test]
    pub fn test_append_content() {
        println!("{:?}", append("tmpfile/qxg", "gagaga"));
    }

    #[test]
    pub fn test_path() {
        use std::path::Path;
        let path = Path::new("tmpfile/qxg");
        println!("path parent:{:?}", path.parent().unwrap());
    }

    #[test]
    pub fn test_create_file() {
        let res = create_file("tmpfile/qxg");
        println!("path parent:{:?}", res);
    }
}
