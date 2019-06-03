use super::wallet;

use rpc::v1::types::{TransactionOutputs, TransactionOutputWithAddress};
use rpc::v1::types::H256;
use primitives::bytes::Bytes;
use primitives::hash::H512;
use chain::Transaction as GlobalTransaction;
use chain::{TransactionInput, TransactionOutput, OutPoint};
use primitives::hash::H256 as GlobalH256;
use serialization::{Reader, serialize, deserialize};
use script::{TransactionInputSigner, UnsignedTransactionInput, SignatureVersion, TransactionSignatureChecker, SignatureChecker, Script};
use keys::Signature;

pub fn output(value: u64, script_pubkey: Bytes) -> TransactionOutput {
	TransactionOutput{ 
        value,
        script_pubkey
    }
}

pub fn input(previous_output: OutPoint, prev_script_pubkey: Bytes) -> TransactionInput {
    TransactionInput{
        previous_output,
        script_sig: prev_script_pubkey, 
        sequence: 4294967295,
        script_witness: Vec::new()
    }
}

pub fn hex_tx(raw_transaction: GlobalTransaction) -> primitives::bytes::Bytes {
    let transaction = serialize(&raw_transaction);
	transaction
}

pub fn prepare_tx(prev_tx: chain::Transaction, prev_output_index_string: &str, amount: &str, send_to_address: &str, wallet: wallet::Wallet) -> chain::Transaction {
    // input
    let prev_output = prev_tx.outputs.get(0).unwrap();
    let prevout_hash = prev_tx.hash();
    let outpoint = chain::OutPoint{hash: prevout_hash, index: prev_output_index_string.parse().unwrap()};
    let input = input(
        outpoint,
        prev_output.script_pubkey.clone()
    );

    // output
    let send_to: keys::Address = send_to_address.parse().unwrap();
    let p2pkh_script = script::Builder::build_p2pkh(&send_to.hash);
    let output = output(
        amount.parse::<u64>().unwrap(),
        p2pkh_script.to_bytes()
    );
    // key
    // value

    let tx = create_signed_transaction(
        vec![input],
        vec![output],
        &wallet.key,
        prev_output.value,
        prev_output_index_string.parse::<usize>().unwrap()
    );
    tx
}

pub fn create_signed_transaction(inputs: Vec<TransactionInput>, outputs: Vec<TransactionOutput>, key: &keys::KeyPair, input_value: u64, prev_output_index: usize) -> chain::Transaction {
    // unsigned transaction
	let input = inputs[0].clone();
	let output = outputs[0].clone();
    let script = input.clone().script_sig.into();
    
    let unsigned_transaction: UnsignedTransactionInput = input.into();

    // transaction input signer
    let signer = TransactionInputSigner{
        version: 1,
        inputs: vec![unsigned_transaction],
        outputs: vec![output],
        lock_time: 0
    };

    // signed input
    /// .push_data(keypair.public())したinputをつくってしまっている。
    /// これはp2pkhのしかつくれない？
    let signed_input = signer.signed_input(
        key,
        prev_output_index,
        input_value,
        &script,
        SignatureVersion::Base,
        1 
    );

    // let script_bytes = signed_input.script_sig;
    // let script: Script = script.into();
    // println!{"script: {:?}", script.script_type()};

    // std::process::exit(0x0100);

    // build transaction
    // parity-bitcoin has no function to build signed transaction
    let signed_inputs = vec![signed_input];    
    //let raw_tx = create_raw_transaction(signed_inputs, outputs);
    let raw_tx = chain::Transaction{
        version: 1,
        inputs: signed_inputs,
        outputs: outputs,
        lock_time: 0
    };
    
    raw_tx
}

#[cfg(test)]
mod tests {
    #[test]
    fn create_signed_transaction() {
        use super::*;
        use super::super::Wallet;

        let prev_tx: chain::Transaction = "0100000001ea1cbd6ca39069c3db42be1e5592d1b36e2c87d6f1bf2d4d4ac4184f2104bd11010000006b483045022100fe7991c981c770caf58fd9d1b7df7e46d36a5316c6cc10cec2ee6b02f54dec4302205a4e776f42b421a32e0120f736f9b21dab7bc1e4b87514e33110c6300bb63f580121039a189d7da053d3e9642de6f9e11a888b8b5dc620892e7a3951716c7b8225f04affffffff01804a5d05000000001976a914a9f24bf3c64c549b326d21e08282e7065a91828788ac00000000".into();
        let this_tx: chain::Transaction = "01000000012a7687179a794800a4a12dbce26343cd9e7eff4294a81d8648a449b32bb4ed45000000006b483045022100db7bb29c91ef1b96269f04ae1c99f5e768b7601bcb2eab59ebf884fd23385db5022035195bf0661bdf1800084e191b513bce56ecd1aa042879430714024e43fe4fc90121031b5432cf28e91328dc43acb2d43164272bd977016ff149b2e2454cf5f371776cffffffff0100b4c404000000001976a9141883a28a847a4417ef771f43cfcdaa6a31d2753888ac00000000".into();
    
        let sender_wallet = Wallet::from_str("cUktzwKkuzcy4Ddd9HBG2VtX4kmsfKNjLmvmVa4NKyotFfMM5Ypa");

        let recipient_wallet = Wallet::from_str("cN2gn1wm6YG9XRpbhsPinQ92ayM6RKW4RKfbgBky5X7LDQoVxj24");
        let address_hash = recipient_wallet.public().address_hash();

        // script_pubkey to cN2gn1wm6YG9XRpbhsPinQ92ayM6RKW4RKfbgBky5X7LDQoVxj24
        let p2pkh_script = script::Builder::build_p2pkh(&address_hash);
        let output = output(80000000, p2pkh_script.to_bytes());

        let prevout_hash = prev_tx.hash();
        let outpoint = chain::OutPoint{hash: prevout_hash, index: 0};
        let prev_script_pubkey = prev_tx.outputs.get(0).unwrap().script_pubkey.clone();
        
        let input = input(
            outpoint,
            prev_script_pubkey
        );

        // sign input
        // input value and prev_output_index
        let signed_tx = create_signed_transaction(
            vec![input],
            vec![output],
            &sender_wallet.key,
            90000000,
            0
        );

        // check signature
        let sig_checker = TransactionSignatureChecker{
            signer: prev_tx.clone().into(),
            input_index: 0,
            input_amount: 90000000
        };
        let script_sig = signed_tx.inputs[0].clone().script_sig.take();
        let script_pubkey = prev_tx.outputs[0].clone().script_pubkey;
        let pubkey = sender_wallet.public();

        // message is target of signature
        // script_code is script_pubkey
        let result = sig_checker.check_signature(
            &script_sig.into(),
            &pubkey,
            &script_pubkey.into(),
            1,
            SignatureVersion::Base
        );
                                                 
        assert_eq!(signed_tx.outputs(), this_tx.outputs());
        assert_eq!(signed_tx.inputs()[0].previous_output, this_tx.inputs()[0].previous_output);
        assert_eq!(signed_tx.inputs()[0].sequence, this_tx.inputs()[0].sequence);
        // assert_eq!(result, true);
//        assert_eq!(raw_tx, this_tx);
    }

}
