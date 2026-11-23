//! The `bmo-skills` binary as a consumer's deployment runs it (`docs/boundary.md`, 「The `bmo-skills`
//! binary」): what each verb writes, what it refuses and how it exits.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use tempfile::TempDir;

/// This checkout, whose skills the real placements read.
fn repository() -> PathBuf {
  PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn run(from: &Path, words: &[&str]) -> Output {
  Command::new(env!("CARGO_BIN_EXE_bmo-skills"))
    .args(words)
    .arg("--from")
    .arg(from)
    .output()
    .expect("the binary runs")
}

fn exit(output: &Output) -> i32 {
  output.status.code().expect("the binary exits rather than being killed")
}

fn stderr(output: &Output) -> String {
  String::from_utf8_lossy(&output.stderr).into_owned()
}

/// Every file under `dir`, as paths relative to it.
fn files_under(dir: &Path) -> BTreeSet<PathBuf> {
  let mut found = BTreeSet::new();
  let mut pending = vec![dir.to_owned()];
  while let Some(next) = pending.pop() {
    for entry in fs::read_dir(&next).expect("the directory reads") {
      let path = entry.expect("the entry reads").path();
      if path.is_dir() {
        pending.push(path);
      } else {
        found.insert(path.strip_prefix(dir).expect("under the directory").to_owned());
      }
    }
  }
  found
}

fn names_in(dir: &Path) -> BTreeSet<String> {
  fs::read_dir(dir)
    .expect("the directory reads")
    .map(|entry| entry.expect("the entry reads").file_name().to_string_lossy().into_owned())
    .collect()
}

/// What `list --json` answers for this checkout.
fn listed() -> Vec<serde_json::Value> {
  let output = run(&repository(), &["list", "--json"]);
  assert_eq!(exit(&output), 0, "{}", stderr(&output));
  serde_json::from_slice(&output.stdout).expect("the answer is JSON")
}

/// The names of the skills `list` gives `bundle`.
fn listed_in(bundle: &str) -> BTreeSet<String> {
  listed()
    .iter()
    .filter(|skill| skill["bundle"] == bundle)
    .map(|skill| skill["name"].as_str().expect("a name").to_owned())
    .collect()
}

/// A root holding the manifest and one skill folder whose `SKILL.md` front matter is `front`.
fn root_with_skill(folder: &str, front: &str) -> TempDir {
  let root = TempDir::new().expect("a temporary root");
  fs::create_dir_all(root.path().join(".claude-plugin")).expect("the manifest folder");
  fs::write(root.path().join(".claude-plugin/plugin.json"), r#"{ "name": "sample-plugin" }"#)
    .expect("manifest");
  fs::create_dir_all(root.path().join(folder)).expect("the skill folder");
  fs::write(root.path().join(folder).join("SKILL.md"), format!("---\n{front}\n---\n\nThe rules.\n"))
    .expect("skill");
  root
}

#[test]
fn the_worker_placement_holds_the_manifest_and_every_worker_skill_byte_for_byte() {
  let consumer = TempDir::new().expect("a consumer tree");
  let plugin = consumer.path().join("plugin");
  let output = run(&repository(), &["place", "--bundle", "worker", "--into", plugin.to_str().unwrap()]);
  assert_eq!(exit(&output), 0, "{}", stderr(&output));

  assert_eq!(names_in(&plugin), BTreeSet::from([".claude-plugin".to_owned(), "skills".to_owned()]));
  let placed = names_in(&plugin.join("skills"));
  assert!(!placed.is_empty());
  assert_eq!(placed, listed_in("worker"), "every worker skill and no other");
  assert!(placed.is_disjoint(&listed_in("project")), "a project skill is not part of the plugin");
  assert_eq!(
    fs::read(plugin.join(".claude-plugin/plugin.json")).unwrap(),
    fs::read(repository().join(".claude-plugin/plugin.json")).unwrap()
  );
  for skill in &placed {
    let source = repository().join(skill);
    let copy = plugin.join("skills").join(skill);
    assert_eq!(files_under(&copy), files_under(&source), "{skill}");
    for file in files_under(&source) {
      assert_eq!(
        fs::read(copy.join(&file)).unwrap(),
        fs::read(source.join(&file)).unwrap(),
        "{skill}/{}",
        file.display()
      );
    }
  }
  assert_eq!(
    names_in(consumer.path()),
    BTreeSet::from(["plugin".to_owned()]),
    "nothing is left beside it"
  );
}

#[test]
fn a_skill_is_placed_in_its_own_folder_and_nothing_beside_it_is_touched() {
  let consumer = TempDir::new().expect("a consumer tree");
  let skills = consumer.path().join(".claude/skills");
  fs::create_dir_all(skills.join("its-own")).unwrap();
  fs::write(skills.join("its-own/SKILL.md"), "the consumer's own skill").unwrap();

  let output = run(&repository(), &["place", "--skill", "bmo-asking", "--into", skills.to_str().unwrap()]);
  assert_eq!(exit(&output), 0, "{}", stderr(&output));
  assert_eq!(
    fs::read(skills.join("bmo-asking/SKILL.md")).unwrap(),
    fs::read(repository().join("bmo-asking/SKILL.md")).unwrap()
  );
  assert_eq!(fs::read_to_string(skills.join("its-own/SKILL.md")).unwrap(), "the consumer's own skill");
  assert_eq!(names_in(&skills), BTreeSet::from(["its-own".to_owned(), "bmo-asking".to_owned()]));
}

#[test]
fn a_directory_holding_anything_else_is_refused_and_left_as_it_was() {
  let consumer = TempDir::new().expect("a consumer tree");
  let foreign = consumer.path().join("plugin");
  fs::create_dir_all(&foreign).unwrap();
  fs::write(foreign.join("notes.txt"), "kept").unwrap();

  let output = run(&repository(), &["place", "--bundle", "worker", "--into", foreign.to_str().unwrap()]);
  assert_eq!(exit(&output), 2, "{}", stderr(&output));
  assert!(stderr(&output).contains("nothing was written"), "{}", stderr(&output));
  assert_eq!(names_in(&foreign), BTreeSet::from(["notes.txt".to_owned()]));
  assert_eq!(fs::read_to_string(foreign.join("notes.txt")).unwrap(), "kept");
  assert_eq!(
    names_in(consumer.path()),
    BTreeSet::from(["plugin".to_owned()]),
    "no staging is left beside it"
  );
}

#[test]
fn an_earlier_placement_is_replaced_whole() {
  let consumer = TempDir::new().expect("a consumer tree");
  let plugin = consumer.path().join("plugin");
  let into = plugin.to_str().unwrap();
  assert_eq!(exit(&run(&repository(), &["place", "--bundle", "worker", "--into", into])), 0);
  fs::create_dir_all(plugin.join("skills/gone-since")).unwrap();
  fs::write(plugin.join("skills/gone-since/SKILL.md"), "a skill the bundle no longer holds").unwrap();

  let output = run(&repository(), &["place", "--bundle", "worker", "--into", into]);
  assert_eq!(exit(&output), 0, "{}", stderr(&output));
  assert_eq!(names_in(&plugin.join("skills")), listed_in("worker"), "the stale skill is gone");
  assert_eq!(
    names_in(consumer.path()),
    BTreeSet::from(["plugin".to_owned()]),
    "the earlier one is removed"
  );
}

#[test]
fn the_list_names_every_skill_with_its_bundle_and_files() {
  let listed = listed();
  let names: Vec<&str> = listed.iter().map(|skill| skill["name"].as_str().unwrap()).collect();
  let mut sorted = names.clone();
  sorted.sort();
  assert_eq!(names, sorted, "in name order");

  let folders: BTreeSet<String> = names_in(&repository())
    .into_iter()
    .filter(|name| repository().join(name).join("SKILL.md").is_file())
    .collect();
  assert_eq!(names.iter().map(|name| name.to_string()).collect::<BTreeSet<_>>(), folders);
  for skill in &listed {
    let name = skill["name"].as_str().unwrap();
    assert!(
      ["worker", "coder", "producer", "project"].contains(&skill["bundle"].as_str().unwrap()),
      "{name}"
    );
    let files: BTreeSet<PathBuf> = skill["files"]
      .as_array()
      .unwrap()
      .iter()
      .map(|file| PathBuf::from(file.as_str().unwrap()))
      .collect();
    assert_eq!(files, files_under(&repository().join(name)), "{name}");
  }
}

#[test]
fn a_skill_whose_name_is_not_its_folder_fails_the_verb_naming_the_folder() {
  let root = root_with_skill("folder-name", "name: other-name\ndescription: d\nbundle: worker");
  let output = run(root.path(), &["list"]);
  assert_eq!(exit(&output), 1);
  assert!(stderr(&output).contains("folder-name"), "{}", stderr(&output));
}

#[test]
fn a_skill_with_no_bundle_or_an_unknown_one_fails_the_verb() {
  for front in ["name: lone\ndescription: d", "name: lone\ndescription: d\nbundle: everyone"] {
    let root = root_with_skill("lone", front);
    let into = root.path().join("placed");
    let output = run(root.path(), &["place", "--bundle", "worker", "--into", into.to_str().unwrap()]);
    assert_eq!(exit(&output), 1, "{front}");
    assert!(stderr(&output).contains("bundle"), "{}", stderr(&output));
    assert!(!into.exists(), "nothing is written");
  }
}

#[test]
fn a_skill_name_with_no_folder_is_refused() {
  let consumer = TempDir::new().expect("a consumer tree");
  let output = run(
    &repository(),
    &["place", "--skill", "no-such-skill", "--into", consumer.path().to_str().unwrap()],
  );
  assert_eq!(exit(&output), 2);
  assert!(names_in(consumer.path()).is_empty());
}
