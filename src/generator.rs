use rand::prelude::*; // for RNG
use image::{DynamicImage, GenericImage, GenericImageView}; // image manipulation
use std::path::Path;

#[derive(Clone, Debug)]
pub enum Rarity {
    Common,
    Rare,
    Epic,
    Legendary,
}

#[derive(Clone, Debug)]
pub struct TraitOption {
    pub name: String,
    pub rarity: Rarity,
    pub file_path: String, // Path to the image for this trait
}

#[derive(Clone, Debug)]
pub struct GeneratedNFT {
    pub background: String,
    pub head: String,
    pub eyes: String,
    pub mouth: String,
    pub accessory: String,
}

pub fn select_trait(traits: &[TraitOption]) -> TraitOption {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..traits.len());
    traits[index].clone()
}

pub fn generate_nft() -> GeneratedNFT {
    // Define traits with placeholder image paths
    let backgrounds = vec![
        TraitOption { name: "Cyber Grid".to_string(), rarity: Rarity::Epic, file_path: "images/background/cyber_grid.png".to_string() },
        TraitOption { name: "Neon City".to_string(), rarity: Rarity::Rare, file_path: "images/background/neon_city.png".to_string() },
    ];

    let heads = vec![
        TraitOption { name: "Neon Circuit".to_string(), rarity: Rarity::Rare, file_path: "images/head/neon_circuit.png".to_string() },
        TraitOption { name: "Quantum Lens".to_string(), rarity: Rarity::Epic, file_path: "images/head/quantum_lens.png".to_string() },
    ];

    let eyes = vec![
        TraitOption { name: "Glowing Red".to_string(), rarity: Rarity::Epic, file_path: "images/eyes/glowing_red.png".to_string() },
        TraitOption { name: "Laser Blue".to_string(), rarity: Rarity::Rare, file_path: "images/eyes/laser_blue.png".to_string() },
    ];

    let mouths = vec![
        TraitOption { name: "Neutral".to_string(), rarity: Rarity::Common, file_path: "images/mouth/neutral.png".to_string() },
        TraitOption { name: "Smile".to_string(), rarity: Rarity::Rare, file_path: "images/mouth/smile.png".to_string() },
    ];

    let accessories = vec![
        TraitOption { name: "Holo Shield".to_string(), rarity: Rarity::Epic, file_path: "images/accessory/holo_shield.png".to_string() },
        TraitOption { name: "Cyber Cape".to_string(), rarity: Rarity::Legendary, file_path: "images/accessory/cyber_cape.png".to_string() },
    ];

    let bg = select_trait(&backgrounds);
    let head = select_trait(&heads);
    let eye = select_trait(&eyes);
    let mouth = select_trait(&mouths);
    let accessory = select_trait(&accessories);

    // Attempt to combine images if they exist
    let image_paths = vec![
        &bg.file_path,
        &head.file_path,
        &eye.file_path,
        &mouth.file_path,
        &accessory.file_path,
    ];

    if let Some(first_path) = image_paths.first() {
        if Path::new(first_path).exists() {
            let mut base = image::open(first_path).unwrap();
            for layer_path in &image_paths[1..] {
                if Path::new(layer_path).exists() {
                    let layer = image::open(layer_path).unwrap();
                    image::imageops::overlay(&mut base, &layer, 0, 0);
                }
            }
            // Save the final NFT image
            base.save("generated_nft.png").unwrap();
            println!("NFT image saved as generated_nft.png");
        } else {
            println!("Image paths do not exist yet. Traits selected will be printed.");
        }
    }

    GeneratedNFT {
        background: bg.name,
        head: head.name,
        eyes: eye.name,
        mouth: mouth.name,
        accessory: accessory.name,
    }
}

fn main() {
    let nft = generate_nft();
    println!("NFT generated: {:?}", nft);
}
