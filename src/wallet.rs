use keys::generator::Generator;
use std::fs;
use keys::{Address, Type, Network};

#[derive(Debug)]
pub struct Wallet {
    path: String,
    pub key: keys::KeyPair,
}

impl Wallet {
    pub fn generate_key(network: &str) -> Result<Self, String> {
        let key_path = "./test-privkey.txt";
        let network_type = match network {
            "mainnet" => keys::Network::Mainnet,
            "testnet" => keys::Network::Testnet,
            _ => return Err("type didn't passed, or wrong word is passed.".to_string()),
        };
        let keypair = keys::generator::Random::new(network_type).generate().unwrap();
 //       if let Err(err) = fs::write(key_path, keypair.private().to_string()) {
        if let Err(err) = fs::write(key_path, "cN2gn1wm6YG9XRpbhsPinQ92ayM6RKW4RKfbgBky5X7LDQoVxj24") {
            return Err(err.to_string());
        }
        println!("keypair: {}", keypair);
        let wallet = Wallet {
            path: key_path.to_string(),
            key: keypair,
        };
        Ok(wallet)
    }

    pub fn open(privkey_path: &str) -> Result<Self, String> {
        let keypair_readed = match fs::read_to_string(privkey_path) {
            Ok(privkey_str) =>  {
                let private: keys::Private = privkey_str.parse().expect("parse failed");
                keys::KeyPair::from_private(private) 
            },
            Err(e) => return Err(e.to_string()),
        };
        let wallet = match keypair_readed {
            Ok(keypair) => Wallet { path: privkey_path.to_string(), key: keypair },
            Err(err) => return Err(err.to_string()),
        };
        Ok(wallet)
    }
    
    pub fn public(&self) -> &keys::Public {
        &self.key.public()
    }

    pub fn from_str(wif: &str) -> Self {
        let private: keys::Private = wif.parse().expect("parse failed");
        build_wallet("", keys::KeyPair::from_private(private).unwrap())
    }
}

fn build_wallet(path: &str, key: keys::KeyPair) -> Wallet {
    Wallet { path: path.to_string(), key}
}

