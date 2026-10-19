//! Writing a bundle or one skill into a consumer's tree (`docs/boundary.md`, 「Placing」).
//!
//! A placement is built in a temporary directory beside its target and swapped in, so a reader never
//! sees half of one and a placement that fails leaves the target as it was. A target is replaced only
//! where it is absent, empty, or an earlier placement of the same thing.

use crate::Fault;
use crate::skill::{Bundle, SKILL_FILE, Skill, named_skill, read_skills};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// The plugin manifest's place, in the repository and in Claude Code's standard plugin layout alike.
const MANIFEST: &str = ".claude-plugin/plugin.json";
/// Where the standard plugin layout keeps the plugin's skills.
const PLUGIN_SKILLS: &str = "skills";

/// What a placement wrote.
#[derive(Debug)]
pub struct Placed {
  pub skills: usize,
  pub path: PathBuf,
}

/// Writes the plugin of `bundle` into `into`: the manifest, and every skill of that bundle under
/// `skills/<name>/`.
pub fn place_bundle(root: &Path, bundle: Bundle, into: &Path) -> Result<Placed, Fault> {
  let skills = read_skills(root)?;
  let manifest = root.join(MANIFEST);
  let plugin = plugin_name(&manifest).ok_or_else(|| Fault::failed(&manifest, "names no plugin"))?;
  let members: Vec<&Skill> = skills.iter().filter(|skill| skill.bundle == bundle).collect();
  let replaceable = |target: &Path| plugin_name(&target.join(MANIFEST)).as_deref() == Some(plugin.as_str());
  check_target(into, replaceable, &format!("a placement of the {plugin} plugin"))?;

  let staging = staging_beside(into)?;
  copy_file(&manifest, &staging.path().join(MANIFEST))?;
  for skill in &members {
    copy_skill(skill, &staging.path().join(PLUGIN_SKILLS).join(&skill.name))?;
  }
  swap_in(staging, into)?;
  Ok(Placed {
    skills: members.len(),
    path: into.to_owned(),
  })
}

/// Writes the one skill `name` at `into/<name>/`; `into` is made where it is absent, and nothing else
/// in it is touched.
pub fn place_skill(root: &Path, name: &str, into: &Path) -> Result<Placed, Fault> {
  let skills = read_skills(root)?;
  let skill = skills
    .iter()
    .find(|skill| skill.name == name)
    .ok_or_else(|| Fault::refused(root, format!("holds no skill named {name:?}")))?;
  let target = into.join(&skill.name);
  let replaceable = |target: &Path| named_skill(&target.join(SKILL_FILE)).as_deref() == Some(name);
  check_target(&target, replaceable, &format!("a placement of the skill {name}"))?;

  fs::create_dir_all(into).map_err(|why| Fault::failed(into, why))?;
  let staging = staging_beside(&target)?;
  copy_skill(skill, staging.path())?;
  swap_in(staging, &target)?;
  Ok(Placed {
    skills: 1,
    path: target,
  })
}

fn plugin_name(manifest: &Path) -> Option<String> {
  let text = fs::read_to_string(manifest).ok()?;
  let value: serde_json::Value = serde_json::from_str(&text).ok()?;
  value.get("name")?.as_str().map(str::to_owned)
}

/// Refuses a target that is there and is neither an empty directory nor what `replaceable` knows as an
/// earlier placement.
fn check_target(target: &Path, replaceable: impl Fn(&Path) -> bool, what: &str) -> Result<(), Fault> {
  let Ok(meta) = target.symlink_metadata() else {
    return Ok(());
  };
  if !meta.is_dir() {
    return Err(Fault::refused(target, "is there and is not a directory; nothing was written"));
  }
  let empty = fs::read_dir(target).map_err(|why| Fault::failed(target, why))?.next().is_none();
  if empty || replaceable(target) {
    return Ok(());
  }
  Err(Fault::refused(
    target,
    format!("holds something other than {what}; nothing was written"),
  ))
}

fn staging_beside(target: &Path) -> Result<TempDir, Fault> {
  let parent = parent_of(target);
  fs::create_dir_all(parent).map_err(|why| Fault::failed(parent, why))?;
  let staging = tempfile::Builder::new()
    .prefix(".bmo-skills-placing-")
    .tempdir_in(parent)
    .map_err(|why| Fault::failed(parent, why))?;
  // A temporary directory is made private; the placement takes a directory's ordinary mode.
  fs::set_permissions(staging.path(), fs::Permissions::from_mode(0o755))
    .map_err(|why| Fault::failed(staging.path(), why))?;
  Ok(staging)
}

/// Puts the built placement where `target` is. An earlier one is moved aside first and removed only once
/// the new one stands; where the swap fails, the earlier one is moved back. A temporary directory that
/// has been renamed into place is let go of, so that dropping it removes nothing.
fn swap_in(staging: TempDir, target: &Path) -> Result<(), Fault> {
  if target.symlink_metadata().is_err() {
    fs::rename(staging.path(), target).map_err(|why| Fault::failed(target, why))?;
    let _ = staging.keep();
    return Ok(());
  }
  let aside = tempfile::Builder::new()
    .prefix(".bmo-skills-replaced-")
    .tempdir_in(parent_of(target))
    .map_err(|why| Fault::failed(target, why))?;
  fs::rename(target, aside.path()).map_err(|why| Fault::failed(target, why))?;
  if let Err(why) = fs::rename(staging.path(), target) {
    if fs::rename(aside.path(), target).is_ok() {
      let _ = aside.keep();
    }
    return Err(Fault::failed(target, why));
  }
  let _ = staging.keep();
  aside
    .close()
    .map_err(|why| Fault::failed(target, format!("the earlier placement was not removed: {why}")))
}

fn copy_skill(skill: &Skill, into: &Path) -> Result<(), Fault> {
  for file in &skill.files {
    copy_file(&skill.dir.join(file), &into.join(file))?;
  }
  Ok(())
}

fn copy_file(from: &Path, to: &Path) -> Result<(), Fault> {
  if let Some(dir) = to.parent() {
    fs::create_dir_all(dir).map_err(|why| Fault::failed(dir, why))?;
  }
  fs::copy(from, to).map_err(|why| Fault::failed(from, why))?;
  Ok(())
}

fn parent_of(path: &Path) -> &Path {
  match path.parent() {
    Some(parent) if !parent.as_os_str().is_empty() => parent,
    _ => Path::new("."),
  }
}
