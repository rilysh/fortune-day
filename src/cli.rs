use clap::{Args, Parser, Subcommand};

/// Unless structures needed to printed by print macros,
/// Debug trait is not necessary.
#[derive(Debug, Args)]
pub struct SearchCommand {
    /// Case-insensitive search for the query
    #[arg(short, long)]
    pub insensitive: bool,

    /// Add colors on the matching queries
    #[arg(short, long)]
    pub colors: bool,

    /// Only print the single value (first match)
    #[arg(short, long)]
    pub single: bool,

    /// A query to search in every single fortunes file
    #[arg(value_name = "QUERY")]
    pub query: String,

    /// Regex and pattern matching
    #[arg(short, long)]
    pub regex: bool,
}

#[derive(Debug, Args)]
pub struct FortuneCommand {
    /// Number of random fortunes
    #[arg(long)]
    #[clap(default_value_t = 1)]
    pub howmany: i32,
}

#[derive(Debug, Args)]
pub struct DayCommand {
    /// Number of random fortunes
    #[arg(long)]
    #[clap(default_value_t = 1)]
    pub howmany: i32,
}

#[derive(Debug, Args)]
pub struct WaitCommand {
    /// Wait in days
    #[arg(long)]
    #[clap(default_value_t = 0)]
    pub days: u64,

    /// Wait in hours
    #[arg(long)]
    #[clap(default_value_t = 0)]
    pub hours: u64,

    /// Wait in minutes
    #[arg(long)]
    #[clap(default_value_t = 0)]
    pub mins: u64,

    /// Wait in seconds
    #[arg(long)]
    #[clap(default_value_t = 0)]
    pub secs: u64,

    /// Wait in milliseconds
    #[arg(long)]
    #[clap(default_value_t = 0)]
    pub millis: u64,

    /// Wait in nanoseconds
    #[arg(long)]
    #[clap(default_value_t = 0)]
    pub nanos: u64,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Fortune anytime you want
    Fortune(FortuneCommand),

    /// Search for a keyword in cookie files
    Search(SearchCommand),

    /// Fortune of a day
    Day(DayCommand),

    /// Wait given amount of time between fortunes in an infinite loop
    Wait(WaitCommand),
}

#[derive(Parser, Debug)]
pub struct Cli {
    /// Accept list of (enum) commands
    #[command(subcommand)]
    pub command: Option<Commands>,
}
