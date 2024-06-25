use methods::{GUEST_ELF};
use risc0_zkvm::{default_prover, ExecutorEnv};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    const MUST_CONTAIN: u32 = 1234;
    let mut input: Vec<u32> = (0..10).map(|_| rand::random::<u32>()).collect();
    input.push(MUST_CONTAIN);
    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .write(&MUST_CONTAIN)
        .unwrap()
        .build()
        .unwrap();

    let prover = default_prover();

    let prove_info = prover.prove(env, GUEST_ELF).unwrap();
    let receipt = prove_info.receipt;
    let output: [u8; 32] = receipt.journal.decode().unwrap();

    println!(
        "The storage contains value {}. Root hash is: {:?}",
        MUST_CONTAIN,
        output
    );

    let serialized = bincode::serialize(&receipt).unwrap();

		// Writing the serialized contect to receipt.bin file
    let _saved_file = match std::fs::write("./receipt.bin", serialized){
         Ok(()) => println!("Receipt saved and serialized as receipt.bin"),
         Err(err) => println!("Proof serialization failed: {:?}", err),
    };
}
