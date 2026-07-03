use std::env;
use std::fs;
use std::io::{self, Write};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::process::Command;
use std::fmt::Write as FmtWrite;

fn copy_to_clipboard(text: &str) {
    let (cmd, args, err_msg) = if env::var_os("WAYLAND_DISPLAY").is_some() {
        ("wl-copy", vec![], "wl-clipboard")
    } else {
        ("xclip", vec!["-selection", "clipboard"], "xclip")
    };

    if let Ok(mut child) = Command::new(cmd).args(args).stdin(std::process::Stdio::piped()).spawn() {
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(text.as_bytes());
        }
        let _ = child.wait();
        println!("\n✅ Output copied to clipboard successfully!");
    } else {
        eprintln!("\n❌ Copy failed. Please make sure '{}' is installed on your system.", err_msg);
    }
}

fn is_too_large(path: &Path, hide_hidden: bool, limit: usize) -> bool {
    let mut count = 0;
    let mut dirs = vec![path.to_path_buf()];

    while let Some(current) = dirs.pop() {
        if let Ok(entries) = fs::read_dir(current) {
            for entry in entries.flatten() {
                if hide_hidden && entry.file_name().as_bytes().starts_with(b".") {
                    continue;
                }

                count += 1;
                if count >= limit {
                    return true;
                }

                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        dirs.push(entry.path());
                    }
                }
            }
        }
    }
    false
}

fn build_tree(path: &Path, prefix: &mut String, hide_hidden: bool, output: &mut String) {
    let mut entries: Vec<_> = match fs::read_dir(path) {
        Ok(read_dir) => read_dir
            .filter_map(Result::ok)
            .filter(|e| !hide_hidden || !e.file_name().as_bytes().starts_with(b"."))
            .collect(),
        Err(_) => {
            let _ = write!(output, "{} [Access Denied]\n", prefix);
            return;
        }
    };

    entries.sort_by_key(|a| a.file_name());
    let count = entries.len();

    for (i, entry) in entries.iter().enumerate() {
        let is_last = i == count - 1;
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        
        let branch = if is_last { "└── " } else { "├── " };
        let _ = write!(output, "{}{}{}", prefix, branch, name_str);

        if let Ok(file_type) = entry.file_type() {
            if file_type.is_symlink() {
                if let Ok(target) = fs::read_link(entry.path()) {
                    let _ = write!(output, " -> {}", target.display());
                }
                output.push('\n');
            } else if file_type.is_dir() {
                output.push('\n');
                
                let old_len = prefix.len();
                if is_last {
                    prefix.push_str("    ");
                } else {
                    prefix.push_str("│   ");
                }
                
                build_tree(&entry.path(), prefix, hide_hidden, output);
                prefix.truncate(old_len);
            } else {
                output.push('\n');
            }
        } else {
            output.push('\n');
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let hide_hidden = args.contains(&"-h".to_string());

    println!("========================================================");
    println!("      Tree Generator V1.0.0 - by Neuwj - neuwj@linuxmail.org    ");
    println!("========================================================\n");

    let current_dir = env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));

    if is_too_large(&current_dir, hide_hidden, 500) {
        println!("\nWarning: This directory tree may be very large.");
        print!("Are you sure? (Y/N): ");
        io::stdout().flush().unwrap();

        let mut answer = String::new();
        io::stdin().read_line(&mut answer).unwrap();
        if answer.trim().eq_ignore_ascii_case("y") == false {
            println!("Operation cancelled.");
            return;
        }
    }

    let mut tree_output = String::with_capacity(1024 * 1024);
    let root_dir_name = current_dir
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| current_dir.to_string_lossy().into_owned());

    let _ = write!(tree_output, "{}\n", root_dir_name);
    
    let mut prefix = String::with_capacity(256);
    build_tree(&current_dir, &mut prefix, hide_hidden, &mut tree_output);

    println!("\n{}", tree_output);

    print!("Press 1 and Enter to copy the output to the clipboard, or any other key to exit: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if input.trim() == "1" {
        copy_to_clipboard(&tree_output);
    }
}