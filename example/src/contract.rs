use alloc::rc::Rc;
use core::{cell::RefCell, ops::Add};
use rust_runtime::{
    blockchain::{AddressHash, Environment},
    contract::op_20::Pointer,
    ethnum::u256,
    math::abi::encode_selector_const,
    storage::{
        map::Map,
        multi_address_map::MultiAddressMemoryMap,
        stored::{StoredTrait, StoredU256, StoredU8},
        stored_map::{StoredAddresValueMap, StoredMap},
        StorageValue,
    },
    types::{CallData, Selector},
    Context, ContractTrait, OP20Trait,
};

const SELECTOR_AIRDROP: Selector = encode_selector_const("airdrop(address)");
const SELECTOR_AIRDROP_WITH_AMOUNT: Selector =
    encode_selector_const("airdropWithAmount(address, address[]");
const SELECTOR_MINT: Selector = encode_selector_const("mint(address,uint256)");

pub struct Contract {
    environment: Option<rust_runtime::blockchain::Environment>,
    params: rust_runtime::contract::op_20::OP20Params,
    nonce_map: StoredMap<AddressHash, u256>,
    balance_of_map: StoredMap<AddressHash, u256>,
    allowance_map: MultiAddressMemoryMap,
    total_supply: StoredU256,
    context: Rc<dyn Context>,
}
impl Contract {
    pub fn new(context: Rc<dyn Context>) -> Self {
        Self {
            environment: None,
            params: rust_runtime::contract::op_20::OP20Params {
                max_supply: StoredU256::new_const(
                    context.clone(),
                    Pointer::MaxSupply.u16(),
                    u256::new(100000000000000000000000000),
                ),
                decimals: StoredU8::new_const(context.clone(), Pointer::Decimals.u16(), 18),
                name: "MyToken",
                symbol: "TOKEN",
            },
            nonce_map: StoredMap::new(context.clone(), Pointer::NonceMap.u16(), u256::ZERO),
            balance_of_map: StoredMap::new(
                context.clone(),
                Pointer::BalanceOfMap.u16(),
                u256::ZERO,
            ),
            allowance_map: MultiAddressMemoryMap::new(
                context.clone(),
                Pointer::AllowanceMap.u16(),
                StorageValue::ZERO,
            ),
            total_supply: StoredU256::new_const(
                context.clone(),
                Pointer::TotalSupply.u16(),
                u256::ZERO,
            ),
            context,
        }
    }
}

impl Contract {
    fn execute(
        &mut self,
        selector: Selector,
        mut call_data: CallData,
    ) -> Result<rust_runtime::cursor::Cursor, rust_runtime::error::Error> {
        match selector {
            SELECTOR_MINT => {
                let address = call_data.read_address()?;
                let amount = call_data.read_u256(true)?;

                self.mint(address, amount)
            }
            SELECTOR_AIRDROP => {
                let drops = call_data.read_address_value_map()?;

                self.airdrop(drops)
            }
            SELECTOR_AIRDROP_WITH_AMOUNT => self.airdrop_with_amount(call_data),
            _ => OP20Trait::execute_base(self, selector, call_data),
        }
    }

    fn mint(
        &mut self,
        address: AddressHash,
        amount: u256,
    ) -> Result<rust_runtime::cursor::Cursor, rust_runtime::error::Error> {
        let caller = self.environment().caller;
        self.only_deployer(&caller)?;

        let mut result = rust_runtime::cursor::Cursor::new(1);

        result.write_bool(self.mint_base(&address, amount, false)?)?;

        return Ok(result);
    }

    fn airdrop(
        &mut self,
        drops: Map<AddressHash, u256>,
    ) -> Result<rust_runtime::cursor::Cursor, rust_runtime::error::Error> {
        let sender = self.environment().caller;
        self.only_deployer(&sender)?;

        for (address, amount) in drops.iter() {
            self.mint_base(address, amount.clone(), false)?;
        }

        let mut cursor = rust_runtime::cursor::Cursor::new(1);
        cursor.write_bool(true)?;

        Ok(cursor)
    }

    fn optimized_mint(
        &mut self,
        address: AddressHash,
        amount: u256,
    ) -> Result<(), rust_runtime::error::Error> {
        self.balance_of_map.set(&address, amount);
        let value = self.total_supply.value() + amount;
        self.total_supply.set_no_commit(value);

        self.create_mint_event(address, amount)?;

        Ok(())
    }

    fn airdrop_with_amount(
        &mut self,
        mut call_data: CallData,
    ) -> Result<rust_runtime::cursor::Cursor, rust_runtime::error::Error> {
        let sender = self.environment().caller;
        self.only_deployer(&sender)?;
        let amount = call_data.read_u256(true)?;
        let amount_of_addresses: u32 = call_data.read_u32(true)?;

        for _ in 0..amount_of_addresses {
            let address = call_data.read_address()?;
            self.optimized_mint(address, amount)?;
        }

        self.total_supply.commit();

        let mut cursor = rust_runtime::cursor::Cursor::new(1);
        cursor.write_bool(true)?;
        Ok(cursor)
    }
}

