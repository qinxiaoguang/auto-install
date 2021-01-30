use async_std::fs::{self, File, OpenOptions};
use async_std::io::prelude::*;
use async_std::io::Result;
use std::path::Path;

// 获取某个文件的内容，若文件为空，则创建
pub async fn get_content(filename: &str) -> Result<String> {
    let filename = handle_path(filename);
    let mut file = match OpenOptions::new().read(true).open(&filename).await {
        Err(e) => {
            // 文件不存在,则创建
            if e.kind() == std::io::ErrorKind::NotFound {
                create_file(&filename).await?;
                return Ok(String::from(""));
            }
            return Err(e);
        }
        Ok(f) => f,
    };

    let mut content = String::new();
    file.read_to_string(&mut content).await?;
    Ok(content)
}

// 将要写的内容添加到文件中，以覆盖的方式
pub async fn save(filename: &str, content: &str) -> Result<String> {
    let filename = handle_path(filename);
    create_file(&filename)
        .await?
        .write_all(content.as_bytes())
        .await
        .map(|_| "write success".to_string())
}

// 将要写的内容添加到文件中，以追加的方式
pub async fn append(filename: &str, content: &str) -> Result<String> {
    let filename = handle_path(filename);
    open_file(&filename, true)
        .await?
        .write_all(content.as_bytes())
        .await
        .map(|_| "write success".to_string())
}

// 创建文件，文件包括目录也可以创建
pub async fn create_file(filename: &str) -> Result<File> {
    let filename = handle_path(filename);
    let path = Path::new(&filename);
    // 创建parent
    path.parent().map(|parent| fs::create_dir_all(parent));

    // parent创建失败时，此处直接失败
    File::create(filename).await
}

// 打开文件，若不存在该文件，则创建,且文件为append类型的
pub async fn open_file(filename: &str, append: bool) -> Result<File> {
    let filename = handle_path(filename);
    let path = Path::new(&filename);
    let res = OpenOptions::new()
        .read(true)
        .append(append)
        .open(path)
        .await;
    if res.is_ok() {
        return res;
    }
    // 创建parent
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await;
    }

    // parent创建失败时，此处直接失败
    File::create(filename).await
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
    #[async_std::test]
    pub async fn test_get_content() {
        let content = get_content("~/.bashrc").await;
        println!("content is :{:?}", content);
    }

    #[async_std::test]
    pub async fn test_save_content() {
        println!("{:?}", save("tmpfile/qxg", "async_gagha\n").await);
    }

    #[async_std::test]
    pub async fn test_append_content() {
        println!("{:?}", append("tmpfile/qxg", "async_gagaga\n").await);
    }

    #[async_std::test]
    pub async fn test_path() {
        use std::path::Path;
        let path = Path::new("tmpfile/qxg");
        println!("path parent:{:?}", path.parent().unwrap());
    }

    #[async_std::test]
    pub async fn test_create_file() {
        let res = create_file("tmpfile/qxg").await;
        println!("path parent:{:?}", res);
    }
}
