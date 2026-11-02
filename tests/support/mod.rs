//! What the checks share: the root, the skill folders, and a skill's text as its readers take it.

use gray_matter::Matter;
use gray_matter::engine::YAML;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

pub const ROOT: &str = env!("CARGO_MANIFEST_DIR");

/// Directories no check reads: build output and git's own.
const SKIPPED: [&str; 2] = ["target", ".git"];

/// A file of the repository, read as text.
pub fn read(relative: &str) -> String {
  fs::read_to_string(Path::new(ROOT).join(relative))
    .unwrap_or_else(|error| panic!("{relative} reads: {error}"))
}

/// A skill's `SKILL.md` as it stands.
pub fn skill(name: &str) -> String {
  read(&format!("{name}/SKILL.md"))
}

/// A skill's `SKILL.md` laid flat: a skill is wrapped markdown, and a rule spanning a wrap is still one
/// sentence.
pub fn skill_flat(name: &str) -> String {
  skill(name).split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The name of every skill folder at the root: every folder holding a `SKILL.md`, in name order.
pub fn skill_names() -> Vec<String> {
  let mut names: Vec<String> = fs::read_dir(ROOT)
    .expect("the root reads")
    .map(|entry| entry.expect("an entry reads").path())
    .filter(|path| path.join("SKILL.md").is_file())
    .map(|path| path.file_name().expect("a folder name").to_string_lossy().into_owned())
    .collect();
  names.sort();
  names
}

/// The front matter keys a check reads.
#[derive(Deserialize)]
pub struct FrontMatter {
  pub description: Option<String>,
  pub license: Option<String>,
}

/// A skill's front matter, read the way the placer reads it.
pub fn front_matter(name: &str) -> FrontMatter {
  Matter::<YAML>::new()
    .parse::<FrontMatter>(&skill(name))
    .unwrap_or_else(|error| panic!("{name}: the front matter reads: {error}"))
    .data
    .unwrap_or_else(|| panic!("{name}: SKILL.md has front matter"))
}

/// Every file under `dir`, at any depth, skipping build output and git's directory.
pub fn files_under(dir: &Path) -> Vec<PathBuf> {
  let mut files = Vec::new();
  if dir.is_dir() {
    collect(dir, &mut files);
  }
  files.sort();
  files
}

fn collect(dir: &Path, files: &mut Vec<PathBuf>) {
  for entry in fs::read_dir(dir).expect("a directory reads") {
    let entry = entry.expect("an entry reads");
    let kind = entry.file_type().expect("an entry's type reads");
    if kind.is_dir() {
      if !SKIPPED.contains(&entry.file_name().to_string_lossy().as_ref()) {
        collect(&entry.path(), files);
      }
    } else if kind.is_file() {
      files.push(entry.path());
    }
  }
}

/// A path as it reads relative to the root, with forward slashes, for a failure's message.
pub fn shown(path: &Path) -> String {
  let relative = path.strip_prefix(ROOT).unwrap_or(path);
  relative
    .components()
    .map(|part| part.as_os_str().to_string_lossy())
    .collect::<Vec<_>>()
    .join("/")
}

/// Asserts that `text` holds every one of `expected`, naming the skill and the missing passage.
pub fn holds(skill: &str, text: &str, expected: &[&str]) {
  for passage in expected {
    assert!(text.contains(passage), "{skill} no longer says: {passage}");
  }
}

/// Asserts that `text` holds none of `refused`.
pub fn lacks(skill: &str, text: &str, refused: &[&str]) {
  for passage in refused {
    assert!(!text.contains(passage), "{skill} says again: {passage}");
  }
}
