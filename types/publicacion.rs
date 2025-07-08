use crate::types::enums::{Categoria, EstadoPublicacion};
use ink::prelude::string::String;
use ink::primitives::AccountId;
use ink::scale::{Decode, Encode};

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(ink::scale_info::TypeInfo))]
pub struct Publicacion {
    pub id_publicacion: u32,       // ID de la publicación
    pub id_vendedor: AccountId,    // ID del vendedor
    pub nombre_producto: String,   // Nombre del producto
    pub descripcion: String,       // Descripción del producto
    pub stock: u32,                // Cantidad disponible del producto
    pub categoria: Categoria,      // Categoría del producto
    pub precio: u128,              // Precio de la publicación
    pub estado: EstadoPublicacion, // Estado de la publicación
    pub fecha_publicacion: u64,    // Fecha de publicación (timestamp, as UNIX timestamp)
}

impl Publicacion {
    pub fn new(
        id_publicacion: u32,
        id_vendedor: AccountId,
        nombre_producto: String,
        descripcion: String,
        stock: u32,
        categoria: Categoria,
        precio: u128,
        estado: EstadoPublicacion,
        fecha_publicacion: u64,
    ) -> Self {
        Self {
            id_publicacion,
            id_vendedor,
            nombre_producto,
            descripcion,
            stock,
            categoria,
            precio,
            estado,
            fecha_publicacion: 0, // Se actualizará en el contrato
        }
    }
}
