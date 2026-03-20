use std::{
    collections::HashMap,
    fs::{self},
    io,
    path::Path,
};

use clap::Parser;

use crate::args::Args;

mod args;

fn main() -> io::Result<()> {
    let args = Args::parse();
    let mut words: HashMap<String, u64> = HashMap::new();

    visit_dirs(&args.path, &mut words, &args)?;

    let mut words: Vec<(String, u64)> = words.into_iter().collect();
    words.sort_by(|(_, a), (_, b)| b.cmp(a));

    for (word, count) in words {
        println!("{word}: {count}");
    }

    Ok(())
}

fn visit_dirs(dir: &Path, words: &mut HashMap<String, u64>, args: &Args) -> io::Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }

    'entries: for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            visit_dirs(&path, words, args)?;
            continue;
        }

        if let Some(ignore) = &args.ignore {
            for ignore in ignore {
                if path.to_string_lossy().contains(ignore) {
                    continue 'entries;
                }
            }
        }

        let content = fs::read_to_string(path)?;
        parse_content(content, words);
    }
    Ok(())
}

fn parse_content(content: String, words: &mut HashMap<String, u64>) {
    content.split_whitespace().for_each(|word| {
        *words.entry(word.to_string()).or_insert(0) += 1;
    });
}
