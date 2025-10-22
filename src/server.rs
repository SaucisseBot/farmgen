use crate::cli::prompt;
use crate::filesystem::expand_tilde;
use std::io;
use std::process::Command;


const SERVER_PATH: &str = "~/pokecity/server";

const TMUX_SESSION: &str = "pokecity";

pub fn maybe_restart() -> io::Result<()> {
    let answer = prompt("\n🔁 Restart Minecraft server now? (y/N): ");
    if answer.eq_ignore_ascii_case("y") {
        restart_server()?;
    } else {
        println!("✅ Server not restarted. Remember to reload in-game with /reload if needed.");
    }
    Ok(())
}


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
