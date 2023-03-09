use std::fs::{self, File};
use std::io::{BufReader, BufRead};
use std::io;
use chrono::{Local};
use std::time::{Duration, SystemTime};
use humantime::{format_duration, Duration as HumanDuration};
use colored::*;
use std::env;

pub fn cat(file_name: &str) -> std::io::Result<()> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}

pub fn cd(dir_name: &str) -> std::io::Result<()> {
    env::set_current_dir(dir_name)?;
    Ok(())
}

pub fn date() {
    let now = Local::now();
    println!("{}", now);
}

pub fn echo(args: Vec<String>) {
    if args.is_empty() {
        println!("No command-line arguments passed to echo function");
    } else {
        println!("{}", args.join(" "));
    }
}

pub fn ls() -> io::Result<()> {
    let current_dir = std::env::current_dir()?;
    let entries = fs::read_dir(current_dir)?;

    let mut index: i32 = 0;
    println!("╭─────┬───────────────────┬──────┬───────────┬──────────────────────╮");
    println!("│  {:^2} │ {:^17} │ {:^4} │ {:^9} │ {:^20} │", "#".green(), "name".green(), "type".green(), "size".green(), "modified".green());
    println!("├─────┼───────────────────┼──────┼───────────┼──────────────────────┤");

    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        let metadata = entry.metadata()?;
        let file_type = if metadata.is_file() {
            "file"
        } else if metadata.is_dir() {
            "dir"
        } else {
            "other"
        };
        let file_size = metadata.len();
        let file_size_str = if file_size == 0 {
            "0 B".to_owned()
        } else {
            let units = ["B", "KiB", "MiB", "GiB", "TiB"];
            let digit_groups = (file_size as f64).log(1024.0).floor() as u32;
            let size = file_size as f64 / 1024.0_f64.powi(digit_groups as i32);
            format!("{:.2} {}", size, units[digit_groups as usize])
        };
        let file_modified = metadata.modified()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let time_since_modified = SystemTime::now().duration_since(file_modified)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let duration = Duration::from_secs(time_since_modified.as_secs());
        let modified_str = format_duration(*HumanDuration::from(duration)).to_string().replace(", ", "");

        let colored_file_name = match file_name.to_str() {
            Some(name) if name.ends_with(".png") || name.ends_with(".svg") || name.ends_with(".jpg") || name.ends_with(".jpeg") || name.ends_with(".jfif") || name.ends_with(".mp4") => name.purple(),
            _ => file_name.to_string_lossy().cyan(),
        };

        println!(
            "│  {:^2} │ {:^17} │ {:^4} │ {:^9} │ {:^20} │",
            index.to_string().green(),
            colored_file_name,
            file_type,
            file_size_str,
            modified_str,
        );

        index += 1;
    }
    println!("╰─────┴───────────────────┴──────┴───────────┴──────────────────────╯");

    Ok(())
}

pub fn mv(file_name_one: &str, file_name_two: &str) -> std::io::Result<()> {
    fs::rename(file_name_one, file_name_two)?;
    Ok(())
}

pub fn mkdir(dir_name: &str) -> std::io::Result<()> {
    fs::create_dir_all(dir_name)?;
    Ok(())
}

pub fn rmdir(dir_name: &str) -> std::io::Result<()> {
    fs::remove_dir_all(dir_name)?;
    Ok(())
}

pub fn rm(file_name: &str) -> std::io::Result<()> {
    let mut _file = fs::remove_file(file_name)?;
    Ok(())
}

pub fn printf(args: Vec<String>) {
    if args.is_empty() {
        println!("No command-line arguments passed to echo function");
    } else {
        print!("{}", args.join(" "));
    }
}

pub fn pwd() {
    if let Ok(current_dir) = env::current_dir() {
        println!("{}", current_dir.display());
    } else {
        eprintln!("Error: could not get current working directory.");
    }
}

pub fn touch(file_name: &str) -> std::io::Result<()> {
    let mut _file = File::create(file_name)?;
    Ok(())
}

// Credit to https://github.com/kddnewton/tree
//----------------------------------------------
struct Counts {
  dirs: i32,
  files: i32
}

fn walk(dir: &str, prefix: &str, counts: &mut Counts) -> io::Result<()> {
  let mut paths: Vec<_> = fs::read_dir(dir)?.map(|entry| entry.unwrap().path()).collect();
  let mut index = paths.len();

  paths.sort_by(|a, b| {
    let aname = a.file_name().unwrap().to_str().unwrap();
    let bname = b.file_name().unwrap().to_str().unwrap();
    aname.cmp(bname)
  });

  for path in paths {
    let name = path.file_name().unwrap().to_str().unwrap();
    index -= 1;

    if name.starts_with(".") {
      continue;
    }

    if path.is_dir() {
      counts.dirs += 1;
    } else {
      counts.files += 1;
    }

    if index == 0 {
      println!("{}└── {}", prefix, name);
      if path.is_dir() {
        walk(&format!("{}/{}", dir, name), &format!("{}    ", prefix), counts)?;
      }
    } else {
      println!("{}├── {}", prefix, name);
      if path.is_dir() {
        walk(&format!("{}/{}", dir, name), &format!("{}│   ", prefix), counts)?;
      }
    }
  }

  Ok(())
}

pub fn tree() -> io::Result<()> {
  let dir = env::args().nth(1).unwrap_or(".".to_string());
  println!("{}", dir);

  let mut counts = Counts { dirs: 0, files: 0 };
  walk(&dir, "", &mut counts)?;

  println!("\n{} directories, {} files", counts.dirs, counts.files);

  Ok(())
}
//----------------------------------------------
