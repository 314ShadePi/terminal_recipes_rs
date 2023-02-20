use inquire::validator::Validation;
use inquire::CustomUserError;

pub trait EnumCommandLine: Clone {
    fn run(&self) -> anyhow::Result<()>;
    fn validate(input: &str) -> Result<Validation, CustomUserError>;
    fn command_line<F>(prompt: &str, error_handler: F)
    where
        F: Fn(anyhow::Error) -> Result<(), ()>;
    fn from_cl(s: &str) -> anyhow::Result<Self>;
}
