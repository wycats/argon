use crate::{Type, TypeVar};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
pub struct TypeVarSet(BTreeSet<TypeVar>);

impl TypeVarSet {
    pub fn var(var: TypeVar) -> TypeVarSet {
        let mut set = BTreeSet::new();
        set.insert(var);
        TypeVarSet(set)
    }

    pub fn empty() -> TypeVarSet {
        TypeVarSet(BTreeSet::new())
    }
}

impl std::ops::Add for TypeVarSet {
    type Output = TypeVarSet;

    fn add(self, rhs: TypeVarSet) -> TypeVarSet {
        TypeVarSet(self.0.union(&rhs.0).cloned().collect())
    }
}

impl std::ops::AddAssign for TypeVarSet {
    fn add_assign(&mut self, rhs: TypeVarSet) {
        self.0 = self.0.union(&rhs.0).cloned().collect()
    }
}

impl std::ops::Add<TypeVar> for TypeVarSet {
    type Output = TypeVarSet;

    fn add(self, rhs: TypeVar) -> TypeVarSet {
        let mut set = self.0;
        set.insert(rhs);
        TypeVarSet(set)
    }
}

impl std::ops::AddAssign<TypeVar> for TypeVarSet {
    fn add_assign(&mut self, rhs: TypeVar) {
        self.0.insert(rhs);
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
pub struct TypeSet(BTreeSet<Type>);

impl TypeSet {
    pub fn has(&self, ty: &Type) -> bool {
        self.0.contains(ty)
    }

    pub fn vars(&self) -> TypeVarSet {
        let mut vars = TypeVarSet::empty();

        for ty in &self.0 {
            vars += ty.vars();
        }

        vars
    }
}

pub struct TypeMap<V>(BTreeMap<TypeVar, V>);

impl<V> TypeMap<V> {
    pub fn empty() -> TypeMap<V> {
        TypeMap(BTreeMap::new())
    }

    pub fn take(&mut self, var: TypeVar) -> V {
        self.0.remove(&var).unwrap()
    }

    pub fn has(&self, var: TypeVar) -> bool {
        self.0.contains_key(&var)
    }
}

impl<T> std::ops::Index<TypeVar> for TypeMap<T> {
    type Output = T;

    fn index(&self, var: TypeVar) -> &T {
        self.0.get(&var).unwrap()
    }
}

impl<T> std::ops::IndexMut<TypeVar> for TypeMap<T> {
    fn index_mut(&mut self, var: TypeVar) -> &mut T {
        self.0.get_mut(&var).unwrap()
    }
}
