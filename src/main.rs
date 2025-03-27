use clap::{arg, command, ValueEnum, Parser, Subcommand};
use std::path::Path;

mod core;

#[derive(Parser,Debug)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,

    #[arg(long,required=false,default_value="tests_dir")]
    storage_dir_path: String,

    #[arg(long,required=false,default_value="./.dummy")]
    template_path: String,

    #[arg(long,required=false,default_value="./Makefile")]
    env_makefile_path: String,
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
    #[command(about = "Setup the test environment")]
    SetEnv{
        status: EnvStatus
    }
}

#[derive(ValueEnum,Clone,Debug)]
enum EnvStatus{
    Init,
    Close,
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
    let tests_path = Path::new(cli.storage_dir_path.as_str());
    let template_path = Path::new(cli.storage_dir_path.as_str());
    let setup_makefile_path = Path::new(cli.env_makefile_path.as_str());
    let rcv = core::RCV::new(tests_path, template_path,setup_makefile_path)
        .unwrap_or_else(|err|{
            println!("{}",err);
            std::process::exit(1);
        });


    match cli.commands {
        Commands::List => rcv.list_tests(),
        Commands::Add { name } => {
            let err = rcv.add_test(name.as_str());
            match err {
                Ok(_) => (),
                Err(_) => println!("add test failed"),
            }
        },
        Commands::Delete { name } => {
            let err = rcv.rem_test(name.as_str());
            match err {
                Ok(_) => (),
                Err(_) => println!("rm test failed"),
            }
        },
        Commands::SetEnv{ status } =>
        {
            match status{
                EnvStatus::Init => rcv.setup_env(true),
                EnvStatus::Close => rcv.setup_env(false),
            }
        }
        Commands::RunTests { commands } => {
            match commands{
                RunTestsCommands::All => rcv.run_tests(None, None),
                RunTestsCommands::Set { names } => rcv.run_tests(Some(&names), None),
                RunTestsCommands::Skip { names } => rcv.run_tests(None, Some(&names)),
            }
        },
    };
}
