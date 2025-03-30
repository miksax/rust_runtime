//! Simplified version Map
//!

/// Map structure
#[derive(Clone)]
pub struct Map<Key, Value>
where
    Key: Sized + Eq,
    Value: Sized + Clone + Eq,
{
    items: alloc::vec::Vec<(Key, Value)>,
}
// Implement default trait
impl<Key: Sized + Eq, Value: Sized + Eq + Clone> Default for Map<Key, Value> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Key: Sized + Eq, Value: Sized + Eq + Clone> Map<Key, Value> {
    pub const fn new() -> Self {
        Self {
            items: alloc::vec::Vec::new(),
        }
    }

    pub fn new_with_capacity(capacity: usize) -> Self {
        Self {
            items: alloc::vec::Vec::with_capacity(capacity),
        }
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Doesn not check for duplicities.
    /// Faster, but can make map unstable
    pub fn push(&mut self, key: Key, value: Value) {
        self.items.push((key, value));
    }

    /// Insert element and check for duplicities
    pub fn insert(&mut self, key: Key, value: Value) -> Option<Value> {
        if let Some((_, val)) = self.items.iter_mut().find(|(key, _)| key.eq(key)) {
            let result = val.clone();
            *val = value;
            Some(result)
        } else {
            self.push(key, value);
            None
        }
    }

    pub fn pop(&mut self) -> Option<(Key, Value)> {
        self.items.pop()
    }

    pub fn contains_key(&self, key: &Key) -> bool {
        self.items.iter().any(|(k, _)| k.eq(key))
    }

    pub fn contains_value(&self, value: &Value) -> bool {
        self.items.iter().any(|(_, v)| v.eq(value))
    }

    pub fn get(&self, key: &Key) -> Option<&Value> {
        self.items.iter().find(|(k, _)| k.eq(key)).map(|(_, v)| v)
    }

    pub fn get_mut(&mut self, key: &Key) -> Option<&mut Value> {
        self.items
            .iter_mut()
            .find(|(k, _)| k.eq(key))
            .map(|(_, v)| v)
    }

    pub fn iter(&self) -> impl Iterator<Item = &(Key, Value)> {
        self.items.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut (Key, Value)> {
        self.items.iter_mut()
    }

    pub fn iter_keys(&self) -> impl Iterator<Item = &Key> {
        self.items.iter().map(|i| &i.0)
    }

    pub fn iter_keys_mut(&mut self) -> impl Iterator<Item = &mut Key> {
        self.items.iter_mut().map(|i| &mut i.0)
    }

    pub fn iter_values(&self) -> impl Iterator<Item = &Value> {
        self.items.iter().map(|i| &i.1)
    }

    pub fn iter_values_mut(&mut self) -> impl Iterator<Item = &mut Value> {
        self.items.iter_mut().map(|(_, v)| v)
    }

    pub fn remove(&mut self, key: &Key) -> Option<Value> {
        if let Some(index) = self.items.iter().position(|(k, _)| k.eq(key)) {
            Some(self.items.remove(index).1)
        } else {
            None
        }
    }
}

impl<Key: Sized + Eq, Value: Sized + Eq + Clone> core::ops::Index<&Key> for Map<Key, Value> {
    type Output = Value;

    fn index(&self, index: &Key) -> &Value {
        self.get(index).unwrap()
    }
}

impl<Key: Sized + Eq, Value: Sized + Eq + Clone> core::ops::Index<Key> for Map<Key, Value> {
    type Output = Value;

    fn index(&self, index: Key) -> &Value {
        self.get(&index).unwrap()
    }
}

impl<Key: Sized + Eq, Value: Sized + Eq + Clone> core::ops::IndexMut<&Key> for Map<Key, Value> {
    fn index_mut(&mut self, index: &Key) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl<Key: Sized + Eq, Value: Sized + Eq + Clone> core::ops::IndexMut<Key> for Map<Key, Value> {
    fn index_mut(&mut self, index: Key) -> &mut Self::Output {
        self.get_mut(&index).unwrap()
    }
}
