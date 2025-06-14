mod git;
mod personalities;
mod generator;
mod error;

use clap::Parser;
use anyhow::Result;
use crate::generator::CommitGenerator;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Personality mode for commit message generation
    #[arg(short, long, default_value = "corporate")]
    personality: String,

    /// Save the generated message to favorites
    #[arg(short = 'f', long)]
    save: bool,

    /// Show the git diff before generating message
    #[arg(short = 'd', long)]
    show_diff: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut generator = CommitGenerator::new()?;

    if args.show_diff {
        let git_analyzer = git::GitAnalyzer::new()?;
        let diff = git_analyzer.get_diff()?;
        println!("Git diff:");
        diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
            print!("{}", String::from_utf8_lossy(line.content()));
            true
        })?;
        println!("\n");
    }

    let message = generator.generate_commit_message(&args.personality)?;
    println!("Generated commit message:");
    println!("{}", message);

    if args.save {
        generator.save_favorite(&args.personality, &message)?;
        println!("\nMessage saved to favorites!");
    }

    Ok(())
} 