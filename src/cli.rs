use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[clap(short, long)]
    pub file: PathBuf,

    #[arg(short, long, env = "G5K_USERNAME")]
    pub username: String,

    #[arg(short, long, env = "G5K_PASSWORD")]
    pub password: String,

    #[clap(short, long)]
    pub command: String,

    #[clap(short, long)]
    pub queue: Option<String>,

    #[clap(short, long)]
    pub directory: Option<String>,

    #[clap(long)]
    pub stdout: Option<String>,

    #[clap(long)]
    pub stderr: Option<String>,
}
