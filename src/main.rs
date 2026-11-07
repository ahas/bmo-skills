//! `bmo-skills`: lists the skills of a checkout of this repository and places a bundle or one skill into
//! a consumer's tree (`docs/boundary.md`).

use bmo_skills::{Bundle, Fault, place_bundle, place_skill, read_skills};
use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(
  name = "bmo-skills",
  version,
  about = "List the skills and place a bundle or a skill into a consumer's tree."
)]
struct Cli {
  #[command(subcommand)]
  verb: Verb,
}

#[derive(Subcommand)]
enum Verb {
  /// List every skill folder with its bundle.
  List {
    /// Print one JSON array instead of a line per skill.
    #[arg(long)]
    json: bool,
    #[command(flatten)]
    from: From,
  },
  /// Write a bundle, or one skill, into a directory.
  Place {
    #[command(flatten)]
    what: What,
    /// The directory the bundle is written into, or the one the skill's folder is written in.
    #[arg(long, value_name = "dir")]
    into: PathBuf,
    #[command(flatten)]
    from: From,
  },
}

#[derive(Args)]
struct From {
  /// The checkout of this repository the skills are read from.
  #[arg(long = "from", value_name = "root", default_value = ".")]
  root: PathBuf,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct What {
  /// The bundle to write as a plugin.
  #[arg(long, value_enum)]
  bundle: Option<PluginBundle>,
  /// The one skill to write.
  #[arg(long, value_name = "name")]
  skill: Option<String>,
}

/// The bundles a plugin is placed for.
#[derive(Clone, Copy, ValueEnum)]
enum PluginBundle {
  Worker,
  Coder,
  Producer,
}

impl PluginBundle {
  fn bundle(self) -> Bundle {
    match self {
      PluginBundle::Worker => Bundle::Worker,
      PluginBundle::Coder => Bundle::Coder,
      PluginBundle::Producer => Bundle::Producer,
    }
  }
}

fn main() -> ExitCode {
  match run(Cli::parse()) {
    Ok(()) => ExitCode::SUCCESS,
    Err(fault) => {
      eprintln!("bmo-skills: {fault}");
      ExitCode::from(fault.exit_code())
    }
  }
}

fn run(cli: Cli) -> Result<(), Fault> {
  match cli.verb {
    Verb::List { json, from } => {
      let skills = read_skills(&from.root)?;
      if json {
        let text = serde_json::to_string_pretty(&skills).map_err(|why| Fault::Failed(why.to_string()))?;
        println!("{text}");
      } else {
        for skill in &skills {
          println!("{}\t{}", skill.name, skill.bundle.as_str());
        }
      }
    }
    Verb::Place { what, into, from } => match (what.bundle, what.skill) {
      (Some(bundle), _) => {
        let placed = place_bundle(&from.root, bundle.bundle(), &into)?;
        let bundle = bundle.bundle().as_str();
        println!("placed {} skills of the {bundle} bundle into {}", placed.skills, placed.path.display());
      }
      (None, Some(name)) => {
        let placed = place_skill(&from.root, &name, &into)?;
        println!("placed the skill {name} into {}", placed.path.display());
      }
      (None, None) => unreachable!("clap requires one of --bundle and --skill"),
    },
  }
  Ok(())
}
