use core::ops::Add;

use alloc::format;
use ethnum::u256;

use crate::{
    blockchain::AddressHash,
    constant::{ADDRESS_BYTE_LENGTH, BOOLEAN_BYTE_LENGTH, U256_BYTE_LENGTH},
    cursor::{self, Cursor},
    error::Error,
    math::abi::encode_selector_const,
    storage::{
        multi_address_map::MultiAddressMemoryMap,
        stored::{StoredTrait, StoredU256, StoredU8},
        stored_map::StoredMap,
    },
    types::{CallData, Selector},
    AsBytes, ToHex,
};

pub struct OP20Params {
    pub max_supply: StoredU256,
    pub decimals: StoredU8,
    pub name: &'static str,
    pub symbol: &'static str,
}

#[repr(u16)]
pub enum Pointer {
    NonceMap = 1,
    MaxSupply,
    Decimals,
    String,
    TotalSupply,
    AllowanceMap,
    BalanceOfMap,
}

impl Pointer {
    pub const fn u16(self) -> u16 {
        self as u16
    }
}

pub const SELECTOR_OWNER: Selector = encode_selector_const("owner");
pub const SELECTOR_DEPLOYER: Selector = encode_selector_const("deployer");
pub const SELECTOR_DECIMALS: Selector = encode_selector_const("decimals");
pub const SELECTOR_NAME: Selector = encode_selector_const("name");
pub const SELECTOR_SYMBOL: Selector = encode_selector_const("symbol");
pub const SELECTOR_TOTAL_SUPPLY: Selector = encode_selector_const("totalSupply");
pub const SELECTOR_MAXIMUM_SUPPLY: Selector = encode_selector_const("maximumSupply");
pub const SELECTOR_ALLOWANCE: Selector = encode_selector_const("allowance");
pub const SELECTOR_APPROVE: Selector = encode_selector_const("approve(address,uint256)");
pub const SELECTOR_APPROVE_FROM: Selector =
    encode_selector_const("approveFrom(address,uint256,uint256,bytes)");
pub const SELECTOR_BALANCE_OF: Selector = encode_selector_const("balanceOf(address)");
pub const SELECTOR_BURN: Selector = encode_selector_const("burn(uint256)");
pub const SELECTOR_TRANSFER: Selector = encode_selector_const("transfer(address,uint256)");
pub const SELECTOR_TRANSFER_FROM: Selector =
    encode_selector_const("transferFrom(address,address,uint256)");
pub const SELECTOR_NONCE_OF: Selector = encode_selector_const("nonceOf(address)");

pub trait OP20Trait: super::ContractTrait {
    fn execute_base(
        &mut self,
        selector: Selector,
        mut call_data: CallData,
    ) -> Result<Cursor, crate::error::Error> {
        match selector {
            SELECTOR_DEPLOYER => {
                let mut cursor = Cursor::new(ADDRESS_BYTE_LENGTH);
                cursor.write_address(&self.environment().contract_deployer)?;
                Ok(cursor)
            }
            SELECTOR_OWNER => {
                let mut cursor = Cursor::new(ADDRESS_BYTE_LENGTH);
                cursor.write_address(&self.environment().contract_deployer)?;
                Ok(cursor)
            }
            SELECTOR_DECIMALS => {
                let mut cursor = Cursor::new(1);
                cursor.write_u8(self.decimals())?;
                Ok(cursor)
            }
            SELECTOR_NAME => {
                let name = self.name();
                let mut cursor = Cursor::new(name.len() + 2);
                cursor.write_string_with_len(name)?;
                Ok(cursor)
            }
            SELECTOR_SYMBOL => {
                let symbol = self.symbol();

                let mut cursor = Cursor::new(symbol.len() + 2);
                cursor.write_string_with_len(symbol)?;
                Ok(cursor)
            }
            SELECTOR_TOTAL_SUPPLY => {
                let mut cursor = Cursor::new(32);
                cursor.write_u256(&self.total_supply().value(), true)?;
                Ok(cursor)
            }
            SELECTOR_MAXIMUM_SUPPLY => {
                let mut cursor = Cursor::new(32);
                cursor.write_u256(&self.max_supply(), true)?;
                Ok(cursor)
            }
            SELECTOR_ALLOWANCE => {
                let address_owner = call_data.read_address()?;
                let address_spender = call_data.read_address()?;

                self.allowance(&address_owner, &address_spender)
            }
            SELECTOR_APPROVE => {
                let owner = self.environment().caller;
                let spender = call_data.read_address()?;
                let value = call_data.read_u256(true)?;

                self.approve(owner, spender, value)
            }
            SELECTOR_APPROVE_FROM => {
                let owner = self.environment().origin;
                let spender = call_data.read_address()?;
                let value = call_data.read_u256(true)?;
                let nonce = call_data.read_u256(true)?;
                let signature = call_data.read_bytes_with_length(true)?;

                self.approve_from(owner, spender, value, nonce, signature)
            }
            SELECTOR_NONCE_OF => {
                let owner = call_data.read_address()?;
                self.nonce_of(owner)
            }
            SELECTOR_BALANCE_OF => {
                let address = call_data.read_address()?;

                self.balance_of(address)
            }

            SELECTOR_BURN => {
                let amount = call_data.read_u256(true)?;

                self.burn(amount)
            }

            SELECTOR_TRANSFER => {
                let address = call_data.read_address()?;
                let amount = call_data.read_u256(true)?;
                self.transfer(address, amount)
            }

            SELECTOR_TRANSFER_FROM => {
                let address_from = call_data.read_address()?;
                let address_to = call_data.read_address()?;
                let amount = call_data.read_u256(true)?;

                self.transfer_from(address_from, address_to, amount)
            }

            _ => Err(crate::error::Error::UnknownSelector),
        }
    }
    fn execute(
        &mut self,
        selector: Selector,
        call_data: crate::types::CallData,
    ) -> Result<Cursor, crate::error::Error> {
        self.execute_base(selector, call_data)
    }

