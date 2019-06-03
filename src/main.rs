mod wallet;
mod transaction;

use crate::wallet::Wallet;
use script::{UnsignedTransactionInput, TransactionInputSigner, SignatureVersion};
use serialization::{Reader, Deserializable};
use primitives::bytes::Bytes;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let prev_output_index_string = &args[2];
    let prev_tx_string = &args[3];
    let send_to_address = &args[4];
    let amount = &args[5];

    println!("Output Index: {}", prev_output_index_string);
    println!("Previous Transaction: {}", prev_tx_string);
    println!("Send to: {}", send_to_address);
    println!("Amount: {}", amount);

    let parity_bytes: Bytes = prev_tx_string.parse().unwrap();
    let mut reader = Reader::new(&parity_bytes);

    let prev_tx = chain::Transaction::deserialize(&mut reader).unwrap();
    // let prev_output = prev_tx.outputs.get(0).unwrap();

    let wallet = Wallet::open("privkey.txt").unwrap();

    // let send_to: keys::Address = send_to_address.parse().unwrap();

    // let p2pkh_script = script::Builder::build_p2pkh(&send_to.hash);
    // let output = create_transaction::output(
    //     amount.parse::<u64>().unwrap(),
    //     p2pkh_script.to_bytes()
    // );
    // let prevout_hash = prev_tx.hash();
    // let outpoint = chain::OutPoint{hash: prevout_hash, index: prev_output_index_string.parse().unwrap()};
    
    // let input = create_transaction::input(
    //     outpoint,
    //     prev_output.script_pubkey.clone()
    // );

    // sign input
    // let raw_tx = create_transaction::create_signed_transaction(
    //     vec![input],
    //     vec![output],
    //     &wallet.key,
    //     prev_output.value,
    //     prev_output_index_string.parse::<usize>().unwrap()
    // );
    let raw_tx = transaction::prepare_tx(prev_tx, prev_output_index_string, amount, send_to_address, wallet);
    println!("Raw Transaction: {:?}", raw_tx);

    let hex_tx = transaction::hex_tx(raw_tx);
    println!("Hex Transaction: {:?}", hex_tx);

    Ok(())
}
