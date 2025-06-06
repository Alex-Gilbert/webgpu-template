use std::marker::PhantomData;

use super::interpolation_value::InterpolationValue;

// Your existing VariableEnum trait
pub trait VariableEnum: Copy + Clone + 'static {
    const COUNT: usize;
    fn from_name(name: &str) -> Option<Self>;
    fn to_index(self) -> usize;
}

// Your existing macro (same as before)
macro_rules! define_variables {
    ($enum_name:ident { $($variant:ident => $name:literal),* $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[repr(usize)]
        pub enum $enum_name {
            $($variant),*
        }

        impl VariableEnum for $enum_name {
            const COUNT: usize = {
                let mut count = 0;
                $(
                    let _ = stringify!($variant);
                    count += 1;
                )*
                count
            };

            fn from_name(name: &str) -> Option<Self> {
                match name {
                    $($name => Some(Self::$variant),)*
                    _ => None,
                }
            }

            fn to_index(self) -> usize {
                self as usize
            }
        }
    };
}

// Trait for dynamic dispatch of variable storage
pub trait VariableStorage {
    fn get_value(&self, name: &str) -> Option<&InterpolationValue>;
    fn set_value_by_name(&mut self, name: &str, value: InterpolationValue) -> bool;
    fn count(&self) -> usize;
    fn clone_box(&self) -> Box<dyn VariableStorage>;
}

// Empty variable storage for static text (no interpolation)
#[derive(Debug, Clone)]
pub struct EmptyVariableStorage;

impl EmptyVariableStorage {
    pub fn new() -> Self {
        Self
    }
}

impl VariableStorage for EmptyVariableStorage {
    fn get_value(&self, _name: &str) -> Option<&InterpolationValue> {
        None
    }

    fn set_value_by_name(&mut self, _name: &str, _value: InterpolationValue) -> bool {
        false
    }

    fn count(&self) -> usize {
        0
    }

    fn clone_box(&self) -> Box<dyn VariableStorage> {
        Box::new(EmptyVariableStorage)
    }
}

// Concrete implementation for specific enum types
pub struct EnumVariableStorage<T: VariableEnum> {
    values: Vec<Option<InterpolationValue>>,
    _phantom: PhantomData<T>,
}

impl<T: VariableEnum> EnumVariableStorage<T> {
    pub fn new() -> Self {
        Self {
            values: vec![None; T::COUNT],
            _phantom: PhantomData,
        }
    }

    pub fn set_variable(&mut self, var: T, value: impl Into<InterpolationValue>) {
        self.values[var.to_index()] = Some(value.into());
    }

    pub fn get_variable(&self, var: T) -> Option<&InterpolationValue> {
        self.values[var.to_index()].as_ref()
    }

    pub fn with_variable(mut self, var: T, value: impl Into<InterpolationValue>) -> Self {
        self.set_variable(var, value);
        self
    }
}

impl<T: VariableEnum> VariableStorage for EnumVariableStorage<T> {
    fn get_value(&self, name: &str) -> Option<&InterpolationValue> {
        T::from_name(name).and_then(|var| self.get_variable(var))
    }

    fn set_value_by_name(&mut self, name: &str, value: InterpolationValue) -> bool {
        if let Some(var) = T::from_name(name) {
            self.set_variable(var, value);
            true
        } else {
            false
        }
    }

    fn count(&self) -> usize {
        T::COUNT
    }

    fn clone_box(&self) -> Box<dyn VariableStorage> {
        Box::new(EnumVariableStorage::<T> {
            values: self.values.clone(),
            _phantom: PhantomData,
        })
    }
}

impl<T: VariableEnum> Clone for EnumVariableStorage<T> {
    fn clone(&self) -> Self {
        Self {
            values: self.values.clone(),
            _phantom: PhantomData,
        }
    }
}
