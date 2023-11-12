use anyhow::Result;
use bonsai_sdk::alpha as bonsai_sdk;
use methods::{METHOD_ELF, METHOD_ID};
use risc0_zkvm::{serde::to_vec, MemoryImage, Program, Receipt, GUEST_MAX_MEM, PAGE_SIZE};
use std::time::Duration;

fn main() {
    // let client = bonsai_sdk::Client::from_env(risc0_zkvm::VERSION).unwrap();
    let url = "https://api.bonsai.xyz/";
    let key = "f1tDhvOD3nz4S9OnZXTG9GGE5kEUXEpUFMa0yaa0";
    let client =
        bonsai_sdk::Client::from_parts(url.to_string(), key.to_string(), risc0_zkvm::VERSION)
            .unwrap();

    let img_id = {
        let program = Program::load_elf(METHOD_ELF, GUEST_MAX_MEM as u32).unwrap();
        let image = MemoryImage::new(&program, PAGE_SIZE as u32).unwrap();
        let image_id = hex::encode(image.compute_id());
        let image = bincode::serialize(&image).expect("Failed to serialize memory img");
        let is_exist = client.upload_img(&image_id, image);
        println!("is_exist: {:?}", is_exist);
        image_id
    };

    println!("img_id: {}", img_id);
}
