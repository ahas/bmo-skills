//! The rules the skills carry for every job that loads them. A skill that lost one of these passages
//! would leave its rule read by nobody, so each is held here by the passage itself. Each test is named
//! after the case of the source tree that held the same passages (`docs/inventory.md`, 「Tests」).

use crate::support;

use support::{holds, lacks, skill, skill_flat};

#[test]
fn makes_the_checking_the_jobs_own_work() {
  holds(
    "bmo-closing-a-job",
    &skill_flat("bmo-closing-a-job"),
    &[
      "Verifying your own work is your own work.",
      "the work is not done",
      "go and check it here",
    ],
  );
}

#[test]
fn closes_a_job_by_handing_its_commit_over_after_the_whole_suite_where_the_change_reaches_it() {
  let closing = skill_flat("bmo-closing-a-job");
  holds(
    "bmo-closing-a-job",
    &closing,
    &[
      "where your change reaches what the whole suite runs, the close runs the suite once, at the end",
      "you may skip it, the judgement is yours, and the report says in one line why it was skipped",
      "A passing close hands your commit over as waiting to go live; you do not deploy",
      "Say in the report that a commit is waiting to go live",
      "A failing suite is yours to mend in this same job",
    ],
  );
  lacks(
    "bmo-closing-a-job",
    &closing,
    &["put the change live", "the deploy job", "scripts/suite-deploy.ts"],
  );
}

#[test]
fn excuses_only_what_no_machine_can_reach_and_still_demands_an_account_of_it() {
  holds(
    "bmo-closing-a-job",
    &skill_flat("bmo-closing-a-job"),
    &[
      "what no machine can reach",
      "you write what you did check and what exactly is beyond a machine and why",
      "The only thing that excuses a check left undone is what no machine can reach",
    ],
  );
}

#[test]
fn reports_nothing_unchecked_that_a_check_could_settle_and_names_only_what_is_beyond_reach() {
  holds(
    "bmo-closing-a-job",
    &skill_flat("bmo-closing-a-job"),
    &[
      "Nothing a check could settle is reported as unchecked.",
      "check first, in the maker's own material, the code, the issues and pull requests, and a real run",
      "Only what no job can reach now",
      "Left lists only the rows this job raised, by number, a fact the next reader needs, and what no job can reach now",
    ],
  );
}

#[test]
fn keeps_a_task_handed_to_the_owner_out_of_the_unfinished_note() {
  holds(
    "bmo-closing-a-job",
    &skill_flat("bmo-closing-a-job"),
    &["is work somebody has to pick up next, never a task handed to"],
  );
}

#[test]
fn makes_closing_with_no_unfinished_note_the_ordinary_ending_rather_than_a_blank_field() {
  holds(
    "bmo-closing-a-job",
    &skill_flat("bmo-closing-a-job"),
    &[
      "Closing without it is the ordinary ending and the good one",
      "never a record of what you merely did not do",
    ],
  );
}

#[test]
fn keeps_a_row_from_becoming_the_other_way_out_of_finishing_it() {
  holds(
    "bmo-closing-a-job",
    &skill_flat("bmo-closing-a-job"),
    &[
      "a row is not the other way out",
      "the chat agent approves a row a job raised and launches it at once",
      "raising one to be rid of something only moves the same work into a second run",
      "finishing it here is cheaper than the row, the approval and the second run put together",
    ],
  );
}

#[test]
fn keeps_a_structural_fix_the_work_needs_inside_the_job_and_leaves_only_what_it_cannot_reach() {
  holds(
    "bmo-closing-a-job",
    &skill_flat("bmo-closing-a-job"),
    &[
      "A structural fix the work needs is never left behind, as a note or as a row.",
      "a defect on the same code path as your own change",
      "is yours to design, implement and test inside this job",
      "\"it needs a design of its own\" or \"it is a larger change\" is not a reason to stop short of it",
      "--unfinished is for work this job's hands could not reach: what only",
      "can give — a password or an account of his that no saved secret holds, a judgement of his own taste, a change to STRUCTURE.md",
    ],
  );
}

#[test]
fn leaves_no_improvement_behind_as_a_proposal_and_names_what_only_the_owner_can_give() {
  holds(
    "bmo-closing-a-job",
    &skill_flat("bmo-closing-a-job"),
    &[
      "the report's Left part is never a list of improvements or proposals",
      "The closing refuses a Left part, and a",
      "strand, that proposes, suggests or defers an improvement",
      "An improvement is never anybody's strand",
    ],
  );
}

#[test]
fn sends_the_job_to_finish_it_in_the_same_pass_or_to_raise_a_row_instead_of_writing_a_note() {
  holds(
    "bmo-closing-a-job",
    &skill_flat("bmo-closing-a-job"),
    &[
      "If the same pass finishes it, do it and write nothing",
      "raise it as its own row on your own judgement instead of leaving it as a note",
    ],
  );
}

#[test]
fn sends_a_job_that_changed_a_screen_back_through_the_design_rules_before_it_closes() {
  holds(
    "bmo-closing-a-job",
    &skill_flat("bmo-closing-a-job"),
    &[
      "sweep it against",
      "design rules before you close",
      "where it changed none, skip this paragraph whole",
      "load the skill again at the end",
      "fix what the sweep finds in this same pass",
      "write the sweep into the report under Checked",
    ],
  );
}

#[test]
fn the_workers_own_rules_stand_in_the_workers_own_texts_and_the_skill_says_where_each_rule_belongs() {
  holds(
    "bmo-docs-conventions",
    &skill_flat("bmo-docs-conventions"),
    &[
      "The worker's own rules stand in the worker's own texts, whatever repository the work is in",
      "hold that repository's own business",
      "A worker rule is never copied into a repository's document",
      "ask whether a worker in another checkout, doing other work, needs it",
    ],
  );
}

