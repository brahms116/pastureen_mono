use clap::{Args, Parser, Subcommand};
use fin_api::*;

#[derive(Parser, Debug)]
struct CliArgs {
    #[clap(subcommand)]
    command: Option<FinSubcommand>,
}

#[derive(Subcommand, Debug)]
enum FinSubcommand {
    Process,
    Unprocessed,
    TransactionTypes(TransactionTypes),
}

#[derive(Args, Debug)]
struct TransactionTypes {
    #[clap(subcommand)]
    subcommand: Option<TransactionTypesSubcommand>,
}

#[derive(Subcommand, Debug)]
enum TransactionTypesSubcommand {
    Add {
        #[clap(long)]
        name: String,
    },
    Update {
        #[clap(long)]
        id: i32,
        #[clap(long)]
        name: String,
    },
}

fn handle_error<T: std::error::Error> (error: T) {
    println!("Oops something went wrong: {}", error);
}

async fn transaction_types<T: FinApi>(api: T, args: TransactionTypes) {
    match args.subcommand {
        Some(TransactionTypesSubcommand::Add { name }) => {
            let result = api.create_transaction_type(&name).await;
            if let Err(e) = result {
                handle_error(e);
                return;
            }
            let transaction_type = result.unwrap();
            println!("Created transaction type: {:?}", transaction_type);
        }
        Some(TransactionTypesSubcommand::Update { id, name }) => {
            let transaction_type = TransactionType {
                id: id.to_string(),
                name,
            };
            let result = api.update_transaction_type(transaction_type).await;
            if let Err(e) = result {
                handle_error(e);
                return;
            }
            let transaction_type = result.unwrap();
            println!("Updated transaction type: {:?}", transaction_type);
        }
        None => {
            let result = api.get_all_transaction_types().await;
            if let Err(e) = result {
                handle_error(e);
                return;
            }
            let transaction_types = result.unwrap();
            println!("Transaction types: {:?}", transaction_types);
        }
    }
}

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    let config = aws_config::load_from_env().await;
    let client = aws_sdk_dynamodb::Client::new(&config);

    let db = FinDynamoDb {
        client,
        transaction_type_tablename: "transaction_types".to_string(),
    };

    let api = FinApiService::new(db);

    if let None = args.command {
        println!("No command provided");
        return;
    }
    let command = args.command.expect("Should handle none case above");

    match command {
        FinSubcommand::Process => {
            println!("Process");
        }
        FinSubcommand::Unprocessed => {
            println!("Unprocessed");
        }
        FinSubcommand::TransactionTypes(args) => {
            transaction_types(api, args).await;
        }
    }
}