    fn params(&mut self) -> &mut OP20Params;
    fn total_supply(&mut self) -> &mut StoredU256;

    fn max_supply(&mut self) -> u256 {
        self.params().max_supply.value()
    }
    fn decimals(&mut self) -> u8 {
        self.params().decimals.value()
    }

    fn name(&mut self) -> &'static str {
        self.params().name
    }
    fn symbol(&mut self) -> &'static str {
        self.params().symbol
    }

    fn allowance_map(&mut self) -> &mut MultiAddressMemoryMap;
    fn balance_of_map(&mut self) -> &mut StoredMap<AddressHash, u256>;
    fn nonce_map(&mut self) -> &mut StoredMap<AddressHash, u256>;

    #[inline]
    fn allowance_base(&mut self, owner: &AddressHash, spender: &AddressHash) -> u256 {
        let mut sender_map = self.allowance_map().get(owner);
        sender_map.get(&spender).u256()
    }

    #[inline]
    fn allowance(
        &mut self,
        owner: &AddressHash,
        spender: &AddressHash,
    ) -> Result<crate::cursor::Cursor, crate::error::Error> {
        let mut cursor = Cursor::new(32);
        let allowance = self.allowance_base(&owner, &spender);
        cursor.write_u256(&allowance, true)?;
        Ok(cursor)
    }

    #[inline]
    fn approve_base(
        &mut self,
        owner: AddressHash,
        spender: AddressHash,
        amount: u256,
    ) -> Result<bool, crate::error::Error> {
        if AddressHash::DEAD.eq(&owner) {
            return Err(crate::error::Error::Revert(
                "Address con not be dead address",
            ));
        }

        if AddressHash::DEAD.eq(&spender) {
            return Err(crate::error::Error::Revert(
                "Address con not be dead address",
            ));
        }

        self.allowance_map()
            .get(&owner)
            .set(&spender, amount.into());

        self.create_approve_event(owner, spender, amount)?;
        Ok(true)
    }

    #[inline]
    fn approve(
        &mut self,
        owner: AddressHash,
        spender: AddressHash,
        amount: u256,
    ) -> Result<Cursor, crate::error::Error> {
        let mut cursor = Cursor::new(BOOLEAN_BYTE_LENGTH);
        cursor.write_bool(self.approve_base(owner, spender, amount)?)?;
        Ok(cursor)
    }

    #[inline]
    fn approve_from_base(
        &mut self,
        owner: &AddressHash,
        spender: &AddressHash,
        value: &u256,
        nonce: &u256,
        signature: &[u8],
    ) -> Result<bool, crate::error::Error> {
        if AddressHash::DEAD.eq(owner) {
            return Err(crate::error::Error::DeadAddress);
        }

        if AddressHash::DEAD.eq(spender) {
            return Err(crate::error::Error::DeadAddress);
        }

        let stored_nonce = self.nonce_map().get(owner);
        if stored_nonce.eq(nonce) {
            return Err(crate::error::Error::Revert(
                "Invalid nonce (possible replay or out-of-sync)",
            ));
        }

        let mut data = Cursor::new(ADDRESS_BYTE_LENGTH * 2 + 2 * U256_BYTE_LENGTH);
        data.write_address(owner)?;
        data.write_address(spender)?;
        data.write_u256(&value, true)?;
        data.write_u256(&nonce, true)?;

        let hash = self.context().sha256(data.into_inner());
        if !self
            .context()
            .verify_schnorr_signature(owner, signature, &hash)?
        {
            return Err(crate::error::Error::Revert(
                "ApproveFrom: Invalid signature",
            ));
        }

        let mut sender_map = self.allowance_map().get(owner);
        sender_map.set(&spender, (*value).into());

        self.create_approve_event(*owner, *spender, *value)?;

        Ok(true)
    }

    #[inline]
    fn approve_from(
        &mut self,
        owner: AddressHash,
        spender: AddressHash,
        value: u256,
        nonce: u256,
        signature: &[u8],
    ) -> Result<Cursor, Error> {
        if owner == spender {
            return Err(Error::Revert(
                "Direct owner approval detected. Use approve function instead of approveFrom.",
            ));
        }

        if signature.len() != 64 {
            return Err(Error::Revert("Invalid signature length"));
        }

        let mut response = Cursor::new(BOOLEAN_BYTE_LENGTH);
        response
            .write_bool(self.approve_from_base(&owner, &spender, &value, &nonce, signature)?)?;
        Ok(response)
    }

    #[inline]
    fn nonce_of(&mut self, owner: AddressHash) -> Result<Cursor, Error> {
        let nonce = self.nonce_map().get(&owner);
        let mut response = Cursor::new(U256_BYTE_LENGTH);
        response.write_u256(&nonce, true)?;
        Ok(response)
    }

    #[inline]
    fn balance_of_base(&mut self, address: &AddressHash) -> u256 {
        self.balance_of_map().get(address)
    }

    #[inline]
    fn balance_of(
        &mut self,
        address: AddressHash,
    ) -> Result<crate::cursor::Cursor, crate::error::Error> {
        let mut result = Cursor::new(U256_BYTE_LENGTH);

        let balance = self.balance_of_base(&address);

        result.write_u256(&balance, true)?;
        Ok(result)
    }

    #[inline]
    fn burn_base(&mut self, value: u256, only_deployer: bool) -> Result<bool, crate::error::Error> {
        if value.eq(&u256::ZERO) {
            return Err(crate::error::Error::NoTokens);
        }

        let caller = self.environment().caller;
        if only_deployer {
            self.only_deployer(&caller)?;
        }

        let total_supply = self.total_supply().value();
        if total_supply < value {
            return Err(Error::Revert("Insufficient total supply."));
        }

        if !self.balance_of_map().contains_key(&caller) {
            return Err(crate::error::Error::Revert("No balance"));
        }

        let balance: u256 = self.balance_of_map().get(&caller);
        if balance < value {
            return Err(Error::Revert("Insufficient balance"));
        }

        let new_balance = balance - value;
        self.balance_of_map().set(&caller, new_balance);
        let value = self.total_supply().set(total_supply - value);

        self.create_burn_event(value)?;

        Ok(true)
    }

    #[inline]
    fn burn(&mut self, amount: u256) -> Result<crate::cursor::Cursor, Error> {
        let mut response = Cursor::new(1);

        response.write_bool(self.burn_base(amount, true)?)?;
        Ok(response)
    }

    #[inline]
    fn mint_base(
        &mut self,
        to: &AddressHash,
        value: u256,
        only_deployer: bool,
    ) -> Result<bool, Error> {
        if only_deployer {
            let caller = self.environment().caller;
            self.only_deployer(&caller)?;
        }

        if !self.balance_of_map().contains_key(to) {
            self.balance_of_map().set(to, value);
        } else {
            let to_balance = self.balance_of_map().get(to);
            self.balance_of_map().set(to, to_balance + value);
        }

        let old = self.total_supply().value();
        let new = old + value;

        if new > self.max_supply() {
            return Err(Error::Revert("Max supply reached"));
        }
        self.total_supply().set(new);
        self.create_mint_event(*to, value)?;
        self.context()
            .log(&format!("Minted! {}: {}", to.to_hex(), value));
        Ok(true)
    }

    #[inline]
    fn transfer_base(
        &mut self,
        to: &AddressHash,
        value: &u256,
    ) -> Result<bool, crate::error::Error> {
        let caller = self.environment().caller;
        if self.is_self(&caller) {
            return Err(Error::Revert("Can not transfer from self account"));
        }

        if u256::ZERO.eq(value) {
            return Err(Error::Revert("Cannot transfer 0 tokens"));
        }

        let balance = self.balance_of_map().get(&caller);

        if balance.lt(value) {
            return Err(Error::Revert("Insufficient balance"));
        }
        let new_balance = balance - value;
        self.balance_of_map().set(&caller, new_balance);

        let balance = self.balance_of_map().get(to);
        let new_balance = balance + value;
        self.balance_of_map().set(to, new_balance);

        self.create_transfer_event(caller, *to, *value)?;
        Ok(true)
    }

    #[inline]
    fn transfer(
        &mut self,
        address: AddressHash,
        amount: u256,
    ) -> Result<crate::cursor::Cursor, crate::error::Error> {
        let mut result = Cursor::new(1);

        result.write_bool(self.transfer_base(&address, &amount)?)?;
        Ok(result)
    }

    #[inline]
    fn spend_allowance(
        &mut self,
        owner: &AddressHash,
        spender: &AddressHash,
        value: u256,
    ) -> Result<(), crate::error::Error> {
        let mut owner_allowance_map = self.allowance_map().get(owner);
        let allowed: u256 = owner_allowance_map.get(&spender).u256();

        if allowed < value {
            return Err(crate::error::Error::InsufficientAllowance);
        }

        let new_allowance = allowed - value;
        owner_allowance_map.set(&spender, new_allowance.into());
        self.allowance_map().set(*owner, owner_allowance_map);
        Ok(())
    }

    #[inline]
    fn transfer_from_unsafe(
        &mut self,
        from: &AddressHash,
        to: &AddressHash,
        value: u256,
    ) -> Result<bool, crate::error::Error> {
        let balance: u256 = self.balance_of_map().get(from);
        if balance < value {
            return Err(crate::error::Error::InsufficientBalance);
        }

        let new_balance = balance - value;
        self.balance_of_map().set(from, new_balance);

        if !self.balance_of_map().contains_key(to) {
            self.balance_of_map().set(to, value);
        } else {
            let to_balance: u256 = self.balance_of_map().get(to);
            let new_to_balance = to_balance + value;
            self.balance_of_map().set(to, new_to_balance);
        }

        self.create_transfer_event(*from, *to, value)?;

        Ok(true)
    }

    #[inline]
    fn transfer_from_base(
        &mut self,
        from: &AddressHash,
        to: &AddressHash,
        value: u256,
    ) -> Result<bool, crate::error::Error> {
        if AddressHash::DEAD.eq(from) || AddressHash::DEAD.eq(to) {
            return Err(crate::error::Error::DeadAddress);
        }

        let sender = self.environment().caller;

        self.spend_allowance(from, &sender, value)?;
        self.transfer_from_unsafe(from, to, value)?;
        Ok(true)
    }

    #[inline]
    fn transfer_from(
        &mut self,
        address_from: AddressHash,
        address_to: AddressHash,
        amount: u256,
    ) -> Result<crate::cursor::Cursor, crate::error::Error> {
        let mut result = Cursor::new(BOOLEAN_BYTE_LENGTH);
        result.write_bool(self.transfer_from_base(&address_from, &address_to, amount)?)?;
        Ok(result)
    }

    #[inline]
    fn create_burn_event(&mut self, value: u256) -> Result<(), crate::error::Error> {
        let burn_event = crate::event::Event::burn(value)?;
        self.emit(&burn_event);
        Ok(())
    }

    fn create_approve_event(
        &self,
        deployer: AddressHash,
        spender: AddressHash,
        value: u256,
    ) -> Result<(), crate::error::Error> {
        let approve_event = crate::event::Event::approve(deployer, spender, value)?;
        self.emit(&approve_event);
        Ok(())
    }

    fn create_mint_event(
        &mut self,
        deployer: AddressHash,
        amount: u256,
    ) -> Result<(), crate::error::Error> {
        let mint_event = crate::event::Event::mint(deployer, amount)?;
        self.emit(&mint_event);
        Ok(())
    }

    fn create_transfer_event(
        &mut self,
        from: AddressHash,
        to: AddressHash,
        amount: u256,
    ) -> Result<(), crate::error::Error> {
        let transfer_event = crate::event::Event::transfer(from, to, amount)?;
        self.emit(&transfer_event);
        Ok(())
    }
}
