use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("用法: {} <输入目录> <输出目录> <字符串>", args[0]);
        std::process::exit(1);
    }

    let input_dir = &args[1];
    let output_dir = &args[2];
    let prefix_string = &args[3];

    // 检查输入目录是否存在
    if !Path::new(input_dir).exists() {
        eprintln!("输入目录不存在");
        std::process::exit(1);
    }

    // 递归复制目录结构并创建对应的strm文件
    copy_directory_structure(input_dir, output_dir, prefix_string)?;
    println!("目录结构复制完成");
    
    // 调用函数遍历文件夹并输出文件的相对路径，带前缀字符串，并写入对应的strm文件
    list_files_in_folder(input_dir, output_dir, prefix_string)?;
    
    Ok(())
}

fn copy_directory_structure(input_dir: &str, output_dir: &str, prefix_string: &str) -> io::Result<()> {
    let entries = fs::read_dir(input_dir)?;

    for entry in entries {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_dir() {
            // 如果是目录，则递归复制
            let new_output_dir = format!(
                "{}/{}",
                output_dir,
                entry_path.file_name().unwrap().to_string_lossy()
            );
            fs::create_dir_all(&new_output_dir)?;

            copy_directory_structure(&entry_path.to_string_lossy(), &new_output_dir, prefix_string)?;
        } else {
            // 如果是文件，则创建相应的strm文件并写入内容
            //let new_file_name = format!(
                //"{}/{}.strm",
                //output_dir,
                //entry_path.file_stem().unwrap().to_string_lossy()
            //);
            //fs::write(&new_file_name, prefix_string)?;
        }
    }

    Ok(())
}

fn list_files_in_folder(input_dir: &str, output_dir: &str, prefix_string: &str) -> io::Result<()> {
    let folder = Path::new(input_dir).canonicalize()?;
    list_files_recursively(&folder, &folder, output_dir, prefix_string)?;
    Ok(())
}

fn list_files_recursively(folder: &Path, base_folder: &Path, output_dir: &str, prefix_string: &str) -> io::Result<()> {
    if let Ok(entries) = fs::read_dir(folder) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    // 如果是文件夹，则递归遍历
                    list_files_recursively(&path, base_folder, output_dir, prefix_string)?;
                } else {
                    // 如果是文件，则输出相对路径带前缀字符串，并写入对应的strm文件
                    match path.strip_prefix(base_folder) {
                        Ok(relative_path) => {
                            let strm_file_name = format!(
                                "{}/{}.strm",
                                output_dir,
                                relative_path.with_extension("").to_string_lossy()
                            );
                            fs::write(&strm_file_name, format!("{}{}", prefix_string, relative_path.display()))?;
                        }
                        Err(_) => eprintln!("Error stripping prefix from path: {:?}", path),
                    }
                }
            }
        }
    }
    Ok(())
}
