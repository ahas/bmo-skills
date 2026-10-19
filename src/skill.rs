//! Reading the skill folders at a repository's root, as `docs/format.md` states them.

use crate::Fault;
use gray_matter::Matter;
use gray_matter::engine::YAML;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// The file every skill folder holds, whose front matter names the skill and its bundle.
pub const SKILL_FILE: &str = "SKILL.md";

/// Which consumers receive a skill, as the skill's own front matter declares it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Bundle {
  /// The plugin every job worker is given.
  Worker,
  /// A project skill one repository's checkout receives into its own skills folder.
  Project,
}

impl Bundle {
  const ALL: [Bundle; 2] = [Bundle::Worker, Bundle::Project];

  pub fn as_str(self) -> &'static str {
    match self {
      Bundle::Worker => "worker",
      Bundle::Project => "project",
    }
  }

  fn named(name: &str) -> Option<Bundle> {
    Bundle::ALL.into_iter().find(|bundle| bundle.as_str() == name)
  }
}

/// One skill folder: its name, its bundle and every file it holds.
#[derive(Clone, Debug, Serialize)]
pub struct Skill {
  pub name: String,
  pub bundle: Bundle,
  /// The folder's file paths relative to it, in order.
  pub files: Vec<PathBuf>,
  #[serde(skip)]
  pub dir: PathBuf,
}

/// The keys of a skill's front matter the placer reads; every other key is the skill's own.
#[derive(Deserialize)]
struct FrontMatter {
  name: Option<String>,
  bundle: Option<String>,
}

/// Every skill folder at the root, in name order. A folder at the root is a skill where it holds a
/// `SKILL.md`; every skill is read whole before any is returned, so a bad one stops the verb.
pub fn read_skills(root: &Path) -> Result<Vec<Skill>, Fault> {
  let entries = fs::read_dir(root).map_err(|why| Fault::failed(root, why))?;
  let mut skills = Vec::new();
  for entry in entries {
    let dir = entry.map_err(|why| Fault::failed(root, why))?.path();
    if dir.is_dir() && dir.join(SKILL_FILE).symlink_metadata().is_ok() {
      skills.push(read_skill(&dir)?);
    }
  }
  skills.sort_by(|a, b| a.name.cmp(&b.name));
  Ok(skills)
}

fn read_skill(dir: &Path) -> Result<Skill, Fault> {
  let folder = dir.file_name().and_then(|name| name.to_str()).unwrap_or_default().to_owned();
  let front = skill_front_matter(&dir.join(SKILL_FILE)).map_err(|why| Fault::failed(dir, why))?;
  let name = front.name.ok_or_else(|| Fault::failed(dir, "SKILL.md names no skill"))?;
  if name != folder {
    return Err(Fault::failed(dir, format!("SKILL.md names {name:?}, not its folder")));
  }
  let named = front.bundle.ok_or_else(|| Fault::failed(dir, "SKILL.md names no bundle"))?;
  let bundle = Bundle::named(&named)
    .ok_or_else(|| Fault::failed(dir, format!("SKILL.md names an unknown bundle {named:?}")))?;
  let mut files = Vec::new();
  list_files(dir, Path::new(""), &mut files)?;
  files.sort();
  Ok(Skill {
    name,
    bundle,
    files,
    dir: dir.to_owned(),
  })
}

/// The name a folder's `SKILL.md` gives, where it reads; used to know an earlier placement of a skill.
pub(crate) fn named_skill(skill_file: &Path) -> Option<String> {
  skill_front_matter(skill_file).ok().and_then(|front| front.name)
}

fn skill_front_matter(skill_file: &Path) -> Result<FrontMatter, String> {
  let text = fs::read_to_string(skill_file).map_err(|why| format!("SKILL.md does not read: {why}"))?;
  let parsed = Matter::<YAML>::new()
    .parse::<FrontMatter>(&text)
    .map_err(|why| format!("SKILL.md's front matter does not read: {why}"))?;
  parsed.data.ok_or_else(|| "SKILL.md has no front matter".to_owned())
}

/// Every file under `dir`, as paths relative to the skill folder. A link is refused: a placement copies
/// what a folder holds byte for byte, and a link's target is not inside the folder.
fn list_files(dir: &Path, relative: &Path, files: &mut Vec<PathBuf>) -> Result<(), Fault> {
  let entries = fs::read_dir(dir).map_err(|why| Fault::failed(dir, why))?;
  for entry in entries {
    let entry = entry.map_err(|why| Fault::failed(dir, why))?;
    let kind = entry.file_type().map_err(|why| Fault::failed(&entry.path(), why))?;
    let path = relative.join(entry.file_name());
    if kind.is_dir() {
      list_files(&entry.path(), &path, files)?;
    } else if kind.is_file() {
      files.push(path);
    } else {
      return Err(Fault::failed(&entry.path(), "is neither a file nor a folder"));
    }
  }
  Ok(())
}
