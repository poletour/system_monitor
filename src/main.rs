use std::fs; // Filesystem standard library
use std::thread;
use system_monitor::config;
use system_monitor::system;


fn main() {
   let mut args = std::env::args();
   let _program = args.next();

   let config = match config::parse_args(args) {
      Ok(config) => config, 
      Err(err) => {
         eprintln!("Erreur: {err}");
         std::process::exit(1);
      }
   };

   println!("Moniteur démarré...");

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

        thread::sleep(config.interval);
   }
}

