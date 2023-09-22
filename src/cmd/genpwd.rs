use clap::Parser;

#[derive(Debug, Parser)]
pub struct GenPwdArgs {
    /// The password length must be greater than 7
    #[arg(short, long, default_value_t = 30)]
    length: u8,

    /// Available chars: abcdefghijklmnopqrstuvwxyz
    #[arg(long, default_value_t = false)]
    exclude_lowercase_char: bool,

    /// Available chars: ABCDEFGHIJKLMNOPQRSTUVWXYZ
    #[arg(long, default_value_t = false)]
    exclude_uppercase_char: bool,

    /// Available chars: 0123456789
    #[arg(long, default_value_t = false)]
    exclude_digital_char: bool,

    /// Available chars: !@#$%^&*+=?
    #[arg(long, default_value_t = false)]
    exclude_special_char: bool,
}

impl GenPwdArgs {
    pub async fn run(self) -> eyre::Result<()> {
        if self.length < 8 {
            eprintln!("error: Invalid value for '--length <LENGTH>': Must be greater than 7");
            return Ok(());
        }

        if self.exclude_lowercase_char
            && self.exclude_uppercase_char
            && self.exclude_digital_char
            && self.exclude_special_char
        {
            eprintln!("error: You should not exclude all available chars!");
            return Ok(());
        }

        let password = genpwd::gen(
            self.length as usize,
            self.exclude_lowercase_char,
            self.exclude_uppercase_char,
            self.exclude_digital_char,
            self.exclude_special_char,
        );

        println!("{}", password);
        Ok(())
    }
}