impl rust_runtime::contract::op_20::OP20Trait for Contract {
    fn params(&mut self) -> &mut rust_runtime::OP20Params {
        &mut self.params
    }

    fn total_supply(&mut self) -> &mut StoredU256 {
        &mut self.total_supply
    }

    fn allowance_map(&mut self) -> &mut MultiAddressMemoryMap {
        &mut self.allowance_map
    }

    fn balance_of_map(&mut self) -> &mut StoredMap<AddressHash, u256> {
        &mut self.balance_of_map
    }

    fn nonce_map(&mut self) -> &mut StoredMap<AddressHash, u256> {
        &mut self.nonce_map
    }
}

impl rust_runtime::contract::ContractTrait for Contract {
    fn environment(&mut self) -> &Environment {
        if self.environment.is_none() {
            self.environment = Some(self.context().environment());
        }
        self.environment.as_ref().unwrap()
    }

    fn context(&self) -> Rc<dyn rust_runtime::env::Context> {
        self.context.clone()
    }

    fn execute(
        &mut self,
        mut call_data: CallData,
    ) -> Result<rust_runtime::cursor::Cursor, rust_runtime::error::Error> {
        let selector = call_data.read_selector()?;

        Contract::execute(self, selector, call_data)
    }

    fn on_deploy(&mut self, _call_data: CallData) {}
}

// To run the tests, run `cargo test -p example` in the root of the workspace
#[cfg(test)]
mod tests {
    use core::cell::RefCell;

    use crate::contract::SELECTOR_MINT;
    use alloc::rc::Rc;
    use rust_runtime::{
        contract::op_20::{
            SELECTOR_BALANCE_OF, SELECTOR_NAME, SELECTOR_SYMBOL, SELECTOR_TOTAL_SUPPLY,
        },
        cursor::Cursor,
        ethnum::u256,
        global::call,
        tests::{execute, execute_address, execute_address_amount},
        OP20Trait,
    };

    fn context() -> Rc<rust_runtime::env::TestContext> {
        Rc::new(rust_runtime::env::TestContext::default())
    }

    #[test]
    fn test_contract_name() {
        let context = context();
        let mut contract = super::Contract::new(context);
        let mut result = contract.execute(SELECTOR_NAME, Cursor::new(0)).unwrap();
        assert_eq!(contract.params.name, result.read_string_with_len().unwrap());
    }

    #[test]
    fn test_contract_symbol() {
        let context = context();
        let mut contract = super::Contract::new(context);
        let mut result = contract.execute(SELECTOR_SYMBOL, Cursor::new(0)).unwrap();
        assert_eq!(
            contract.params.symbol,
            result.read_string_with_len().unwrap()
        );
    }

    #[test]
    fn test_contract_mint() {
        let router = Rc::new(RefCell::new(rust_runtime::env::TestRouter::new()));

        let contract_address = rust_runtime::tests::random_address();
        let address = rust_runtime::tests::random_address();
        let amount = u256::new(10000000);
        let environment = rust_runtime::blockchain::Environment {
            contract_deployer: address.clone(),
            contract_address,
            caller: address.clone(),
            ..Default::default()
        };
        let context = rust_runtime::env::TestContext {
            environment,
            ..Default::default()
        };

        router.borrow_mut().push(
            contract_address,
            alloc::boxed::Box::new(super::Contract::new(Rc::new(context.clone()))),
        );

        let mut call_data = Cursor::new(68);
        call_data.write_selector(&SELECTOR_MINT).unwrap();
        call_data.write_address(&address).unwrap();
        call_data.write_u256(&amount, true).unwrap();

        let mut result = router.borrow().call(contract_address, call_data);
        assert_eq!(result.read_bool().unwrap(), true);

        let mut call_data = Cursor::new(36);
        call_data.write_selector(&SELECTOR_BALANCE_OF).unwrap();
        call_data.write_address(&address).unwrap();

        let mut result = router.borrow_mut().call(contract_address, call_data);
        assert_eq!(result.read_u256(true).unwrap(), amount);
    }

    #[test]
    fn test_contract_total_supply() {
        let context = context();
        let mut contract = super::Contract::new(context);

        let address = rust_runtime::tests::random_address();

        contract.environment = Some(rust_runtime::blockchain::Environment {
            contract_deployer: address.clone(),
            caller: address.clone(),
            ..Default::default()
        });
        let amount = u256::new(10000000);

        let mut result = contract
            .execute(SELECTOR_TOTAL_SUPPLY, Cursor::new(0))
            .unwrap();
        assert_eq!(result.read_u256(true).unwrap(), 0);

        let mut result = contract.mint(address.clone(), amount).unwrap();
        assert_eq!(result.read_bool().unwrap(), true);

        let mut result = contract
            .execute(SELECTOR_TOTAL_SUPPLY, Cursor::new(0))
            .unwrap();
        assert_eq!(result.read_u256(true).unwrap(), amount);
    }
}
