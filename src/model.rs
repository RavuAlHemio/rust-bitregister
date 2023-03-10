use serde::{Deserialize, Serialize};


type NumericValue = u128;


/// Grouping of functionally related register blocks.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Group {
    /// The name of this group.
    pub name: String,

    /// The blocks of which this group consists.
    pub blocks: Vec<Block>,
}

/// A block of consecutive registers.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Block {
    /// The name of this block.
    pub name: String,

    /// The registers or paddings in this block.
    pub registers_or_paddings: Vec<RegisterOrReserved>,
}

/// A register or padding value.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum RegisterOrReserved {
    /// A register.
    Register(Register),

    /// A reserved value.
    Reserved(ReservedValue),
}

/// A block of consecutive registers.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Register {
    /// The name of this block.
    pub name: String,

    /// The size of this register in bytes.
    pub size_bytes: u8,

    /// The default value of this register, if any.
    ///
    /// If specified, generates a write accessor.
    pub default_value: Option<NumericValue>,

    /// The fields, fixed or variable, of this register.
    pub fields: Vec<Field>,
}

/// A reserved value between registers.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ReservedValue {
    /// The size of this reserved value in bytes.
    pub size_bytes: u8,
}

/// A fixed or variable field within a register.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Field {
    /// A fixed field.
    Fixed(FixedField),

    /// A variable field.
    Variable(VariableField),
}

/// A fixed-value field within a register.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct FixedField {
    /// The bit, counted from LSB, where the field starts within the register.
    pub start_bit: u8,

    /// The number of bits occupied by this fixed field.
    pub bit_count: u8,

    /// The fixed value of this field.
    pub value: NumericValue,
}

/// A variable-value field within a register.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct VariableField {
    /// The bit, counted from LSB, where the field starts within the register.
    pub start_bit: u8,

    /// The number of bits occupied by this fixed field.
    pub bit_count: u8,

    /// Enumerated values for this field, where applicable.
    pub values: Option<Vec<EnumeratedValue>>,
}


/// An enumerated value within a field.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct EnumeratedValue {
    /// The name of this enumerated value.
    pub name: String,

    /// The numeric value of this enumerated value.
    pub value: NumericValue,
}
