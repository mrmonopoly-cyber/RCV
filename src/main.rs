use clap::{arg, command,Parser, Subcommand};
use std::fs;
use std::path::Path;

#[derive(Parser,Debug)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,

    #[arg(short,long)]
    tests_dir_path: String
}

#[derive(Debug,Subcommand)]
enum Commands {
    #[command(about = "List all available tests")]
    List,
    #[command(about = "Add a new test if it does not already exist")]
    Add,
    #[command(about = "Delete a test if it exist")]
    Delete,
    #[command(about = "Run a set of tests sequentially")]
    RunTests{
        #[command(subcommand)]
        commands: RunTestsCommands
    },
}

#[derive(Debug,Subcommand)]
enum RunTestsCommands{
    #[command(about = "Run all tests")]
    All,
    #[command(about = "Run a given set of tests")]
    Set{
        #[arg(value_name = "TEST_NAME")]
        name: Vec<String>,
    },
    #[command(about = "Run all tests except the given ones")]
    Skip{
        #[arg(value_name = "TEST_NAME")]
        name: Vec<String>,
    },

}

fn main() {
    let cli = Cli::parse();

    let tests_path =
    {
        Path::new(match cli.tests_dir_path.as_str(){
            "" => "tests_dir",
            str => str,
        })
    };

    if !tests_path.exists(){
        fs::DirBuilder::new().create(tests_path).expect("failed creting a new test directory")
    }


    match cli.commands {
        Commands::List => todo!(),
        Commands::Add => todo!(),
        Commands::Delete => todo!(),
        Commands::RunTests { commands } => todo!(),
    };
}
