use crate::types::{Estado, Producto};
use ink::scale::{Decode, Encode};
use ink::storage::Mapping;

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(ink::scale_info::TypeInfo))]
pub struct Orden {
    pub id: u64,
    pub productos: Vec<(u64, u32)>, // (producto_id, cantidad)
    pub estado: Estado,
}