#[test]
fn the_restructuring_passes_are_found_by_the_owners_words_for_the_work_and_skipped_where_nothing_is_restructured()
 {
  holds(
    "ahas-code-design-patterns",
    &skill_flat("ahas-code-design-patterns"),
    &[
      "and reviewing a design mean the same",
      "Never start at a later pass and never reorder them.",
      "A change that restructures nothing reads none of the three.",
    ],
  );
}

#[test]
fn the_warm_prefix_skill_gives_a_job_the_launch_line_its_item_needs_and_what_a_warm_document_costs_to_edit()
{
  holds(
    "bmo-warm-prefix",
    &skill_flat("bmo-warm-prefix"),
    &[
      "changes that line as part of the item",
      "never worked around with a second road beside the one it should have had",
      "the whole of the change goes in one commit, once",
      "A document in a running session's system prompt",
      "It is expensive to edit.",
      "writes them once",
    ],
  );
}

#[test]
fn and_the_workers_beyond_the_checkout_skill_carries_the_same_rule_from_its_own_side() {
  let beyond = skill_flat("bmo-beyond-the-checkout");
  holds(
    "bmo-beyond-the-checkout",
    &beyond,
    &[
      "Ownership is settled by the domain the work serves**, never by the directory a file lands in",
      "belongs to the repository whose service it stands up",
      "even though not one byte of it is written inside the checkout",
    ],
  );
  lacks(
    "bmo-beyond-the-checkout",
    &beyond,
    &["A job works only inside the repository it runs in"],
  );
  holds(
    "bmo-closing-a-job",
    &skill_flat("bmo-closing-a-job"),
    &["Work the domain rule of the `bmo-beyond-the-checkout` skill gives a repository"],
  );
}

#[test]
fn with_the_estate_to_be_read_rather_than_remembered_and_no_credential_of_its_own() {
  holds(
    "bmo-beyond-the-checkout",
    &skill_flat("bmo-beyond-the-checkout"),
    &[
      "The estate is written down and you read it rather than remember it",
      "the owner's infrastructure repository, which the owner's standing rules name, its `CLAUDE.md` first and then the document under its `docs/`",
      "none of it is copied here",
      "A key, a token or a password never appears in a document or in a report",
      "The permission is bounded by the service it was given for",
    ],
  );
}

#[test]
fn the_rules_only_some_jobs_meet_stand_in_skills_of_their_own_each_saying_when_it_is_read() {
  holds(
    "bmo-asking",
    &skill_flat("bmo-asking"),
    &[
      "`bmo job ask --todo <id>",
      "Each holder has a minute, and nothing lengthens it.",
      "it exits 1, prints nothing on stdout and closes the question unsent",
      "An acknowledgement is written on the question and handed to nobody, so it wakes nobody",
      "`bmo archive doc --title",
      "a question or a closing that carries such an address is refused",
      "A document with a narration is handed over only once its sound is kept",
    ],
  );
  holds(
    "bmo-tools-researching-and-benching",
    &skill_flat("bmo-tools-researching-and-benching"),
    &[
      "Use before the first step of any item that measures, compares, tunes or researches",
      "**1. The maker's material first.**",
      "The work stops only when no option a job can try now is left",
      "Never `bmo job ask` between options the job can try",
      "**It is reported only once its voice is ready.**",
      "**The job releases it before it reports:**",
    ],
  );
  holds(
    "ahas-code-test-writing",
    &skill_flat("ahas-code-test-writing"),
    &[
      "A test never waits on the real clock.",
      "A test that starts a process takes it down itself",
      "A check another test already makes is not made again in another file.",
    ],
  );
  holds(
    "bmo-beyond-the-checkout",
    &skill_flat("bmo-beyond-the-checkout"),
    &["`bmo job borrow <repository>`"],
  );
  holds(
    "ahas-code-boundaries",
    &skill("ahas-code-boundaries"),
    &["## A list a person edits is a table"],
  );
  holds(
    "ahas-code-refactoring",
    &skill_flat("ahas-code-refactoring"),
    &[
      "It also carries the rule that a name never becomes a wrong name later",
      "- **A name is chosen so that it never becomes a wrong name later.**",
      "If the name would mislead them, it is a wrong name already.",
      "the code reads it from its source",
      "3. Leave no alias on the old name",
    ],
  );
  holds(
    "ahas-code-clean",
    &skill_flat("ahas-code-clean"),
    &["「Names that stay true」 is that rule and its test."],
  );
}

#[test]
fn the_design_rules_carry_the_sweep_the_closing_skill_sends_a_job_back_to() {
  holds(
    "ahas-ux-design-rules",
    &skill("ahas-ux-design-rules"),
    &[
      "## Sweeping a screen before the work is closed",
      "Sweep only the screens this change built or touched",
      "Fix what the sweep finds in the same pass",
      "A place left unmet is named,",
    ],
  );
}

#[test]
fn the_three_restructuring_skills_each_name_the_order_and_their_place_in_it() {
  holds(
    "ahas-code-design-patterns",
    &skill_flat("ahas-code-design-patterns"),
    &[
      "## Where this pass sits",
      "design patterns, then refactoring, then clean code",
      "The order is by reach",
    ],
  );
  for (name, place) in [("ahas-code-refactoring", "Second of three"), ("ahas-code-clean", "Last of three")]
  {
    let body = skill_flat(name);
    holds(
      name,
      &body,
      &[
        "## Where this pass sits",
        &format!("{place}: design patterns, then refactoring, then clean code"),
        "`ahas-code-design-patterns` 「Where this pass sits」",
      ],
    );
    lacks(name, &body, &["The order is by reach"]);
  }
}
