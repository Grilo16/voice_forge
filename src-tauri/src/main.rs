// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


#[path = "db/database.rs"]
mod database;
#[path = "db/repositories/flag_repo.rs"]
mod flag_repo;
#[path = "db/repositories/ssh_cred_repo.rs"]
mod ssh_cred_repo;
#[path = "./db/models/flag.rs"]
mod flag;
#[path = "ssh_session/terminal_handler.rs"]
mod terminal_handler;
#[path = "ssh_session/session_handler.rs"]
mod session_handler;
#[path = "db/models/ssh_credentials.rs"]
mod ssh_credentials;



use database::Database;
use terminal_handler::run_launch_machine;
use session_handler::SshSession;

use flag::Flag;
// use flag_repo::FlagsRepo;

use ssh_credentials::SshCredentials;


// 0Sh1g0t0! jays pc pass
#[tauri::command]
fn launch_cloud_client() -> String{
    
    let ssh_credentials = match run_launch_machine() {
        Ok(output) => output,
        Err(err) => return format!("Error: {}", err),
    };
    
    let mut ssh_session = match SshSession::new(&ssh_credentials) {
        Ok(ssh_session) => ssh_session,
        Err(err) => return format!("Error creating SSH session: {}", err),
    };

    let command = "tmux new-session -d -s my_session; source ~/puffin_env/bin/activate; python3 ~/spun/repos/speedy/script/run.py -i Asfas -d 2021-14 -n 99 --accent London --donorid Anything --donorvb --dry".to_string();
    
    match ssh_session.execute_command(&command) {
        Ok(output) => format!("Command output: {}", output),
        Err(err) => format!("Error executing SSH command: {}", err),
    }
}



fn main() {
    if let Ok(db) = Database::open() {
        if let Err(err) = db.create_tables() {
            eprintln!("Error creating tables: {:?}", err);
            return;
        }
        tauri::Builder::default()
            .invoke_handler(tauri::generate_handler![
                launch_cloud_client
            ])
            .run(tauri::generate_context!())
            .expect("Error while running Tauri application");
    } else {
        eprintln!("Error opening the database");
    }
}
