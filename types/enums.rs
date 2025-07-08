use ink::{scale::{Decode, Encode}, storage::traits::StorageLayout};

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, StorageLayout)]
#[cfg_attr(feature = "std", derive(ink::scale_info::TypeInfo))]
pub enum Rol {
    Comprador,
    Vendedor,
    Ambos,
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, StorageLayout)]
#[cfg_attr(feature = "std", derive(ink::scale_info::TypeInfo))]
pub enum Categoria {
    Tecnologia,
    Indumentaria,
    Hogar,
    Alimentos,
    Otros,
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, StorageLayout)]
#[cfg_attr(feature = "std", derive(ink::scale_info::TypeInfo))]
pub enum EstadoOrden {
    Pendiente,
    Enviado,
    Recibido,
    Cancelada,
}
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(ink::scale_info::TypeInfo))]
pub enum EstadoPublicacion {
    Activa,
    Pausada,
    Eliminada,
    Agotada,
}
