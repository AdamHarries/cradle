extern crate libcradle;

mod cli {
    
    use clap::Parser;
    use clap::Subcommand;
    use clap::ValueEnum;
    use std::path::PathBuf;

    #[derive(Parser, Debug)]
    #[command(name = "PlaySong")]
    #[command(author = "Adam B-H. <harries.adam@gmail.com>")]
    #[command(version = "1.0")]
    #[command(about = "Plays a single song to completion", long_about = None)]
    pub struct Cli {
        /// Optional configuration file (ports, audio devices, etc)
        pub config_file: Option<PathBuf>,

        /// Verbosity for debugging purposes
        #[arg(value_enum)]
        pub verbosity: Option<Verbosity>,

        #[command(subcommand)]
        pub mode: Mode,
    }

    #[derive(Subcommand, Debug)]
    pub enum Mode {      
        /// Direct-mode interaction with Cradle.
        /// In this mode, cradle is launched as an interactive application where the user can choose songs to be played. If the user exits cradle while in this mode then the audio will stop playing.
        Direct,
        /// Server-mode operation.
        /// In this mode, cradle is launched as a server, accepting commands from a client-mode cradle instance.
        Server,
        /// Client-mode operation.
        /// In this mode the user can choose songs to be played, and the client will send commands to a running server to play. If the user exits cradle while in this mode, audio will continue to play from the server.
        Client,
    }

    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
    pub enum Verbosity {
        Fatal,
        Error,
        Warning,
        Debug,
        Info,
    }

    pub fn get_cli() -> Cli {
        Cli::parse()
    }
}

mod oneshot {

}

mod direct { 
    
}

mod server {
    use super::cli;
    pub fn run(args: cli::Cli) -> () {}
}

mod client {
    use super::cli;
    pub fn run(args: cli::Cli) -> () {}
}



fn main() -> () {
    let cli = cli::get_cli();
    println!("Arguments: {:#?}", &cli);
    match cli.mode {
        cli::Mode::Direct => {}
        cli::Mode::Server => server::run(cli),
        cli::Mode::Client => client::run(cli),
    }
}
