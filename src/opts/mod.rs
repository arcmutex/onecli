use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "onecli")]
#[command(version = "0.0.1")]
pub struct Opts {
    #[clap(subcommand)]
    pub sub: SubCommands,
}

#[derive(Debug, Subcommand)]
pub enum SubCommands {
    #[clap(name = "gen-password")]
    #[clap(about = "Generate random password(s)")]
    GenPwd(crate::cmd::genpwd::GenPwdArgs),
}
