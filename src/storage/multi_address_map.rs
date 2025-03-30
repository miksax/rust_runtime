use alloc::{rc::Rc, vec::Vec};

use crate::{blockchain::AddressHash, AsBytes, Context};

use super::{array_merger::ArrayMerger, map::Map, StorageValue};

pub struct MultiAddressMemoryMap {
    context: Rc<dyn Context>,
    pointer: u16,
    default_value: StorageValue,
    pub map: Map<AddressHash, ArrayMerger>,
}

impl MultiAddressMemoryMap {
    pub const fn new(context: Rc<dyn Context>, pointer: u16, default_value: StorageValue) -> Self {
        Self {
            context,
            pointer,
            default_value,
            map: Map::new(),
        }
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }

    pub fn get(&mut self, address: &AddressHash) -> ArrayMerger {
        self.create_key_merger(address);
        self.map.get(address).unwrap().clone()
    }

    pub fn set(&mut self, key: AddressHash, value: ArrayMerger) {
        self.create_key_merger(&key);
        self.map.insert(key, value);
    }

    pub fn contains_key(&self, key: &AddressHash) -> bool {
        self.map.contains_key(key)
    }

    fn create_key_merger(&mut self, key: &AddressHash) {
        if !self.map.contains_key(key) {
            self.map.push(
                *key,
                ArrayMerger::new(
                    self.context.clone(),
                    key.as_bytes().to_vec(),
                    self.pointer,
                    self.default_value,
                ),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MultiAddressMemoryMap;
    use crate::{storage::StorageValue, tests::random_address, AsBytes, TestContext};
    use alloc::rc::Rc;

    #[test]
    fn test1() {
        let context = Rc::new(TestContext::default());
        let address1 = random_address();
        let address2 = random_address();
        let one = [1; 32];
        let mut mamm1 = MultiAddressMemoryMap::new(context.clone(), 0, StorageValue::ZERO);
        let mut merger1 = mamm1.get(&address1);
        assert_eq!(
            merger1.get(&address2).as_bytes(),
            StorageValue::ZERO.as_bytes()
        );

        merger1.set(&address2, StorageValue::new(one));
        assert_eq!(merger1.get(&address2).as_bytes(), &one);

        let mut mamm2 = MultiAddressMemoryMap::new(context.clone(), 0, StorageValue::ZERO);
        let mut merger2 = mamm2.get(&address1);
        assert_eq!(merger2.get(&address2).as_bytes(), &one);
    }
}
