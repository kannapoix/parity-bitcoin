use script::Builder;
use keys::Public;

pub fn build_script(address_hash: &keys::AddressHash) {
    let script = Builder::build_p2pkh(address_hash);

    println!("{:?}", script.to_bytes());
}
