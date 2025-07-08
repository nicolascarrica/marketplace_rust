use ink::primitives::AccountId;
use parity_scale_codec::{Decode, Encode};

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(ink::scale_info::TypeInfo))]

/// Representa una calificación emitida por un usuario en una orden.
///
/// Contiene:
/// - `calificador`: cuenta que realiza la calificación.
/// - `puntaje`: valor entre 1 y 5.
/// - `orden_id`: ID de la orden relacionada.
pub struct Calificacion {
    pub calificador: AccountId,
    pub puntaje: u8,
    pub orden_id: u64,
}

impl Calificacion {
    /// Crea una nueva calificación válida.
    ///
    /// # Parámetros
    /// - `calificador`: cuenta que realiza la calificación
    /// - `puntaje`: entero entre 1 y 5
    /// - `orden_id`: ID de la orden asociada
    ///
    /// # Retorna
    /// - Instancia de `Calificacion`
    pub fn new(calificador: AccountId, puntaje: u8, orden_id: u64) -> Self {
        Self {
            calificador,
            puntaje,
            orden_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn account(id: u8) -> AccountId {
        AccountId::from([id; 32])
    }

    #[test]
    fn crear_calificacion_ok() {
        let calif = Calificacion::new(account(1), 5, 42);
        assert_eq!(calif.calificador, account(1));
        assert_eq!(calif.puntaje, 5);
        assert_eq!(calif.orden_id, 42);
    }

    #[test]
    fn calificacion_valor_maximo() {
        let calif = Calificacion::new(account(2), 5, 100);
        assert_eq!(calif.puntaje, 5);
    }

    #[test]
    fn calificacion_valor_minimo() {
        let calif = Calificacion::new(account(3), 1, 100);
        assert_eq!(calif.puntaje, 1);
    }
}