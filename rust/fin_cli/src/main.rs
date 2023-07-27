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
    Types(TransactionTypes),
    Rules(ClassifyingRules),
}

#[derive(Args, Debug)]
struct ClassifyingRules {
    #[clap(subcommand)]
    subcommand: Option<ClassifyingRulesSubcommand>,
}

#[derive(Subcommand, Debug)]
enum ClassifyingRulesSubcommand {
    Add {
        #[clap(long)]
        name: String,
        #[clap(long)]
        transaction_type_id: String,
        #[clap(long)]
        pattern: String,
    },
    Update {
        #[clap(long)]
        id: String,
        #[clap(long)]
        name: Option<String>,
        #[clap(long)]
        transaction_type_id: Option<String>,
        #[clap(long)]
        pattern: Option<String>,
    },
    Delete {
        #[clap(long)]
        id: String,
    },
    Reorder {
        #[clap(long)]
        id: String,
        #[clap(long)]
        after: Option<String>,
    },
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

fn classifying_rules_to_row(classifying_rule: &ClassifyingRule) -> [String; 4] {
    [
        classifying_rule.id.to_string(),
        classifying_rule.name.to_string(),
        classifying_rule.transaction_type_id.to_string(),
        classifying_rule.pattern.to_string(),
    ]
}

fn print_classifying_rules(classifying_rules: &[ClassifyingRule]) {
    let headers: [&str; 4] = ["ID", "Name", "Transaction Type ID", "Pattern"];
    let rows: Vec<[String; 4]> = classifying_rules
        .iter()
        .map(classifying_rules_to_row)
        .collect();
    let output_rows = print_table::print_table(&rows, &headers);
    for row in output_rows {
        println!("{}", row);
    }
}

fn transaction_type_to_row(transaction_type: &TransactionType) -> [String; 2] {
    [
        transaction_type.id.to_string(),
        transaction_type.name.to_string(),
    ]
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

async fn classifying_rules<T: FinApi>(api: T, args: ClassifyingRules) {
    if let None = args.subcommand {
        let result = api.get_all_rules(None).await;
        if let Err(e) = result {
            handle_error(e);
            return;
        }
        let classifying_rules = result.unwrap();
        print_classifying_rules(&classifying_rules);
    }
    let subcommand = args.subcommand.expect("Should handle none case");
    match subcommand {
        ClassifyingRulesSubcommand::Add {
            name,
            transaction_type_id,
            pattern,
        } => {
            let result = api
                .create_rule(ClassifyingRuleCreationArgs {
                    name,
                    transaction_type_id,
                    pattern,
                })
                .await;
            if let Err(e) = result {
                handle_error(e);
                return;
            }
            let classifying_rule = result.unwrap();
            print_classifying_rules(&[classifying_rule]);
        }
        ClassifyingRulesSubcommand::Update {
            id,
            name,
            transaction_type_id,
            pattern,
        } => {
            let classifying_rule = ClassifyingRuleUpdateArgs {
                id: id.to_string(),
                name,
                transaction_type_id,
                pattern,
            };
            let result = api.update_rule(classifying_rule).await;
            if let Err(e) = result {
                handle_error(e);
                return;
            }
            let classifying_rule = result.unwrap();
            print_classifying_rules(&[classifying_rule]);
        }
        ClassifyingRulesSubcommand::Delete { id } => {
            let result = api.delete_rule(&id).await;
            if let Err(e) = result {
                handle_error(e);
                return;
            }
            let classifying_rule = result.unwrap();
            print_classifying_rules(&[classifying_rule]);
        }
        ClassifyingRulesSubcommand::Reorder { id, after } => {
            let result = api.reorder_rule(&id, after.as_deref()).await;
            if let Err(e) = result {
                handle_error(e);
                return;
            }
            let classifying_rules = result.unwrap();
            print_classifying_rules(&classifying_rules);
        }
    }
}

async fn transaction_types<T: FinApi>(api: T, args: TransactionTypes) {
    if let None = args.subcommand {
        let result = api.get_all_transaction_types(None).await;
        if let Err(e) = result {
            handle_error(e);
            return;
        }
        let transaction_types = result.unwrap();
        print_transaction_types(&transaction_types);
        return;
    }
    let subcommand = args.subcommand.expect("Should handle none case");
    match subcommand {
        TransactionTypesSubcommand::Add { name } => {
            let result = api.create_transaction_type(&name).await;
            if let Err(e) = result {
                handle_error(e);
                return;
            }
            let transaction_type = result.unwrap();
            print_transaction_types(&[transaction_type]);
        }
        TransactionTypesSubcommand::Update { id, name } => {
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
    }
}

#[derive(Clone, Debug)]
struct ApplicationConfig {
    transaction_type_tablename: String,
    classifying_rules_tablename: String,
    transactions_tablename: String,
    unprocessed_transactions_tablename: String,
}

fn get_default_config() -> ApplicationConfig {
    let transaction_type_tablename = std::env::var("FIN_CLI_TRANSACTION_TYPE_TABLE_NAME")
        .expect("Env var FIN_CLI_TRANSACTION_TYPE_TABLE_NAME is missing");
    let classifying_rules_tablename = std::env::var("FIN_CLI_CLASSIFYING_RULES_TABLE_NAME")
        .expect("Env var FIN_CLI_CLASSIFYING_RULES_TABLE_NAME is missing");
    let transactions_tablename = std::env::var("FIN_CLI_TRANSACTIONS_TABLE_NAME")
        .expect("Env var FIN_CLI_TRANSACTIONS_TABLE_NAME is missing");
    let unprocessed_transactions_tablename =
        std::env::var("FIN_CLI_UNPROCESSED_TRANSACTIONS_TABLE_NAME")
            .expect("Env var FIN_CLI_UNPROCESSED_TRANSACTIONS_TABLE_NAME is missing");
    ApplicationConfig {
        transaction_type_tablename,
        classifying_rules_tablename,
        transactions_tablename,
        unprocessed_transactions_tablename,
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
        classifying_rules_tablename: app_config.classifying_rules_tablename,
        transactions_tablename: app_config.transactions_tablename,
        unprocessed_transactions_tablename: app_config.unprocessed_transactions_tablename,
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
        FinSubcommand::Types(args) => {
            transaction_types(api, args).await;
        }
        FinSubcommand::Rules(args) => {
            classifying_rules(api, args).await;
        }
    }
}
