use crate::types::enums::Rol;
use ink::prelude::{string::String, vec::Vec};
use ink::primitives::AccountId;
use ink::scale::{Decode, Encode};

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(ink::scale_info::TypeInfo))]
pub struct Usuario {
    pub username: String,
    pub rol: Rol,
    pub calificaciones: Vec<Calificacion>,
    pub verificacion: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(ink::scale_info::TypeInfo))]
pub struct Calificacion {
    pub evaluador: AccountId,
    pub puntaje: u8,
    pub id_orden: u64,
}
