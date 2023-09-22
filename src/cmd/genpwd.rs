use clap::Parser;

#[derive(Debug, Parser)]
pub struct GenPwdArgs {
    #[arg(short, long, default_value_t = 30)]
    length: u8,

    /// Chars: abcdefghijklmnopqrstuvwxyz
    #[arg(long, default_value_t = false)]
    exclude_lowercase_char: bool,

    /// Chars: ABCDEFGHIJKLMNOPQRSTUVWXYZ
    #[arg(long, default_value_t = false)]
    exclude_uppercase_char: bool,

    /// Chars: 0123456789
    #[arg(long, default_value_t = false)]
    exclude_digital_char: bool,

    /// Chars: !@#$%^&*+=?
    #[arg(long, default_value_t = false)]
    exclude_special_char: bool,
}

impl GenPwdArgs {
    pub async fn run(self) -> eyre::Result<()> {
        let password = genpwd::gen(
            self.length as usize,
            self.exclude_lowercase_char,
            self.exclude_uppercase_char,
            self.exclude_digital_char,
            self.exclude_digital_char,
        );

        println!("{}", password);
        Ok(())
    }
}
