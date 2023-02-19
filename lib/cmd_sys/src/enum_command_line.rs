use inquire::validator::Validation;
use inquire::CustomUserError;

pub trait EnumCommandLine: Clone {
    type Err;

    fn run(&self) -> Result<(), Self::Err>;
    fn validate(input: &str) -> Result<Validation, CustomUserError>;
    fn command_line(prompt: &str);
    fn from_cl(s: &str) -> Result<Self, Self::Err>;
}
