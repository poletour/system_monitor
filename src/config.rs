use std::time::Duration;
use std::env::Args;
use std::fmt;

pub struct Config {
    pub interval: Duration,
}

#[derive(Debug, PartialEq)]
pub enum ConfigError {
   MissingIntervalValue,
   InvalidInterval(String),
   IntervalMustBePositive,
   UnexpectedArgument(String),
}

impl fmt::Display for ConfigError {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self {
         ConfigError::MissingIntervalValue => {
            write!(f, "Valeur manquante. Syntaxed attendue: --interval <nombre>")
         }
         ConfigError::InvalidInterval(value) => {
            write!(f, "Intervalle invalide: '{value}'. Syntaxe attendue: --interval <nombre>")
         }
         ConfigError::IntervalMustBePositive => {
            write!(f, "L<intervalle doit être supérieur à 0.")
         }
         ConfigError::UnexpectedArgument(arg) => {
            write!(f, "Argument innatendu: '{arg}'. SYntaxe attendue: --interval <nombre>")
         }
      }
   }
}

pub fn parse_args(mut args: Args) -> Result<Config, ConfigError> {
      match args.next() {
         Some(arg) if arg == "--interval" => {
            let value = args.next().ok_or(ConfigError::MissingIntervalValue)?;

            let secs = value
               .parse::<u64>()
               .map_err(|_| ConfigError::InvalidInterval(value.clone()))?;

            if secs == 0 {
               return Err(ConfigError::IntervalMustBePositive);
            }

            if let Some(extra_arg) = args.next() {
               return Err(ConfigError::UnexpectedArgument(extra_arg));
            }

            Ok(Config {
               interval: Duration::from_secs(secs),
            })
         }

         Some(arg) => Err(ConfigError::UnexpectedArgument(arg)),

         None => Ok(Config {
            interval: Duration::from_secs(5),
         }),
      }
}