mod block;
mod blockchain;
mod hash;
mod merkle;
mod pos;
mod pow;

fn main() {
    let mut blockchain = blockchain::Blockchain::new();
    blockchain.build_with_hash_rate(60.0);
}
