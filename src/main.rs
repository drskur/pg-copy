use clap::{Clap, AppSettings};
use crate::db::{copy_out, copy_in};
use std::fs::{remove_file};

pub mod db;

#[derive(Clap)]
#[clap(version = "0.1", author = "drskur<drskur@me.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(long)]
    origin_database_url: String,
    #[clap(long)]
    target_database_url: String,
    #[clap(long)]
    origin_table_name: String,
    #[clap(long)]
    target_table_name: String
}

fn main() -> anyhow::Result<()>{
    let opts: Opts = Opts::parse();

    let temp = "./temp.csv";
    let mut origin_client = db::create_client(&opts.origin_database_url);
    let mut target_client = db::create_client(&opts.target_database_url);

    copy_out(&mut origin_client, &opts.origin_table_name, temp)?;
    copy_in(&mut target_client, &opts.target_table_name, temp)?;
    remove_file(temp)?;

    Ok(())
}
