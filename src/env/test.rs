use core::cell::RefCell;
extern crate std;
use alloc::vec::Vec;
use alloc::{boxed::Box, rc::Rc};
use bitcoin::{Address, Network};
use core::str::FromStr;

use crate::blockchain::Environment;
use crate::{
    blockchain::AddressHash,
    cursor::Cursor,
    storage::{map::Map, StorageKey, StorageValue},
    ContractTrait,
};

pub struct TestRouter {
    contracts: Vec<(AddressHash, *mut dyn ContractTrait)>,
}
impl TestRouter {
    pub fn new() -> Self {
        Self {
            contracts: Vec::new(),
        }
    }

    pub fn push(&mut self, address: AddressHash, contract: Box<dyn ContractTrait>) {
        let contract = Box::into_raw(contract);
        self.contracts.push((address, contract));
    }

    pub fn call(&self, address: AddressHash, call_data: Cursor) -> Cursor {
        if let Some((_, contract_ptr)) = self.contracts.iter().find(|(addr, _)| address.eq(addr)) {
            unsafe {
                let mut contract: Box<dyn ContractTrait> = Box::from_raw(*contract_ptr);
                let result = contract.execute(call_data).unwrap();
                Box::leak(contract);
                result
            }
        } else {
            panic!("Contract is not present")
        }
    }
}

#[derive(Clone)]
pub struct TestContext {
    pub network: Network,
    pub environment: Environment,
    pub events: RefCell<alloc::vec::Vec<crate::event::Event>>,
    pub global_store: RefCell<Map<StorageKey, StorageValue>>,
    pub cache_store: RefCell<Map<StorageKey, StorageValue>>,
    pub inputs: Vec<crate::blockchain::transaction::Input>,
    pub outputs: Vec<crate::blockchain::transaction::Output>,
    pub router: Option<Rc<RefCell<TestRouter>>>,
}

impl Default for TestContext {
    fn default() -> Self {
        Self {
            network: bitcoin::Network::Bitcoin,
            environment: Environment::default(),
            events: RefCell::new(Vec::new()),
            global_store: RefCell::new(Map::new()),
            cache_store: RefCell::new(Map::new()),
            inputs: Vec::new(),
            outputs: Vec::new(),
            router: None,
        }
    }
}

impl super::Context for TestContext {
    // Just mock, data are passed from different entry point
    fn call_data(&self, size: usize) -> Cursor {
        Cursor::new(size)
    }
    fn environment(&self) -> Environment {
        self.environment.clone()
    }
    fn emit(&self, event: &dyn crate::event::EventTrait) {
        let event = crate::event::Event::new(event.buffer());
        self.events.borrow_mut().push(event);
    }

    fn log(&self, _text: &str) {}

    fn call(
        &self,
        address: &crate::blockchain::AddressHash,
        call_data: crate::cursor::Cursor,
    ) -> Cursor {
        if let Some(router) = &self.router {
            router.borrow().call(address.clone(), call_data)
        } else {
            panic!("There is no router associated with a contract")
        }
    }

    fn deploy_from_address(
        &self,
        from_address: &crate::blockchain::AddressHash,
        salt: [u8; 32],
    ) -> Result<crate::blockchain::AddressHash, crate::error::Error> {
        Ok(*from_address)
    }

