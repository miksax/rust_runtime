use rust_runtime::{
    blockchain::AddressHash,
    contract::op_20::Pointer,
    ethnum::u256,
    math::abi::encode_selector_const,
    storage::{
        multi_address_map::MultiAddressMemoryMap,
        stored::{StoredTrait, StoredU256, StoredU8},
        stored_map::StoredMap,
        StorageValue,
    },
    types::{CallData, Selector},
    ContractTrait, OP20Trait,
};

const SELECTOR_AIRDROP: Selector = encode_selector_const("airdrop");
const SELECTOR_AIRDROP_WITH_AMOUNT: Selector = encode_selector_const("airdropWithAmount");
const SELECTOR_MINT: Selector = encode_selector_const("mint");

pub struct Contract {
    environment: Option<&'static rust_runtime::blockchain::Environment>,
    params: rust_runtime::contract::op_20::OP20Params,
    balance_of_map: StoredMap<AddressHash, u256>,
    allowance_map: MultiAddressMemoryMap,
    total_supply: StoredU256,
}
impl Contract {
    pub const fn new() -> Self {
        Self {
            environment: None,
            params: rust_runtime::contract::op_20::OP20Params {
                max_supply: StoredU256::new_const(
                    Pointer::MaxSupply.u16(),
                    u256::new(100000000000000000000000000),
                ),
                decimals: StoredU8::new_const(Pointer::Decimals.u16(), 18),
                name: "MyToken",
                symbol: "TOKEN",
            },
            balance_of_map: StoredMap::new(Pointer::BalanceOfMap.u16()),
            allowance_map: MultiAddressMemoryMap::new(
                Pointer::AllowanceMap.u16(),
                StorageValue::ZERO,
            ),
            total_supply: StoredU256::new_const(Pointer::TotalSupply.u16(), u256::ZERO),
        }
    }
}

impl Contract {
    fn execute(
        &mut self,
        selector: Selector,
        call_data: CallData,
    ) -> Result<crate::WaBuffer, rust_runtime::error::Error> {
        match selector {
            SELECTOR_MINT => self.mint(call_data),
            SELECTOR_AIRDROP => self.airdrop(call_data),
            SELECTOR_AIRDROP_WITH_AMOUNT => self.airdrop_with_amount(call_data),
            _ => OP20Trait::execute_base(self, selector, call_data),
        }
    }

    fn mint(
        &mut self,
        mut call_data: CallData,
    ) -> Result<crate::WaBuffer, rust_runtime::error::Error> {
        self.only_deployer(&self.environment().sender)?;

        let mut response = crate::WaBuffer::new(1, 1)?;
        let mut cursor = response.cursor();
        let address = call_data.read_address()?;
        let amount = call_data.read_u256_be()?;
        cursor.write_bool(self.mint_base(&address, amount, false)?)?;

        return Ok(response);
    }

    fn airdrop(
        &mut self,
        mut call_data: CallData,
    ) -> Result<crate::WaBuffer, rust_runtime::error::Error> {
        self.only_deployer(&self.environment().sender)?;
        let drops = call_data.read_address_value_map()?;
        for (address, amount) in drops.iter() {
            self.mint_base(address, amount.clone(), false)?;
        }

        let mut response = crate::WaBuffer::new(1, 1)?;
        let mut cursor = response.cursor();
        cursor.write_bool(true)?;

        Ok(response)
    }

    fn optimized_mint(
        &mut self,
        address: AddressHash,
        amount: u256,
    ) -> Result<(), rust_runtime::error::Error> {
        self.balance_of_map.set(&address, amount);
        let value = self.total_supply.value() + amount;
        self.total_supply.set_no_commit(value);

        Self::create_mint_event(address, amount)?;

        Ok(())
    }

    fn airdrop_with_amount(
        &mut self,
        mut call_data: CallData,
    ) -> Result<crate::WaBuffer, rust_runtime::error::Error> {
        self.only_deployer(&self.environment().sender)?;
        let amount = call_data.read_u256_be()?;
        let amount_of_addresses: u32 = call_data.read_u32_le()?;

        for _ in 0..amount_of_addresses {
            let address = call_data.read_address()?;
            self.optimized_mint(address, amount)?;
        }

        self.total_supply.commit();

        let mut response = crate::WaBuffer::new(1, 1)?;
        let mut cursor = response.cursor();
        cursor.write_bool(true)?;
        Ok(response)
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
}

impl rust_runtime::contract::ContractTrait for Contract {
    fn set_environment(&mut self, environment: &'static rust_runtime::blockchain::Environment) {
        self.environment = Some(environment);
    }

    fn environment(&self) -> &'static rust_runtime::blockchain::Environment {
        self.environment.unwrap()
    }

    fn execute(
        &mut self,
        mut call_data: CallData,
    ) -> Result<rust_runtime::WaBuffer, rust_runtime::error::Error> {
        let selector = call_data.read_selector()?;

        Contract::execute(self, selector, call_data)
    }

    fn on_deploy(&mut self, _call_data: CallData) {}
}
