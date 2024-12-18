use map::Map;

pub mod array_merger;
pub mod key;
pub mod map;
pub mod multi_address_map;
pub mod stored;
pub mod stored_map;
pub mod stored_string;
pub mod value;

pub use key::StorageKey;
pub use value::StorageValue;

#[allow(static_mut_refs)]
static mut GLOBAL_STORE: Map<StorageKey, StorageValue> = Map::new();

pub struct GlobalStore {}

impl GlobalStore {
    pub fn get(key: &StorageKey, default_value: StorageValue) -> StorageValue {
        if Self::has_key(key) {
            #[allow(static_mut_refs)]
            unsafe {
                if let Some(value) = GLOBAL_STORE.get(key) {
                    *value
                } else {
                    if let Ok(result) = crate::env::pointer_load(key) {
                        GLOBAL_STORE.insert(key.clone(), result);
                        result
                    } else {
                        default_value
                    }
                }
            }
        } else {
            default_value
        }
    }

    pub fn set(key: StorageKey, value: StorageValue) {
        assert!(crate::env::pointer_store(&key, &value).unwrap());

        #[allow(static_mut_refs)]
        unsafe {
            GLOBAL_STORE.insert(key, value);
        }
    }

    pub fn has_key(key: &StorageKey) -> bool {
        #[allow(static_mut_refs)]
        unsafe {
            if GLOBAL_STORE.contains_key(key) {
                return true;
            };

            if let Ok(result) = crate::env::pointer_load(key) {
                GLOBAL_STORE.insert(*key, result);
                return !result.zero();
            }
        }

        false
    }
}
