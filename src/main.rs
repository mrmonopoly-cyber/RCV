use clap::{arg, command,Parser, Subcommand};
use std::fs;
use std::path::Path;

mod core;

#[derive(Parser,Debug)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,

    #[arg(short,long)]
    tests_dir_path: String,

    #[arg(short,long)]
    test_template_path: String,
}

#[derive(Debug,Subcommand)]
enum Commands {
    #[command(about = "List all available tests")]
    List,
    #[command(about = "Add a new test if it does not already exist")]
    Add{
        #[arg(value_name = "TEST_NAME")]
        name: String,
    },
    #[command(about = "Delete a test if it exist")]
    Delete{
        #[arg(value_name = "TEST_NAME")]
        name: String,
    },
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
        names: Vec<String>,
    },
    #[command(about = "Run all tests except the given ones")]
    Skip{
        #[arg(value_name = "TEST_NAME")]
        names: Vec<String>,
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

    let test_template_path =
    {
        Path::new(match cli.tests_dir_path.as_str(){
            "" => "./.dummy",
            str => str,
        })
    };

    let rcv = core::RCV::new(tests_path, test_template_path)
        .unwrap_or_else(|err|{
            println!("{}",err);
            std::process::exit(1);
        });


    match cli.commands {
        Commands::List => rcv.list_tests(),
        Commands::Add { name } => rcv.add_test(name.as_str()),
        Commands::Delete { name } => rcv.rem_test(name.as_str()),
        Commands::RunTests { commands } => {
            match commands{
                RunTestsCommands::All => rcv.run_tests(None, None),
                RunTestsCommands::Set { names } => rcv.run_tests(Some(&names), None),
                RunTestsCommands::Skip { names } => rcv.run_tests(None, Some(&names)),
            }
        },
    };
}
