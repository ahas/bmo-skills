//! No personal value by its shape in the skills, the manifest or the documents: no chat or account id,
//! email address, IP address, private host, home path, checkout layout or account handle. The check goes
//! by pattern; what it lets through is public by nature, the reserved example names and the public sites,
//! package scopes and code keywords the skills teach, and no list of people's names is kept anywhere.

mod support;

use regex::Regex;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use support::{ROOT, files_under, shown, skill_names};

/// The public sites a skill teaches about, each also allowing its subdomains.
const PUBLIC_SITES: [&str; 16] = [
  "github.com",
  "skills.sh",
  "claude.ai",
  "nuxt.com",
  "icones.js.org",
  "playwright.dev",
  "reka-ui.com",
  "tailwindcss.com",
  "tailwind-variants.org",
  "vercel.com",
  "tanstack.com",
  "app.com",
  "site1.com",
  "site2.com",
  "site3.com",
  "slow-site.com",
];

/// What follows an `@` in code a skill teaches: public package scopes, CSS at-rules, Vue's event
/// shorthand and doc comment tags.
const CODE_AFTER_AT: [&str; 18] = [
  "nuxt",
  "nuxtjs",
  "playwright",
  "ai-sdk",
  "tanstack",
  "iconify-json",
  "comark",
  "click",
  "submit",
  "import",
  "theme",
  "source",
  "deprecated",
  "generated",
  "stop",
  "reload",
  "update",
  "apply",
];

/// A host name ending in a top-level domain, and the names RFC 2606 reserves for examples.
static HOST: LazyLock<Regex> = LazyLock::new(|| {
  Regex::new(r"(?i)\b[a-z0-9-]+(?:\.[a-z0-9-]+)*\.(?:com|net|org|io|kr|dev|app|local|lan|internal|home|sh|ai|co|me)\b")
    .expect("a host")
});

fn is_public_host(host: &str) -> bool {
  let host = host.to_ascii_lowercase();
  let reserved = ["example.com", "example.net", "example.org"];
  reserved
    .iter()
    .chain(PUBLIC_SITES.iter())
    .any(|site| host == *site || host.ends_with(&format!(".{site}")))
}

/// Each shape a personal value takes, with the word a failure calls it by; a shape answers the matches
/// that are personal, so a public name inside one is let through.
struct Shape {
  word: &'static str,
  pattern: Regex,
  personal: fn(&str) -> bool,
}

static SHAPES: LazyLock<Vec<Shape>> = LazyLock::new(|| {
  let shape = |word, pattern: &str, personal| Shape {
    word,
    pattern: Regex::new(pattern).expect("a shape"),
    personal,
  };
  vec![
    shape("a chat id", r"-100\d{6,}", |_| true),
    shape("an id", r"(?i)\b(?:chat|user|room|account)(?:[ _-]?id)?[ :=#]+-?\d{6,}", |_| true),
    shape("an email address", r"[A-Za-z0-9._%+-]+@([A-Za-z0-9-]+(?:\.[A-Za-z0-9-]+)+)", |found| {
      let domain = found.rsplit('@').next().unwrap_or_default();
      !is_public_host(domain)
    }),
    shape("an IP address", r"\b\d{1,3}(?:\.\d{1,3}){3}\b", |found| {
      found != "127.0.0.1" && found != "0.0.0.0"
    }),
    shape("a private host", HOST.as_str(), |found| !is_public_host(found)),
    shape(
      "a home path",
      r"(?:/home/|/Users/)[A-Za-z][A-Za-z0-9._-]*|~[A-Za-z][A-Za-z0-9._-]*/",
      |_| true,
    ),
    shape("a checkout layout", r"~/repos/", |_| true),
    shape(
      "an account handle",
      r"(?:^|[^A-Za-z0-9._%+-])@([A-Za-z0-9_][A-Za-z0-9_-]{2,})",
      |found| {
        let after = found.rsplit('@').next().unwrap_or_default();
        !CODE_AFTER_AT.contains(&after)
      },
    ),
  ]
});

/// Each personal value's shape on a line of `text`, as `path:line: shape (match)`.
fn hits(path: &str, text: &str) -> Vec<String> {
  let mut found = Vec::new();
  for (number, line) in text.lines().enumerate() {
    for shape in SHAPES.iter() {
      for matched in shape.pattern.find_iter(line) {
        let value = matched
          .as_str()
          .trim_start_matches(|c: char| !c.is_ascii_alphanumeric() && c != '-' && c != '@');
        if (shape.personal)(value) {
          found.push(format!("{path}:{}: {} ({value})", number + 1, shape.word));
        }
      }
    }
  }
  found
}

/// The skill folders, the manifest, the documents and the root's markdown.
fn read_files() -> Vec<PathBuf> {
  let root = Path::new(ROOT);
  let mut paths: Vec<PathBuf> =
    skill_names().iter().flat_map(|name| files_under(&root.join(name))).collect();
  paths.extend(files_under(&root.join(".claude-plugin")));
  paths.extend(files_under(&root.join("docs")));
  paths.extend(
    ["README.md", "CLAUDE.md"]
      .iter()
      .map(|name| root.join(name))
      .filter(|path| path.is_file()),
  );
  paths
}

#[test]
fn no_personal_value_by_its_shape() {
  let found: Vec<String> = read_files()
    .iter()
    .flat_map(|path| hits(&shown(path), &std::fs::read_to_string(path).expect("a file reads")))
    .collect();
  assert!(found.is_empty(), "these lines hold a personal value's shape: {found:#?}");
}

#[test]
fn each_shape_is_found() {
  let sample = "\
sent to -1001234567
chat id: 987654321
write to someone@mail.example.invalid.org
at 10.0.0.12
on build.internal
under /home/someone
in ~/repos/project
ask @some_bot
";
  let found: Vec<String> = hits("sample", sample)
    .into_iter()
    .map(|hit| hit.split(" (").next().unwrap().to_owned())
    .collect();
  assert_eq!(
    found,
    [
      "sample:1: a chat id",
      "sample:2: an id",
      "sample:3: an email address",
      "sample:3: a private host",
      "sample:4: an IP address",
      "sample:5: a private host",
      "sample:6: a home path",
      "sample:7: a checkout layout",
      "sample:8: an account handle",
    ]
  );
  let public = "see https://ui.nuxt.com/docs, mail user@example.com, run on 127.0.0.1, import from '@nuxt/ui', \
                `@click`, ~/.local/share/bmo/deploy/bin/bmo, a 86400000 ms day";
  assert!(hits("public", public).is_empty(), "{:?}", hits("public", public));
}
