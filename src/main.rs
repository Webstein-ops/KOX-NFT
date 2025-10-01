mod generator;

fn main() {
    let nft = generator::generate_nft();
    println!("NFT generated: {:?}", nft);
}
