//! The skill bundle's placer: it reads the skill folders at a repository's root (`docs/format.md`) and
//! writes a bundle or a single skill into a consumer's tree (`docs/boundary.md`).

mod place;
mod skill;

pub use place::{Placed, place_bundle, place_skill};
pub use skill::{Bundle, Skill, read_skills};

use std::fmt;
use std::path::Path;

/// Why a verb did not do what it was asked. Each kind answers with its own exit code.
#[derive(Debug)]
pub enum Fault {
  /// A skill that does not read as the format says, or a write that did not go through.
  Failed(String),
  /// A request that cannot be carried out as asked; nothing is written.
  Refused(String),
}

impl Fault {
  pub fn exit_code(&self) -> u8 {
    match self {
      Fault::Failed(_) => 1,
      Fault::Refused(_) => 2,
    }
  }

  pub(crate) fn failed(path: &Path, why: impl fmt::Display) -> Self {
    Fault::Failed(format!("{}: {why}", path.display()))
  }

  pub(crate) fn refused(path: &Path, why: impl fmt::Display) -> Self {
    Fault::Refused(format!("{}: {why}", path.display()))
  }
}

impl fmt::Display for Fault {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Fault::Failed(line) | Fault::Refused(line) => f.write_str(line),
    }
  }
}

impl std::error::Error for Fault {}
