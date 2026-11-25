//! The shape `docs/format.md` holds every skill to, beyond what the placer reads: each skill's name, its
//! folder and its bundle are read whole by the placer's own checks on this checkout (`placer.rs`).

use crate::support;

use regex::Regex;
use std::collections::BTreeSet;
use std::path::Path;
use support::{ROOT, files_under, front_matter, holds, read, skill, skill_names};

/// The categories a skill's name takes after its scope (`docs/format.md`, 「Names」).
const CATEGORIES: [&str; 6] = ["code", "ux", "docs", "tools", "video", "research"];

#[test]
fn every_skill_name_opens_with_its_scope_then_its_category() {
  for name in skill_names() {
    let words: Vec<&str> = name.split('-').collect();
    let opens = match words.as_slice() {
      ["ahas", category, _, ..] => CATEGORIES.contains(category),
      ["bmo", _, ..] => true,
      _ => false,
    };
    assert!(
      opens,
      "{name} opens with neither `bmo-` nor `ahas-` and one of the categories {CATEGORIES:?}"
    );
  }
}

#[test]
fn every_skill_says_when_it_is_read() {
  for name in skill_names() {
    let description = front_matter(&name).description.unwrap_or_default();
    assert!(!description.trim().is_empty(), "{name} has no description");
  }
}

#[test]
fn and_they_are_carried_by_a_plugin_the_launch_line_can_name() {
  let manifest: serde_json::Value =
    serde_json::from_str(&read(".claude-plugin/plugin.json")).expect("the manifest is JSON");
  assert_eq!(manifest["name"], "bmo-worker");
}

#[test]
fn a_skill_copied_from_elsewhere_keeps_its_origin_and_licence_beside_it() {
  let license = front_matter("bmo-docs-eli5").license.expect("eli5 names its licence");
  assert!(license.starts_with("MIT"), "{license}");
  assert!(license.contains("anthropics/claude-plugins-community"), "{license}");
}

#[test]
fn every_file_beside_a_skill_is_named_by_it_and_every_file_it_names_is_there() {
  let named = Regex::new(r"\b(?:references|skill)/[A-Za-z0-9_./-]+\.md\b").expect("a path");
  for name in skill_names() {
    let dir = Path::new(ROOT).join(&name);
    let text = skill(&name);
    for file in files_under(&dir) {
      let relative =
        file.strip_prefix(&dir).expect("under the folder").to_string_lossy().replace('\\', "/");
      if relative != "SKILL.md" {
        assert!(text.contains(&relative), "{name}/SKILL.md never names {relative}");
      }
    }
    for path in named.find_iter(&text) {
      assert!(
        dir.join(path.as_str()).is_file(),
        "{name}/SKILL.md names {} that is not there",
        path.as_str()
      );
    }
  }
}

#[test]
fn every_skill_a_skill_names_is_there() {
  let named =
    Regex::new(r"`([a-z0-9]+(?:-[a-z0-9]+)*)`(?: skill| 「)|bmo-worker:([a-z0-9-]+)").expect("a name");
  let skills: BTreeSet<String> = skill_names().into_iter().collect();
  for name in &skills {
    for found in named.captures_iter(&skill(name)) {
      let other = found.get(1).or(found.get(2)).expect("a name").as_str();
      assert!(skills.contains(other), "{name} names the skill {other}, which is not here");
    }
  }
}

#[test]
fn the_design_rules_keep_their_core_under_500_lines_and_name_every_area_file_beside_it() {
  let core = skill("ahas-ux-design-rules");
  assert!(core.lines().count() < 500, "the core is {} lines", core.lines().count());
  holds("ahas-ux-design-rules", &core, &["## The rules by area"]);

  let entry = Regex::new(r"(?m)^- `([a-z-]+\.md)` — ").expect("an area entry");
  let entries: BTreeSet<String> = entry.captures_iter(&core).map(|found| found[1].to_owned()).collect();
  let areas: BTreeSet<String> = files_under(&Path::new(ROOT).join("ahas-ux-design-rules"))
    .iter()
    .map(|path| path.file_name().expect("a file name").to_string_lossy().into_owned())
    .filter(|file| file.ends_with(".md") && file != "SKILL.md")
    .collect();
  assert!(!areas.is_empty());
  assert_eq!(entries, areas, "the core names exactly the area files beside it");
  for area in &areas {
    holds(
      area,
      &read(&format!("ahas-ux-design-rules/{area}")),
      &["Part of the `ahas-ux-design-rules` skill"],
    );
  }
}
