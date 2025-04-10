pub mod resume;

use std::io;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to resume text file
    #[arg(short, long)]
    input_resume_path: String,

    /// Path to render resume output file at
    #[arg(short, long)]
    output_resume_path: String,
}

fn main() -> Result<(), io::Error> {
    let args = Args::parse();
    resume::render::render_resume(args.input_resume_path, args.output_resume_path)?;
    Ok(())
}
