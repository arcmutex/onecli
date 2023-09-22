use clap::Parser;
use opts::SubCommands;

mod cmd;
mod opts;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let opts: opts::Opts = opts::Opts::parse();

    match opts.sub {
        SubCommands::GenPwd(cmd) => {
            cmd.run().await?;
        }
    }

    Ok(())
}
