use risc0_zkvm::guest::env;
use zk_storage_core::storage::Storage;
fn main() {
    let input: Vec<u32> = env::read();
    let must_contain: u32 = env::read();
    let storage = Storage::new_from_vec(input);
    let root = storage.get_root().unwrap();
    assert!(storage.contains(&must_contain));

    // println!("root hash: {:?}", root);

    // write public output to the journal
    env::commit(&root);
}
