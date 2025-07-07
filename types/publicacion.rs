use ink::primitives::AccountId;
use ink::scale::{Decode, Encode};

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(ink::scale_info::TypeInfo))]
pub struct Publicacion {
    pub vendedor: AccountId,
    pub productos: Vec<u64>, // IDs de productos
}
