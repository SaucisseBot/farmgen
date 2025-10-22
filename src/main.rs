mod cli;
mod datapack;
mod filesystem;
mod server;

use cli::{main_menu, regen_all, delete_all};
use std::io;

fn main() -> io::Result<()> {
    println!("🌍 Minecraft Dimension Manager - Pokecity");
    println!("-----------------------------------------");

    loop {
        match main_menu()? {
            1 => {
                datapack::create_interactive()?;
                server::maybe_restart()?;
            }
            2 => {
                datapack::delete_interactive()?;
                server::maybe_restart()?;
            }
            3 => {
                regen_all()?;
                server::maybe_restart()?;
            }
            4 => {
                delete_all()?;
                server::maybe_restart()?;
            }
            _ => {
                println!("👋 Goodbye!");
                break;
            }
        }
    }

    Ok(())
}
