/// Representa un error al llamar a un metodo del sistema.
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
#[derive(Debug,PartialEq)]

pub enum ErrorMarketplace {
    UsuarioYaRegistrado,
    UsuarioNoExiste,
    RolInvalido,
    ProductoNoExiste,
    StockInsuficiente,
    OrdenNoExiste,
    NoEsComprador,
    NoEsVendedor,
    EstadoInvalido,
    YaCalificado,
    PuntajeInvalido,
    NoAutorizado,
    CancelacionPendiente,
    CalificacionDuplicada,
}

/// implemntacion de Display para ErrorMarketplace
#[cfg(feature = "std")]
impl core::fmt::Display for ErrorMarketplace {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mensaje = match self {
            ErrorMarketplace::UsuarioYaRegistrado => "El usuario ya está registrado",
            ErrorMarketplace::UsuarioNoExiste => "El usuario no fue encontrado",
            ErrorMarketplace::RolInvalido => "Rol inválido",
            ErrorMarketplace::ProductoNoExiste => "Producto no encontrado",
            ErrorMarketplace::StockInsuficiente => "No hay stock suficiente",
            ErrorMarketplace::OrdenNoExiste => "La orden no existe",
            ErrorMarketplace::NoEsComprador => "Solo los compradores pueden realizar esta acción",
            ErrorMarketplace::NoEsVendedor => "Solo los vendedores pueden realizar esta acción",
            ErrorMarketplace::EstadoInvalido => "El estado no es válido para esta acción",
            ErrorMarketplace::YaCalificado => "El usuario ya fue calificado para esta orden",
            ErrorMarketplace::PuntajeInvalido => "Puntaje inválido. Debe estar entre 1 y 5",
            ErrorMarketplace::NoAutorizado => "No tiene autorización para realizar esta acción",
            ErrorMarketplace::CancelacionPendiente => "La orden ya tiene una solicitud de cancelación pendiente",
            ErrorMarketplace::CalificacionDuplicada => "La calificación ya fue registrada para esta orden",
        };
        write!(f, "{mensaje}")
    }
}