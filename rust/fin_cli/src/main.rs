use chrono::NaiveDate;
use clap::{Args, Parser, Subcommand};
use csv_iterator::*;
use fin_api::*;

#[derive(Parser, Debug)]
struct CliArgs {
    #[clap(subcommand)]
    command: FinSubcommand,
}

#[derive(Subcommand, Debug)]
enum FinSubcommand {
    Report {
        #[clap(long)]
        from: String,
        #[clap(long)]
        to: String,
    },
    List {
        #[clap(long)]
        from: String,
        #[clap(long)]
        to: String,
    },
    Import {
        #[clap(long)]
        path: String,
    },
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

fn print_report(report: &Report) {
    println!("Report from {} to {}", report.start_date, report.end_date);
    println!("");
    let headers: [&str; 2] = ["Transaction Type", "Amount"];
    let rows: Vec<[String; 2]> = report
        .by_type
        .iter()
        .map(|(transaction_type, amount)| [transaction_type.to_string(), amount.to_string()])
        .collect();
    let output_rows = print_table::print_table(&rows, &headers);
    for row in output_rows {
        println!("{}", row);
    }
    println!("");
    println!("Total: {}", report.total);
}

fn transaction_to_row(transaction: &Transaction) -> [String; 5] {
    [
        transaction.id.to_string(),
        transaction.date.to_string(),
        transaction.amount_cents.to_string(),
        transaction.description.to_string(),
        transaction.transaction_type_id.to_string(),
    ]
}

fn print_transactions(transactions: &[Transaction]) {
    let headers: [&str; 5] = ["ID", "Date", "Amount", "Description", "Transaction Type ID"];
    let rows: Vec<[String; 5]> = transactions.iter().map(transaction_to_row).collect();
    let output_rows = print_table::print_table(&rows, &headers);
    for row in output_rows {
        println!("{}", row);
    }
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

fn unprocessed_transactions_to_row(transaction: &UnprocessedTransaction) -> [String; 3] {
    [
        transaction.date.to_string(),
        transaction.amount_cents.to_string(),
        transaction.description.to_string(),
    ]
}

fn print_unprocessed_transactions(transacitons: &[UnprocessedTransaction]) {
    let headers: [&str; 3] = ["Date Timestamp", "Amount", "Description"];
    let rows: Vec<[String; 3]> = transacitons
        .iter()
        .map(unprocessed_transactions_to_row)
        .collect();
    let output_rows = print_table::print_table(&rows, &headers);
    for row in output_rows {
        println!("{}", row);
    }
}

async fn report<T: FinApi>(api: T, from: String, to: String) {
    let from = NaiveDate::parse_from_str(&from, "%d/%m/%Y");
    if let Err(e) = from {
        println!("Could not parse from date: {}", e);
        return;
    }
    let from = from
        .expect("Should handle error case")
        .and_hms_opt(0, 0, 0)
        .unwrap();

    let to = NaiveDate::parse_from_str(&to, "%d/%m/%Y");
    if let Err(e) = to {
        println!("Could not parse to date: {}", e);
        return;
    }
    let to = to
        .expect("Should handle error case")
        .and_hms_opt(0, 0, 0)
        .unwrap();

    let result = api.generate_report(from.timestamp(), to.timestamp()).await;
    if let Err(e) = result {
        handle_error(e);
        return;
    }
    let report = result.expect("Should handle error case");
    print_report(&report);
}

async fn unprocessed<T: FinApi>(api: T) {
    let result = api.get_all_unprocessed_transactions(None).await;
    if let Err(e) = result {
        handle_error(e);
        return;
    }
    let unprocessed_transactions = result.expect("Should handle error case");
    print_unprocessed_transactions(&unprocessed_transactions);
}

async fn list<T: FinApi>(api: T, from: String, to: String) {
    let from = NaiveDate::parse_from_str(&from, "%d/%m/%Y");
    if let Err(e) = from {
        println!("Could not parse from date: {}", e);
        return;
    }
    let from = from
        .expect("Should handle error case")
        .and_hms_opt(0, 0, 0)
        .unwrap();

    let to = NaiveDate::parse_from_str(&to, "%d/%m/%Y");
    if let Err(e) = to {
        println!("Could not parse to date: {}", e);
        return;
    }
    let to = to
        .expect("Should handle error case")
        .and_hms_opt(0, 0, 0)
        .unwrap();

    let result = api
        .list_transactions(from.timestamp(), to.timestamp(), None)
        .await;

    if let Err(e) = result {
        handle_error(e);
        return;
    }
    let result = result.expect("Should handle error case");
    print_transactions(&result);
}

async fn import<T: FinApi>(api: T, path: String) {
    let iter = CsvIterator::<INGTransaction>::new(&path);
    if iter.is_err() {
        println!("Could not read file: {}", path);
    }
    let mut iter = iter.expect("Should handle error case");
    let mut transactions = Vec::new();
    while let Some(transaction) = iter.next() {
        if let Ok(transaction) = transaction {
            transactions.push(transaction);
        } else {
            println!("Could not parse transaction {:?}", transaction);
        }
    }
    let result = api.process_ing_transactions(&transactions).await;
    if let Err(e) = result {
        handle_error(e);
        return;
    }
    let result = result.expect("Should hanle error case");
    println!("{result}")
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
        return
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


async fn process<T: FinApi>(api: T) {
    let result = api.process().await;
    if let Err(e) = result {
        handle_error(e);
        return;
    }
    let result = result.expect("Should handle error case");
    println!("Processed count: {result}")
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

    let _db = FinDynamoDb {
        client,
        transaction_type_tablename: app_config.transaction_type_tablename,
        classifying_rules_tablename: app_config.classifying_rules_tablename,
        transactions_tablename: app_config.transactions_tablename,
        unprocessed_transactions_tablename: app_config.unprocessed_transactions_tablename,
    };

    let postgres = FinPostgres::new().await.expect("Should be able to create postgres");

    let api = FinApiService::new(postgres);

    let command = args.command;

    match command {
        FinSubcommand::Process => {
            process(api).await;
        }
        FinSubcommand::Unprocessed => {
            unprocessed(api).await;
        }
        FinSubcommand::Types(args) => {
            transaction_types(api, args).await;
        }
        FinSubcommand::Rules(args) => {
            classifying_rules(api, args).await;
        }
        FinSubcommand::Import { path } => {
            import(api, path).await;
        }
        FinSubcommand::List { from, to } => {
            list(api, from, to).await;
        }
        FinSubcommand::Report { from, to } => {
            report(api, from, to).await;
        }
    }
}
