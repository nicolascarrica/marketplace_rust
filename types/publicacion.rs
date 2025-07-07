use ink::primitives::AccountId;
use ink::scale::{Decode, Encode};

use crate::types::EstadoPublicacion;
#[cfg_attr(feature = "std", derive(ink::scale_info::TypeInfo))]
pub struct Publicacion {
    id_publicacion: u64,       // ID de la publicación
    id_vendedor: AccountId,    // ID del vendedor
    precio: u64,               // Precio de la publicación
    estado: EstadoPublicacion, // Estado de la publicación
    fecha_publicacion: u64,    // Fecha de publicación (timestamp, as UNIX timestamp)
}
