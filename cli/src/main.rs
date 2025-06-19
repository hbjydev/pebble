use clap::{Parser, Subcommand};
use pebble_server::{PebbleServer, args::PebbleServerArgs};

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Start the Pebble server.
    Start(PebbleServerArgs),
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    match args.command {
        Command::Start(args) => {
            let srv = PebbleServer::new(args)
                .await
                .expect("failed to build service");

            srv.start().await.expect("failed to start server");
        },
    }
}
