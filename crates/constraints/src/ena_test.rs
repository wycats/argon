use ena::unify::{InPlaceUnificationTable, UnificationTable, UnifyKey, UnifyValue};
use std::collections::BTreeSet;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct TypeVar(u32);

impl UnifyKey for TypeVar {
    type Value = InferenceValue;

    fn index(&self) -> u32 {
        self.0
    }

    fn from_index(u: u32) -> TypeVar {
        TypeVar(u)
    }

    fn tag() -> &'static str {
        "type"
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum InferenceValue {
    Unbound,
    Bound(TypeBound),
}

impl UnifyValue for InferenceValue {
    type Error = (InferenceValue, InferenceValue);

    fn unify_values(
        a: &InferenceValue,
        b: &InferenceValue,
    ) -> Result<InferenceValue, (InferenceValue, InferenceValue)> {
        match (a, b) {
            (InferenceValue::Unbound, InferenceValue::Unbound) => Ok(InferenceValue::Unbound),
            (bound @ InferenceValue::Bound(_), InferenceValue::Unbound)
            | (InferenceValue::Unbound, bound @ InferenceValue::Bound(_)) => Ok(bound.clone()),
            (InferenceValue::Bound(lhs), InferenceValue::Bound(rhs)) => {
                Ok(InferenceValue::Bound(unify_bound(lhs, rhs)?))
            }
        }
    }
}

fn unify_bound(
    left: &TypeBound,
    right: &TypeBound,
) -> Result<TypeBound, (InferenceValue, InferenceValue)> {
    match (left, right) {
        (TypeBound::Set(set1), TypeBound::Set(set2)) if set1.is_disjoint(&set2) => Err((
            InferenceValue::Bound(TypeBound::Set(set1.clone())),
            InferenceValue::Bound(TypeBound::Set(set2.clone())),
        )),

        (TypeBound::Set(set1), TypeBound::Set(set2)) => {
            Ok(TypeBound::set(set1.intersection(&set2).cloned().collect()))
        }

        (left, right) => Err((
            InferenceValue::Bound(left.clone()),
            InferenceValue::Bound(right.clone()),
        )),
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum TypeBound {
    Set(BTreeSet<Concrete>),
    Apply(Box<InferenceValue>, Vec<InferenceValue>),
    Function(Vec<InferenceValue>, Box<InferenceValue>),
}

impl TypeBound {
    fn ty(ty: Concrete) -> TypeBound {
        let mut set = BTreeSet::new();
        set.insert(ty);
        TypeBound::Set(set)
    }

    fn set(types: BTreeSet<Concrete>) -> TypeBound {
        TypeBound::Set(types)
    }

    fn function(params: Vec<InferenceValue>, ret: InferenceValue) -> TypeBound {
        TypeBound::Function(params, box ret)
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub enum Concrete {
    I32,
    I64,
    U32,
    U64,
    F32,
    F64,
}

#[test]
fn test_unify_bound() -> Result<(), (InferenceValue, InferenceValue)> {
    let mut table: InPlaceUnificationTable<TypeVar> = UnificationTable::new();
    let t1 = table.new_key(InferenceValue::Unbound);
    let t2 = table.new_key(InferenceValue::Unbound);

    table.unify_var_var(t1, t2)?;
    table.unify_var_value(t1, InferenceValue::Bound(integer()))?;
    table.unify_var_value(t2, InferenceValue::Bound(TypeBound::ty(Concrete::I64)))?;

    assert_eq!(
        table.probe_value(t1),
        InferenceValue::Bound(TypeBound::ty(Concrete::I64))
    );

    Ok(())
}

#[test]
fn test_unify_intersect() -> Result<(), (InferenceValue, InferenceValue)> {
    let mut table: InPlaceUnificationTable<TypeVar> = UnificationTable::new();
    let t1 = table.new_key(InferenceValue::Unbound);
    let t2 = table.new_key(InferenceValue::Unbound);

    table.unify_var_var(t1, t2)?;
    table.unify_var_value(t1, InferenceValue::Bound(uint()))?;
    table.unify_var_value(t2, InferenceValue::Bound(int32()))?;

    assert_eq!(
        table.probe_value(t1),
        InferenceValue::Bound(TypeBound::ty(Concrete::U32))
    );

    Ok(())
}

#[test]
fn test_unify_function() -> Result<(), (InferenceValue, InferenceValue)> {
    let mut table: InPlaceUnificationTable<TypeVar> = UnificationTable::new();
    let t1 = table.new_key(InferenceValue::Unbound);
    let t2 = table.new_key(InferenceValue::Unbound);

    table.unify_var_var(t1, t2)?;
    table.unify_var_value(
        t2,
        InferenceValue::Bound(TypeBound::function(
            vec![InferenceValue::Bound(TypeBound::ty(Concrete::U64))],
            InferenceValue::Bound(TypeBound::ty(Concrete::U64)),
        )),
    )?;

    assert_eq!(
        table.probe_value(t1),
        InferenceValue::Bound(TypeBound::function(
            vec![InferenceValue::Bound(TypeBound::ty(Concrete::U64))],
            InferenceValue::Bound(TypeBound::ty(Concrete::U64)),
        ))
    );

    Ok(())
}

fn integer() -> TypeBound {
    type_set(vec![
        Concrete::I32,
        Concrete::I64,
        Concrete::U32,
        Concrete::U64,
    ])
}

fn uint() -> TypeBound {
    type_set(vec![Concrete::U32, Concrete::U64])
}

fn int32() -> TypeBound {
    type_set(vec![Concrete::U32, Concrete::I32])
}

fn type_set(types: Vec<Concrete>) -> TypeBound {
    let mut set = BTreeSet::new();

    for item in types {
        set.insert(item);
    }

    TypeBound::Set(set)
}
