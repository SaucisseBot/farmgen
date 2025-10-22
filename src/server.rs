use crate::cli::prompt;
use crate::filesystem::expand_tilde;
use std::io;
use std::process::Command;

/// Root server path
const SERVER_PATH: &str = "~/pokecity/server";

/// TMUX session name (adjust if needed)
const TMUX_SESSION: &str = "pokecity";

/// Ask the user if they want to restart the server, then do it
pub fn maybe_restart() -> io::Result<()> {
    let answer = prompt("\n🔁 Restart Minecraft server now? (y/N): ");
    if answer.eq_ignore_ascii_case("y") {
        restart_server()?;
    } else {
        println!("✅ Server not restarted. Remember to reload in-game with /reload if needed.");
    }
    Ok(())
}

/// Executes tmux kill + ./start_server.sh
fn restart_server() -> io::Result<()> {
    let _server_path = expand_tilde(SERVER_PATH);

    println!("🛑 Stopping current tmux session...");
    let _ = Command::new("tmux")
        .args(["kill-session", "-t", TMUX_SESSION])
        .status();

    println!("🚀 Restarting server...");
    let status = Command::new("bash")
        .arg("-c")
        .arg(format!("cd ~ && ./start_server.sh"))
        .status()?;

    if status.success() {
        println!("✅ Server successfully restarted!");
    } else {
        println!("❌ Server restart failed — check your tmux session or script.");
    }

    Ok(())
}
