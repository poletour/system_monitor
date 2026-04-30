use std::fs; // Filesystem standard library
use std::thread;
use system_monitor::config;


fn main() {
   let mut args = std::env::args();
   let _program = args.next();

   let config = match config::parse_args(args) {
      Ok(config) => config, 
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
        thread::sleep(config.interval);
   }
}

