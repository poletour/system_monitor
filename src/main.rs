use std::fs; // Filesystem standard library
use std::thread;
use std::time::Duration;
use std::env::Args;
//use system_monitor::meminfo::MemoryStats;


fn main() {
   let mut args = std::env::args();
   let _program = args.next();
   //let mut interval: Duration = Duration::from_secs(5);

   let interval = match parse_args(args) {
      Ok(interval) => { 
         interval
      }
      Err(msg) => {
         eprintln!("{}", msg);
         return;
      }
   };

   println!("Moniteur démarré...");

   // boucle infinie Daemon
   loop{
        let content = fs::read_to_string("/proc/meminfo").unwrap_or_default();

        let stats = system_monitor::meminfo::parse_meminfo(&content);

        match stats {
            Ok(stats) => {
               let total = stats.total;
               let free = match stats.free {
                  Some(v) => (v/1024).to_string(),
                  None => "N/A".to_string()
               };
               let available = match stats.available {
                  Some(v) => (v/1024).to_string(),
                  None => "N/A".to_string()
               };
               println!("RAM Total: {} MB | Libre: {} MB | Disponible: {} MB", 
               total / 1024, free, available);
            }
            Err(err) => {
               eprintln!("...{:?}", err);
            }
        }

        // Etape cruciale: sleep
        // Sans celle ci, la boucle tourne a 100% CPU usage et fait surchauffer la machine.
        // Un bon daemon passe 99% de sa vie en sleep.
        thread::sleep(interval);
   }
}

fn parse_args(mut args: Args) -> Result<Duration, &'static str> {
      match args.next() {
         Some(arg) if arg == "--interval" => {
            if let Some(arg) = args.next() {
               if let Ok(secs) = arg.parse::<u64>() {
                  if secs > 0 {
                     let interval: Duration = Duration::from_secs(secs);
                     //Ok(interval)
                     match args.next() {
                        Some(_) => {
                           Err("Argument en trop. Syntaxe attendue : --interval <nombre>")
                        }
                        None => {
                           Ok(interval)
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
            Ok(interval)
         }
   }
}
