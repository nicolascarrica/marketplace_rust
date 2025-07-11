use crate::types::enums::Categoria;
use ink::prelude::string::String;
use ink::primitives::AccountId;
use ink::scale::{Decode, Encode};
use ink::storage::traits::StorageLayout;

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, StorageLayout)]
#[cfg_attr(feature = "std", derive(ink::scale_info::TypeInfo))]
pub struct Producto {
    pub id: u64,
    pub propietario: AccountId,
    pub nombre: String,
    pub descripcion: String,
    pub precio: u32,
    pub stock: u32,
    pub categoria: Categoria,
}
