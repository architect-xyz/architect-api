use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Bimap<U: Clone + Ord, V: Clone + Ord> {
    map: BTreeMap<U, V>,
    rev: BTreeMap<V, U>,
}

impl<U: Clone + Ord, V: Clone + Ord> Bimap<U, V> {
    pub fn new() -> Self {
        Self { map: BTreeMap::new(), rev: BTreeMap::new() }
    }

    pub fn insert(&mut self, u: U, v: V) {
        self.remove(&u);
        self.remove_rev(&v);
        self.map.insert(u.clone(), v.clone());
        self.rev.insert(v, u);
    }

    pub fn get(&self, u: &U) -> Option<&V> {
        self.map.get(u)
    }

    pub fn get_rev(&self, v: &V) -> Option<&U> {
        self.rev.get(v)
    }

    pub fn get_mut(&mut self, u: &U) -> Option<&mut V> {
        self.map.get_mut(u)
    }

    pub fn get_rev_mut(&mut self, v: &V) -> Option<&mut U> {
        self.rev.get_mut(v)
    }

    pub fn contains(&self, u: &U) -> bool {
        self.map.contains_key(u)
    }

    pub fn contains_rev(&self, v: &V) -> bool {
        self.rev.contains_key(v)
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn remove(&mut self, u: &U) -> Option<V> {
        let v = self.map.remove(u)?;
        self.rev.remove(&v);
        Some(v)
    }

    pub fn remove_rev(&mut self, v: &V) -> Option<U> {
        let u = self.rev.remove(v)?;
        self.map.remove(&u);
        Some(u)
    }

    pub fn clear(&mut self) {
        self.map.clear();
        self.rev.clear();
    }

    pub fn retain(&mut self, mut f: impl FnMut(&U, &V) -> bool) {
        self.map.retain(|u, v| f(u, v));
        self.rev.retain(|v, u| f(u, v));
    }

    pub fn iter(&self) -> impl Iterator<Item = (&U, &V)> {
        self.map.iter()
    }
}

impl<U: Clone + Ord, V: Clone + Ord> Default for Bimap<U, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<U: Clone + Ord, V: Clone + Ord> FromIterator<(U, V)> for Bimap<U, V> {
    fn from_iter<I: IntoIterator<Item = (U, V)>>(iter: I) -> Self {
        let mut bimap = Bimap::new();
        for (u, v) in iter {
            bimap.insert(u, v);
        }
        bimap
    }
}

impl<U: Clone + Ord, V: Clone + Ord> IntoIterator for Bimap<U, V> {
    type Item = (U, V);
    type IntoIter = std::collections::btree_map::IntoIter<U, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}
