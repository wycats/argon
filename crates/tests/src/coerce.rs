use nan_preserving_float::{F32, F64};
use wasmi::RuntimeValue;

pub trait AsRuntimeValue {
    fn as_runtime_value(&self) -> RuntimeValue;
}

impl AsRuntimeValue for u32 {
    fn as_runtime_value(&self) -> RuntimeValue {
        RuntimeValue::I32(*self as i32)
    }
}

impl AsRuntimeValue for i32 {
    fn as_runtime_value(&self) -> RuntimeValue {
        RuntimeValue::I32(*self)
    }
}

impl AsRuntimeValue for u64 {
    fn as_runtime_value(&self) -> RuntimeValue {
        RuntimeValue::I64(*self as i64)
    }
}

impl AsRuntimeValue for i64 {
    fn as_runtime_value(&self) -> RuntimeValue {
        RuntimeValue::I64(*self)
    }
}

impl AsRuntimeValue for f32 {
    fn as_runtime_value(&self) -> RuntimeValue {
        RuntimeValue::F32(F32::from_float(*self))
    }
}

impl AsRuntimeValue for f64 {
    fn as_runtime_value(&self) -> RuntimeValue {
        RuntimeValue::F64(F64::from_float(*self))
    }
}
