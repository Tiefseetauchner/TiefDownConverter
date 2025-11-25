use color_eyre::eyre::{Result, eyre};
use log::debug;
use std::path::{Path, PathBuf};

use crate::injections::RenderingInjections;

pub(crate) fn get_sorted_files(
    input_dir: &Path,
    project_directory_path: &Path,
    compiled_directory_path: &Path,
    injections: &RenderingInjections,
    multi_file_output: bool,
) -> Result<Vec<PathBuf>> {
    let dir_content = std::fs::read_dir(input_dir)?;

    let mut dir_content = dir_content
        .filter_map(|f| {
            let entry = f.ok()?;

            Some(entry.path())
        })
        .collect::<Vec<_>>();

    dir_content.append(&mut injections.body_injections.clone());

    dir_content.sort_by(|a, b| {
        let a_num = retrieve_file_order_number(a);
        let b_num = retrieve_file_order_number(b);

        match a_num.cmp(&b_num) {
            std::cmp::Ordering::Equal => {
                let a_is_file = a.is_file();
                let b_is_file = b.is_file();
                match (a_is_file, b_is_file) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => std::cmp::Ordering::Equal,
                }
            }
            other => other,
        }
    });

    let input_files = dir_content
        .iter()
        .map(|f| {
            if f.is_file() {
                return Ok(vec![f.clone()]);
            } else if f.is_dir() {
                get_sorted_files(
                    f,
                    project_directory_path,
                    compiled_directory_path,
                    &RenderingInjections::new(),
                    multi_file_output,
                )
            } else {
                Err(eyre!(
                    "Input file '{}' was not found or does not exist.",
                    f.display()
                ))
            }
        })
        .collect::<Vec<_>>();

    let input_files: Vec<PathBuf> = input_files
        .into_iter()
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect();

    let input_files: Vec<PathBuf> = input_files
        .iter()
        .map(|f| {
            get_relative_path_from_compiled_dir(f, project_directory_path, compiled_directory_path)
                .unwrap_or(f.to_path_buf())
        })
        .collect();
    debug!(
        "get_sorted_files('{}') -> {} files",
        input_dir.display(),
        input_files.len()
    );
    Ok(input_files)
}

fn retrieve_file_order_number(p: &Path) -> u32 {
    let file_name_regex = regex::Regex::new(r".*?(\d+).*").unwrap();

    if let Some(order_number) = p
        .file_name()
        .and_then(|name| name.to_str().map(|s| s.to_string()))
        .and_then(|s| file_name_regex.captures(&s).map(|cap| cap[1].to_string()))
        .and_then(|n| match n.parse::<u32>() {
            Result::Ok(n) => Some(n),
            Err(_e) => None,
        })
    {
        return order_number;
    }

    0
}

pub(crate) fn get_relative_path_from_compiled_dir(
    original_path: &Path,
    project_root: &Path,
    compiled_dir: &Path,
) -> Option<PathBuf> {
    let relative_to_project = original_path.strip_prefix(project_root).ok()?;

    let depth = compiled_dir
        .strip_prefix(project_root)
        .ok()?
        .components()
        .count();
    let mut relative_path = PathBuf::new();
    for _ in 0..depth {
        relative_path.push("..");
    }

    relative_path.push(relative_to_project);
    Some(relative_path)
}