    fn encode_address(&self, _address: &str) -> &'static [u8] {
        b"abc"
    }

    fn validate_bitcoin_address(&self, address: &str) -> Result<bool, crate::error::Error> {
        match Address::from_str(address) {
            Ok(addr) => {
                if addr.is_valid_for_network(self.network) {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(_) => Err(crate::error::Error::NoValidAddress),
        }
    }

    fn verify_schnorr_signature(
        &self,
        address: &AddressHash,
        signature: &[u8],
        hash: &[u8],
    ) -> Result<bool, crate::error::Error> {
        let secp: secp256k1::Secp256k1<secp256k1::All> = secp256k1::Secp256k1::new();

        let xonly_public_key = secp256k1::XOnlyPublicKey::from_byte_array(&address.0).unwrap();
        let signature = secp256k1::schnorr::Signature::from_byte_array(
            signature
                .try_into()
                .map_err(|_| crate::error::Error::ConvertError)?,
        );
        let result = secp.verify_schnorr(&signature, &hash, &xonly_public_key);
        Ok(result.is_ok())
    }

    fn load(&self, pointer: &crate::storage::StorageKey) -> Option<crate::storage::StorageValue> {
        if let Some(value) = self.cache_store.borrow().get(pointer) {
            Some(*value)
        } else if let Some(value) = self.global_store.borrow().get(pointer) {
            self.cache_store.borrow_mut().insert(*pointer, *value);
            Some(*value)
        } else {
            None
        }
        .filter(|&value| !StorageValue::ZERO.eq(&value))
    }

    fn store(&self, pointer: crate::storage::StorageKey, value: crate::storage::StorageValue) {
        if if let Some(old) = self.cache_store.borrow().get(&pointer) {
            value.ne(old)
        } else {
            true
        } {
            self.cache_store.borrow_mut().insert(pointer, value);
            self.global_store.borrow_mut().insert(pointer, value);
        }
    }

    fn exists(&self, pointer: &StorageKey) -> bool {
        if self.cache_store.borrow().contains_key(pointer) {
            true
        } else if let Some(value) = self.global_store.borrow().get(pointer) {
            self.cache_store.borrow_mut().insert(*pointer, *value);
            true
        } else {
            false
        }
    }

    fn inputs(&self) -> Vec<crate::blockchain::transaction::Input> {
        self.inputs.clone()
    }

    fn outputs(&self) -> Vec<crate::blockchain::transaction::Output> {
        self.outputs.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::TestContext;
    use crate::Context;

    #[test]
    fn test_validate_mainnet_address() {
        let context = TestContext::default();
        assert_eq!(
            true,
            context
                .validate_bitcoin_address("bc1qnghhhgvz5cn8n6x2fy06yzvkuermcm5ljn06gw")
                .unwrap()
        );
    }

    #[test]
    fn test_validate_testnet_address() {
        let context = TestContext {
            network: bitcoin::Network::Testnet,
            ..Default::default()
        };
        assert_eq!(
            true,
            context
                .validate_bitcoin_address("mym4vP87LdQp9YzRbggpS46fYiQFfR52Nq")
                .unwrap()
        );
    }

    #[test]
    fn test_validate_p2sh_mainnet_address() {
        let context = TestContext {
            network: bitcoin::Network::Bitcoin,
            ..Default::default()
        };
        assert_eq!(
            true,
            context
                .validate_bitcoin_address("3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy")
                .unwrap()
        );
    }

    #[test]
    fn test_invalid_address_format() {
        let context = TestContext {
            network: bitcoin::Network::Bitcoin,
            ..Default::default()
        };
        assert_eq!(
            false,
            context.validate_bitcoin_address("invalid_address").unwrap()
        );
    }

    #[test]
    fn test_address_valid_but_wrong_network() {
        let context = TestContext {
            network: bitcoin::Network::Bitcoin,
            ..Default::default()
        };
        assert_eq!(
            false,
            context
                .validate_bitcoin_address("mym4vP87LdQp9YzRbggpS46fYiQFfR52Nq")
                .unwrap()
        );
    }

    #[test]
    fn test_valid_regtest_address() {
        let context = TestContext {
            network: bitcoin::Network::Regtest,
            ..Default::default()
        };
        assert_eq!(
            true,
            context
                .validate_bitcoin_address(
                    "bcrt1pe0slk2klsxckhf90hvu8g0688rxt9qts6thuxk3u4ymxeejw53gs0xjlhn"
                )
                .unwrap()
        );
    }

    #[test]
    fn test_valid_regtest_address_segwit() {
        let context = TestContext {
            network: bitcoin::Network::Regtest,
            ..Default::default()
        };
        assert_eq!(
            true,
            context
                .validate_bitcoin_address("bcrt1qfqsr3m7vjxheghcvw4ks0fryqxfq8qzjf8fxes")
                .unwrap()
        );
    }

    #[test]
    fn test_valid_regtest_address_legacy() {
        let context = TestContext {
            network: bitcoin::Network::Regtest,
            ..Default::default()
        };
        assert_eq!(
            true,
            context
                .validate_bitcoin_address("mn6KYibk94NhScakhgVPQdGE1bnscugRDG")
                .unwrap()
        );
    }

    #[test]
    fn test_valid_regtest_address_p2sh() {
        let context = TestContext {
            network: bitcoin::Network::Regtest,
            ..Default::default()
        };
        assert_eq!(
            true,
            context
                .validate_bitcoin_address("2MyLLEUGJSusHvPDNHTwYnG9FAJcrQ3VPZY")
                .unwrap()
        );
    }

    #[test]
    fn test_valid_schnnor_signature() {
        use secp256k1::hashes::{sha256, Hash};
        use secp256k1::rand::rngs::OsRng;
        let context = TestContext::default();

        let secp: secp256k1::Secp256k1<secp256k1::All> = secp256k1::Secp256k1::new();
        let (secret_key, public_key) = secp256k1::Secp256k1::new().generate_keypair(&mut OsRng);
        let keypair = secp256k1::Keypair::from_secret_key(&secp, &secret_key);
        let msg = sha256::Hash::hash("Hello World!".as_bytes());

        // With random data
        let signature = secp.sign_schnorr_with_rng(msg.as_byte_array(), &keypair, &mut OsRng);
        assert!(secp
            .verify_schnorr(&signature, msg.as_byte_array(), &public_key.into())
            .is_ok());
        assert!(context
            .verify_schnorr_signature(
                &crate::blockchain::AddressHash(public_key.x_only_public_key().0.serialize()),
                signature.as_byte_array(),
                msg.as_byte_array()
            )
            .is_ok());

        // NO random data
        let signature = secp.sign_schnorr_no_aux_rand(msg.as_byte_array(), &keypair);
        assert!(secp
            .verify_schnorr(&signature, msg.as_byte_array(), &public_key.into())
            .is_ok());
        assert!(context
            .verify_schnorr_signature(
                &crate::blockchain::AddressHash(public_key.x_only_public_key().0.serialize()),
                signature.as_byte_array(),
                msg.as_byte_array()
            )
            .is_ok());

        // Custom random data
        let signature = secp.sign_schnorr_with_aux_rand(msg.as_byte_array(), &keypair, &[1; 32]);
        assert!(secp
            .verify_schnorr(&signature, msg.as_byte_array(), &public_key.into())
            .is_ok());
        assert!(context
            .verify_schnorr_signature(
                &crate::blockchain::AddressHash(public_key.x_only_public_key().0.serialize()),
                signature.as_byte_array(),
                msg.as_byte_array()
            )
            .is_ok());
    }
}
