use crate::cli::prompt;
use crate::filesystem::{expand_tilde, remove_dir_if_exists};
use serde_json::json;
use std::fs;
use std::io;

const SERVER_PATH: &str = "~/pokecity/server";

/// Interactive creation of a datapack
pub fn create_interactive() -> io::Result<()> {
    let pack_name = prompt("Datapack name (e.g. farmworld): ");
    let dim_name = prompt("Internal dimension name (e.g. farm): ");

    println!("\nSelect dimension type:");
    println!("1) Overworld");
    println!("2) Nether");
    println!("3) End");
    let choice = prompt("👉 Choice (1/2/3): ");

    let dim_type = match choice.as_str() {
        "2" => "nether",
        "3" => "end",
        _ => "overworld",
    };

    create(dim_type, &pack_name, &dim_name)
}

/// Create a datapack of a given type
pub fn create(dim_type: &str, pack_name: &str, dim_name: &str) -> io::Result<()> {
    let (t, settings, infiniburn, effects) = match dim_type {
        "nether" => (
            "minecraft:the_nether",
            "minecraft:nether",
            "minecraft:infiniburn_nether",
            "minecraft:the_nether",
        ),
        "end" => (
            "minecraft:the_end",
            "minecraft:end",
            "minecraft:infiniburn_end",
            "minecraft:the_end",
        ),
        _ => (
            "minecraft:overworld",
            "minecraft:overworld",
            "minecraft:infiniburn_overworld",
            "minecraft:overworld",
        ),
    };

    let seed_input = prompt("Seed (leave empty for random): ");
    let seed: i64 = if seed_input.is_empty() {
        rand::random::<i64>()
    } else {
        seed_input.parse().unwrap_or(0)
    };

    let biome_source = match dim_type {
        "nether" => json!({
            "type": "minecraft:multi_noise",
            "preset": "minecraft:nether"
        }),
        "end" => json!({
            "type": "minecraft:the_end"
        }),
        _ => json!({
            "type": "minecraft:multi_noise",
            "preset": "minecraft:overworld"
        }),
    };

    let dimension_json = json!({
        "type": t,
        "generator": {
            "type": "minecraft:noise",
            "seed": seed,
            "settings": settings,
            "biome_source": biome_source
        },
        "infiniburn": infiniburn,
        "effects": effects,
        "ambient_light": 0.0
    });

    let base_path = expand_tilde(&format!("{}/world/datapacks/{}/", SERVER_PATH, pack_name));
    let dim_path = base_path.join(format!("data/{}/dimension", dim_name));

    if base_path.exists() {
        let confirm = prompt("⚠️ Datapack already exists. Overwrite? (y/N): ");
        if !confirm.eq_ignore_ascii_case("y") {
            println!("❌ Cancelled.");
            return Ok(());
        }
        fs::remove_dir_all(&base_path)?;
    }

    fs::create_dir_all(&dim_path)?;

    let pack_mcmeta = json!({
        "pack": {
            "pack_format": 48,
            "description": format!("Pokecity {} dimension: {}", dim_type, dim_name)
        }
    });
    fs::write(
        base_path.join("pack.mcmeta"),
        serde_json::to_string_pretty(&pack_mcmeta)?,
    )?;

    fs::write(
        dim_path.join(format!("{}.json", dim_name)),
        serde_json::to_string_pretty(&dimension_json)?,
    )?;

    println!("✅ Created datapack '{}'", pack_name);
    Ok(())
}
/// Create a datapack with a custom namespace (used for multi-world farm regen)

/// Interactive deletion
pub fn delete_interactive() -> io::Result<()> {
    let pack_name = prompt("Datapack to delete: ");
    delete(&pack_name)
}

/// Delete datapack by name
pub fn delete(pack_name: &str) -> io::Result<()> {
    let base_path = expand_tilde(&format!("{}/world/datapacks/{}/", SERVER_PATH, pack_name));
    remove_dir_if_exists(&base_path)?;
    Ok(())
}
/// Create a datapack containing 3 dimensions: farm:overworld, farm:nether, farm:end
pub fn create_multi(pack_name: &str) -> io::Result<()> {
    let base_path = expand_tilde(&format!("{}/world/datapacks/{}/", SERVER_PATH, pack_name));
    let dim_path = base_path.join("data/farm/dimension");
    fs::create_dir_all(&dim_path)?;

    // -- 1️⃣ pack.mcmeta --
    let pack_mcmeta = json!({
        "pack": {
            "pack_format": 48,
            "description": "Pokecity Farm Worlds (overworld/nether/end)"
        }
    });
    fs::write(
        base_path.join("pack.mcmeta"),
        serde_json::to_string_pretty(&pack_mcmeta)?,
    )?;

    // -- 2️⃣ Overworld --
    let overworld_json = json!({
        "type": "minecraft:overworld",
        "generator": {
            "type": "minecraft:noise",
            "seed": rand::random::<i64>(),
            "settings": "minecraft:overworld",
            "biome_source": {
                "type": "minecraft:multi_noise",
                "preset": "minecraft:overworld"
            }
        },
        "infiniburn": "minecraft:infiniburn_overworld",
        "effects": "minecraft:overworld",
        "ambient_light": 0.0
    });
    fs::write(
        dim_path.join("overworld.json"),
        serde_json::to_string_pretty(&overworld_json)?,
    )?;

    // -- 3️⃣ Nether --
    let nether_json = json!({
        "type": "minecraft:the_nether",
        "generator": {
            "type": "minecraft:noise",
            "seed": rand::random::<i64>(),
            "settings": "minecraft:nether",
            "biome_source": {
                "type": "minecraft:multi_noise",
                "preset": "minecraft:nether"
            }
        },
        "infiniburn": "minecraft:infiniburn_nether",
        "effects": "minecraft:the_nether",
        "ambient_light": 0.1
    });
    fs::write(
        dim_path.join("nether.json"),
        serde_json::to_string_pretty(&nether_json)?,
    )?;
    // -- 4️⃣ End --
    let end_json = json!({
        "type": "minecraft:the_end",
        "generator": {
            "type": "minecraft:noise",
            "seed": rand::random::<i64>(),
            "settings": "minecraft:end",
            "biome_source": {
                "type": "minecraft:the_end"
            }
        },
        "infiniburn": "minecraft:infiniburn_end",
        "effects": "minecraft:the_end",
        "ambient_light": 0.0
    });
    fs::write(
        dim_path.join("end.json"),
        serde_json::to_string_pretty(&end_json)?,
    )?;

    println!(
        "✅ Created datapack '{}' with farm:overworld, farm:nether, farm:end",
        pack_name
    );
    Ok(())
}
