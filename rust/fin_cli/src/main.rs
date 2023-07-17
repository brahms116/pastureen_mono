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
    #[clap(alias = "types")]
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
        id: String,
        #[clap(long)]
        name: String,
    },
}

fn handle_error<T: std::error::Error>(error: T) {
    println!("Oops something went wrong: {:?}", error);
}

fn transaction_type_to_row(transaction_type: &TransactionType) -> [String; 2] {
    [transaction_type.id.to_string(), transaction_type.name.to_string()]
}

fn print_transaction_types(transaction_types: &[TransactionType]) {
    let headers: [&str; 2] = ["ID", "Name"];
    let rows: Vec<[String; 2]> = transaction_types
        .iter()
        .map(transaction_type_to_row)
        .collect();
    let output_rows = print_table::print_table(&rows, &headers);
    for row in output_rows {
        println!("{}", row);
    }
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
            print_transaction_types(&[transaction_type]);
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
            print_transaction_types(&[transaction_type]);
        }
        None => {
            let result = api.get_all_transaction_types().await;
            if let Err(e) = result {
                handle_error(e);
                return;
            }
            let transaction_types = result.unwrap();
            print_transaction_types(&transaction_types);
        }
    }
}

#[derive(Clone, Debug)]
struct ApplicationConfig {
    transaction_type_tablename: String,
}

fn get_default_config() -> ApplicationConfig {
    let transaction_type_tablename = std::env::var("FIN_CLI_TRANSACTION_TYPE_TABLE_NAME")
        .expect("Env var FLIN_CLI_TRANSACTION_TYPE_TABLE_NAME is missing");
    ApplicationConfig {
        transaction_type_tablename,
    }
}

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    let aws_config = aws_config::load_from_env().await;
    let client = aws_sdk_dynamodb::Client::new(&aws_config);

    let app_config = get_default_config();

    let db = FinDynamoDb {
        client,
        transaction_type_tablename: app_config.transaction_type_tablename,
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
