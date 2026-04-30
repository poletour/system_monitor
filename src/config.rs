use std::time::Duration;
use std::env::Args;

pub struct Config {
    pub interval: Duration,
}

pub fn parse_args(mut args: Args) -> Result<Config, &'static str> {
      match args.next() {
         Some(arg) if arg == "--interval" => {
            if let Some(arg) = args.next() {
               if let Ok(secs) = arg.parse::<u64>() {
                  if secs > 0 {
                     let interval: Duration = Duration::from_secs(secs);
                     match args.next() {
                        Some(_) => {
                           Err("Argument en trop. Syntaxe attendue : --interval <nombre>")
                        }
                        None => {
                           Ok(Config { interval })
                        }
                     }
                  }
                  else {
                     Err("L'intervalle doit être supérieur à 0.")
                  }
               }
               else {
                  Err("Erreur de parsing. Syntaxe attendue : --interval <nombre>")
               }
            }
            else {
               Err("Valeur manquante; syntaxe attendue : --interval <nombre>")
            }
         }
         Some(_) => {
            Err("Erreur de parsing. Syntaxe attendue : --interval <nombre>")
         }
         None => {
            let interval: Duration = Duration::from_secs(5);
            Ok(Config { interval })
         }
   }
}