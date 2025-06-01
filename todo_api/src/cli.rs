use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'A', long, help = "Database address")]
    pub addr: String,

    #[arg(short = 'U', long, help = "Authentication username")]
    pub username: String,

    #[arg(short = 'P', long, help = "Authentication password")]
    pub passwd: String,
}
