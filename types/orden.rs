use ink::primitives::AccountId;
use ink::scale::{Decode, Encode};
use ink::storage::traits::StorageLayout;

use crate::types::enums::EstadoOrden;
use crate::types::errores::ErrorMarketplace;

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, StorageLayout)]
#[cfg_attr(feature = "std", derive(ink::scale_info::TypeInfo))]
/// Representa una Orden de la plataforma.
pub struct Orden {
    pub id: u64,
    pub comprador: AccountId,
    pub vendedor: AccountId,
    pub productos: Vec<(u64, u32)>, // (producto_id, cantidad)
    pub total: u128,
    pub estado: EstadoOrden,
}

impl Orden {
    /// Crea una nueva orden de compra.
    ///
    /// # Parámetros
    /// - `id`: ID único de la orden.
    /// - `comprador`: Cuenta del comprador.
    /// - `vendedor`: Cuenta del vendedor.
    /// - `productos`: Lista de tuplas (producto_id, cantidad).
    /// - `total`: Monto total de la orden.
    ///
    /// # Retorna
    /// Una nueva instancia de `Orden` en estado `Pendiente`.
    pub fn new(
        id: u64,
        comprador: AccountId,
        vendedor: AccountId,
        productos: Vec<(u64, u32)>,
        total: u128,
    ) -> Self {
        Self {
            id,
            comprador,
            vendedor,
            productos,
            total,
            estado: EstadoOrden::Pendiente,
        }
    }

    /// Marca la orden como enviada.
    ///
    /// # Parámetros
    /// - `caller`: Cuenta que intenta realizar la acción.
    ///
    /// # Retorna
    /// - `Ok(())` si fue exitosa.
    /// - `Err(NoAutorizado)` si el caller no es el vendedor.
    /// - `Err(EstadoInvalido)` si la orden no está en estado `Pendiente`.
    pub fn marcar_enviada(&mut self, caller: AccountId) -> Result<(), ErrorMarketplace> {
        if self.vendedor != caller {
            return Err(ErrorMarketplace::NoAutorizado);
        }
        if self.estado != EstadoOrden::Pendiente {
            return Err(ErrorMarketplace::EstadoInvalido);
        }
        self.estado = EstadoOrden::Enviado;
        Ok(())
    }

    /// Marca la orden como recibida.
    ///
    /// # Parámetros
    /// - `caller`: Cuenta que intenta realizar la acción.
    ///
    /// # Retorna
    /// - `Ok(())` si fue exitosa.
    /// - `Err(NoAutorizado)` si el caller no es el comprador.
    /// - `Err(EstadoInvalido)` si la orden no está en estado `Enviada`.
    pub fn marcar_recibida(&mut self, caller: AccountId) -> Result<(), ErrorMarketplace> {
        if self.comprador != caller {
            return Err(ErrorMarketplace::NoAutorizado);
        }
        if self.estado != EstadoOrden::Enviado {
            return Err(ErrorMarketplace::EstadoInvalido);
        }
        self.estado = EstadoOrden::Recibido;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn account(id: u8) -> AccountId {
        AccountId::from([id; 32])
    }

    #[test]
    fn test_orden_enviada_ok() {
        let mut orden = Orden::new(1, account(1), account(2), vec![], 100);
        let res = orden.marcar_enviada(account(2));
        assert_eq!(res, Ok(()));
        assert_eq!(orden.estado, EstadoOrden::Enviado);
    }

    #[test]
    fn test_orden_enviada_no_autorizado() {
        let mut orden = Orden::new(1, account(1), account(2), vec![], 100);
        let res = orden.marcar_enviada(account(3));
        assert_eq!(res, Err(ErrorMarketplace::NoAutorizado));
    }

    #[test]
    fn test_orden_recibida_ok() {
        let mut orden = Orden::new(1, account(1), account(2), vec![], 100);
        orden.estado = EstadoOrden::Enviado;
        let res = orden.marcar_recibida(account(1));
        assert_eq!(res, Ok(()));
        assert_eq!(orden.estado, EstadoOrden::Recibido);
    }

    #[test]
    fn test_marcar_recibida_estado_invalido() {
        let mut orden = Orden::new(1, account(1), account(2), vec![], 100);
        let res = orden.marcar_recibida(account(1));
        assert_eq!(res, Err(ErrorMarketplace::EstadoInvalido));
    }
}
