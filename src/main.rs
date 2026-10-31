use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use clap::Parser;
use ignore::Walk;

mod ext;

#[derive(Parser)]
struct Args {
    paths: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let mut queue = args.paths;
    let mut count = 0;
    while let Some(path) = queue.pop() {
        count += 1;
        if let Err(e) = scan(&path, &mut queue) {
            eprintln!("\r{path:?}: {e:?}");
        } else {
            eprint!("\rchecked: {count}");
        }
    }
}

fn scan(path: &Path, queue: &mut Vec<PathBuf>) -> anyhow::Result<()> {
    let meta = fs::symlink_metadata(path)?;
    let ft = meta.file_type();

    if ft.is_file() {
        check_file(path)?;
    } else if ft.is_dir() {
        for ent in Walk::new(path) {
            let ent = ent?;
            if ent.depth() == 0 {
                continue;
            }
            queue.push(ent.into_path());
        }
    }

    Ok(())
}

fn check_file(path: &Path) -> anyhow::Result<()> {
    const HEAD_LEN: usize = 1024;
    let mut head = Vec::with_capacity(HEAD_LEN);

    File::open(path)?
        .take(HEAD_LEN as _)
        .read_to_end(&mut head)?;

    let Some(detected) = infer::get(&head) else {
        eprintln!("\runknown: {path:?}");
        return Ok(());
    };

    let ext = path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default();

    let detected_ext = detected.extension();

    if let Some(false) = ext::is_match(detected_ext, ext) {
        println!("\rmv {:?} {:?}", path, path.with_extension(detected_ext));
    }

    Ok(())
}
