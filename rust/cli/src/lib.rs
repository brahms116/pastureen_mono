use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("IO error: {0}")]
    Io(std::io::Error),
}

impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

pub fn choice(question: &str, choices: &[&str]) -> Result<u32, CliError> {
    println!("{}", question);
    loop {
        for (i, choice) in choices.iter().enumerate() {
            println!("{}. {}", i + 1, choice);
        }
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();
        let parsed_input = input.parse::<u32>();
        if let Err(_) = parsed_input {
            println!("Invalid input: {}", input);
            continue;
        }
        let parsed_input = parsed_input.unwrap();
        if parsed_input < 1 || parsed_input > choices.len() as u32 {
            println!("Invalid input: {}", input);
            continue;
        }
        return Ok(parsed_input);
    }
}
