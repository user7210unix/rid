// defargs.rs
//
// responsible for defining args

use clap::Parser;

// TODO: Add color to #[command()]
#[derive(Parser, Debug)]
#[command(
    version,
    about = "Source-based LFS package manager",
    long_about,
    arg_required_else_help = true,
    after_help = "If you have any questions, you can DM me on Discord @toxikuu"
)]
pub struct Args {
    // core flags
    #[arg(short = 'I', long, value_name = "PACKAGE", num_args = 1.., value_delimiter = ' ')]
    pub install_with_dependencies: Option<Vec<String>>,

    #[arg(short = 'i', long, value_name = "PACKAGE", num_args = 1.., value_delimiter = ' ')]
    pub install: Option<Vec<String>>,

    #[arg(short = 'r', long, value_name = "PACKAGE", num_args = 1.., value_delimiter = ' ')]
    pub remove: Option<Vec<String>>,

    #[arg(short = 'u', long, value_name = "PACKAGE", num_args = 1.., value_delimiter = ' ')]
    pub update: Option<Vec<String>>,

    #[arg(short = 'd', long, value_name = "PACKAGE", num_args = 1.., value_delimiter = ' ')]
    pub dependencies: Option<Vec<String>>,

    #[arg(short = 'p', long, value_name = "PACKAGE", num_args = 1.., value_delimiter = ' ')]
    pub prune: Option<Vec<String>>,

    #[arg(short = 'l', long, value_name = "PACKAGE", num_args = 0.., value_delimiter = ' ')]
    pub list: Option<Vec<String>>, // TODO: rewrite this without Option<>

    // function flags
    #[arg(short = 'b', long)]
    pub bootstrap: bool,

    #[arg(short = 's', long)]
    pub sync: bool,

    #[arg(short = 'o', long)]
    pub overwrite: bool,

    #[arg(short = 'D', long)]
    pub download: bool,

    #[arg(short = 'c', long)]
    pub cache: bool,

    #[arg(short = 'k', long)]
    pub check_upstream: bool,

    #[arg(short = 'L', long)]
    pub validate_links: bool,

    // generic flags
    #[arg(short = 'v', long)]
    pub verbose: bool,
 
    #[arg(short = 'q', long)]
    pub quiet: bool,
 
    #[arg(short = 'f', long)]
    pub force: bool,
}

pub fn init_args() -> Args {
    // let mut args = Args::parse(); 
    //
    // if args.update.is_some() {
    //     args.force = true;
    // }
    // args
    Args::parse()
} 
