use crate::datapack;
use std::io::{self, Write};

pub fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}


pub fn main_menu() -> io::Result<u8> {
    println!("\nMain Menu:");
    println!("1) Create a new single dimension");
    println!("2) Delete an existing datapack");
    println!("3) Regenerate farm:overworld, farm:nether, farm:end");
    println!("4) Delete farm_worlds datapack");
    println!("5) Exit");
    let choice = prompt("👉 Choice: ");
    Ok(choice.parse::<u8>().unwrap_or(0))
}


pub fn regen_all() -> io::Result<()> {
    println!("\n🔁 Regenerating all farm dimensions in one datapack...");

    let pack_name = "farm_worlds";
    datapack::delete(pack_name)?;

    datapack::create_multi(pack_name)?;

    println!("\n✅ All 3 dimensions regenerated successfully!");
    println!("🌐 Use these commands in-game:");
    println!("   /execute in farm:overworld run tp @s 0 100 0");
    println!("   /execute in farm:nether run tp @s 0 80 0");
    println!("   /execute in farm:end run tp @s 0 80 0");

    Ok(())
}

pub fn delete_all() -> io::Result<()> {
    println!("\n🗑️ Deleting farm_worlds datapack...");
    datapack::delete("farm_worlds")?;
    println!("✅ farm_worlds deleted!");
    Ok(())
}
