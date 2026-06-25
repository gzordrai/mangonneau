use std::{fs, path::PathBuf};

use clap::Parser;
use futures::{StreamExt, TryStreamExt, stream};
use g5k::{G5KClient, Site, job::JobPayload};
use serde::Serialize;

use crate::{
    cli::Args,
    config::Config,
    error::{MangonneauError, Result},
};

mod cli;
mod config;
mod error;

#[derive(Default, Serialize)]
pub struct BenchJobs<'a> {
    jobs: Vec<BenchJob<'a>>,
}

impl<'a> BenchJobs<'a> {
    pub fn new(jobs: Vec<BenchJob<'a>>) -> Self {
        Self { jobs }
    }
}

#[derive(Serialize)]
pub struct BenchJob<'a> {
    id: String,
    site: Site,
    node: &'a str,
    cpu: &'a str,
}

impl<'a> BenchJob<'a> {
    pub fn new(id: String, site: Site, node: &'a str, cpu: &'a str) -> Self {
        Self {
            id,
            site,
            node,
            cpu,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let data = std::fs::read(args.file)?;
    let config: Config = toml::from_slice(&data)?;
    let client = G5KClient::new(args.username, args.password);

    let jobs: Vec<BenchJob> = stream::iter(config.bench.into_iter())
        .map(|b| {
            let client = &client;
            let command = &args.command;
            let directory = args.directory.as_deref();

            async move {
                let res = client
                    .submit_job(
                        b.site,
                        &JobPayload {
                            command,
                            queue: Some(b.queue),
                            properties: Some(b.node),
                            directory,
                            ..Default::default()
                        },
                    )
                    .await?;

                Ok::<_, MangonneauError>(BenchJob::new(res, b.site, b.node, b.cpu))
            }
        })
        .buffer_unordered(4)
        .try_collect()
        .await?;

    fs::write(
        args.output.unwrap_or(PathBuf::from("./output.toml")),
        toml::to_string_pretty(&BenchJobs::new(jobs))?,
    )?;

    Ok(())
}
