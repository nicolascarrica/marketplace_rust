use ink::prelude::vec::Vec;
use ink::primitives::AccountId;
use ink::scale::{Decode, Encode};
use ink::storage::traits::StorageLayout;

use crate::types::enums::Rol;
use crate::types::calificacion::Calificacion;
use crate::types::errores::ErrorMarketplace;


/// Representa un usuario de la plataforma.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, StorageLayout)]
#[cfg_attr(feature = "std", derive(ink::scale_info::TypeInfo))]
pub struct Usuario {
    pub username: String,
    pub rol: Rol,
    pub calificaciones_como_comprador: Vec<Calificacion>,
    pub calificaciones_como_vendedor: Vec<Calificacion>,
    pub verificado: bool,
}


impl Usuario {
     /// Crea un nuevo usuario con nombre, rol y campos de reputación vacíos.
    ///
    /// # Parámetros
    /// - `username`: nombre público visible.
    /// - `rol`: rol inicial del usuario (Comprador, Vendedor o Ambos).
    /// # Retorna
    /// - Una instancia de `Usuario` lista para ser insertada en el contrato.
    pub fn new(username: String, rol: Rol) -> Self {
        Self {
            username,
            rol,
            calificaciones_como_comprador: Vec::new(),
            calificaciones_como_vendedor: Vec::new(),
            verificado: false,
        }
    }

    /// Califica al usuario como comprador.
    ///
    /// # Parámetros
    /// - `calificador`: cuenta que realiza la calificación.
    /// - `puntaje`: valor entre 1 y 5.
    /// - `orden_id`: ID único de la orden relacionada.
    ///
    /// # Errores
    /// - `PuntajeInvalido` si el puntaje no está entre 1 y 5.
    /// - `YaCalificado` si ya se calificó esta orden por esta cuenta.
    pub fn calificar_como_comprador(&mut self, calificador: AccountId, puntaje: u8, orden_id: u64) -> Result<(), ErrorMarketplace> {
        if !(1..=5).contains(&puntaje) {
            return Err(ErrorMarketplace::PuntajeInvalido);
        }

        // Evitar calificación duplicada
        if self.calificaciones_como_comprador.iter().any(|c| c.orden_id == orden_id && c.calificador == calificador) {
            return Err(ErrorMarketplace::YaCalificado);
        }

        self.calificaciones_como_comprador.push(Calificacion { calificador, puntaje, orden_id });
        Ok(())
    }

    /// Califica al usuario como vendedor.
    /// # Parámetros
    /// - `calificador`: cuenta que realiza la calificación.
    /// - `puntaje`: valor entre 1 y 5.
    /// - `orden_id`: ID único de la orden relacionada.
    ///
    /// # Errores
    /// - `PuntajeInvalido` si el puntaje no está entre 1 y 5.
    /// - `YaCalificado` si ya se calificó esta orden por esta cuenta.


    pub fn calificar_como_vendedor(&mut self, calificador: AccountId, puntaje: u8, orden_id: u64) -> Result<(), ErrorMarketplace> {
        if !(1..=5).contains(&puntaje) {
            return Err(ErrorMarketplace::PuntajeInvalido);
        }

        if self.calificaciones_como_vendedor.iter().any(|c| c.orden_id == orden_id && c.calificador == calificador) {
            return Err(ErrorMarketplace::YaCalificado);
        }

        self.calificaciones_como_vendedor.push(Calificacion { calificador, puntaje, orden_id });
        Ok(())
    }

    /// Calcula el promedio de calificaciones recibidas como comprador.
    ///
    /// # Retorna
    /// - `Some(promedio)` si hay calificaciones.
    /// - `None` si no hay ninguna calificación aún.

    pub fn promedio_como_comprador(&self) -> Option<u128> {
        if self.calificaciones_como_comprador.is_empty() {
            None
        } else {
            let total: u32 = self.calificaciones_como_comprador.iter().map(|c| c.puntaje as u32).sum();
            Some(total as u128 / self.calificaciones_como_comprador.len() as u128)
        }
    }

    /// Calcula el promedio de calificaciones recibidas como vendedor.
    /// # Retorna
    /// - `Some(promedio)` si hay calificaciones.
    /// - `None` si no hay ninguna calificación aún.
    pub fn promedio_como_vendedor(&self) -> Option<u128> {
        if self.calificaciones_como_vendedor.is_empty() {
            None
        } else {
            let total: u32 = self
                .calificaciones_como_vendedor
                .iter()
                .map(|c| c.puntaje as u32)
                .sum();
            Some(total as u128 / self.calificaciones_como_vendedor.len() as u128)
        }
    }


    /// Cambia el rol del usuario.
    /// # Parámetros
    /// - `nuevo_rol`: el nuevo rol que se asignará al usuario.
    pub fn set_rol(&mut self, nuevo_rol: Rol) {
        self.rol = nuevo_rol;
    }

    /// verifica si el usuaruo es vendedor o comprador
    /// # Retorna
    /// - `true` si el usuario es vendedor.
    /// - `false` si el usuario es comprador.

    pub fn es_vendedor(&self) -> bool {
        matches!(self.rol, Rol::Vendedor | Rol::Ambos)
    }

    pub fn es_comprador(&self) -> bool {
        matches!(self.rol, Rol::Comprador | Rol::Ambos)
    }
}

#[cfg(test)]
mod tests {
    use crate::types::ErrorMarketplace;

    use super::*;
    use ink::primitives::AccountId;

    fn account(id: u8) -> AccountId {
        AccountId::from([id; 32])
    }

    #[test]
    fn crear_usuario_funciona() {
        let user = Usuario::new("nico".to_string(), Rol::Ambos);
        assert_eq!(user.username, "nico");
        assert_eq!(user.rol, Rol::Ambos);
        assert!(!user.verificado);
        assert!(user.calificaciones_como_comprador.is_empty());
    }

    #[test]
    fn calificar_comprador_ok() {
        let mut user = Usuario::new("ana".to_string(), Rol::Comprador);
        let result = user.calificar_como_comprador(account(1), 5, 42);
        assert!(result.is_ok());
        assert_eq!(user.calificaciones_como_comprador.len(), 1);
        assert_eq!(user.calificaciones_como_comprador[0].puntaje, 5);
    }

    #[test]
    fn calificar_vendedor_duplicado_falla() {
        let mut user = Usuario::new("leo".to_string(), Rol::Vendedor);
        let _ = user.calificar_como_vendedor(account(1), 4, 100);
        let result = user.calificar_como_vendedor(account(1), 4, 100);
        assert_eq!(result, Err(ErrorMarketplace::YaCalificado));
    }

    #[test]
    fn promedio_calculo_ok() {
        let mut user = Usuario::new("juan".to_string(), Rol::Ambos);
        let _ = user.calificar_como_vendedor(account(1), 5, 1);
        let _ = user.calificar_como_vendedor(account(2), 3, 2);
        let promedio = user.promedio_como_vendedor();
        assert_eq!(promedio, Some(4));
    }

    #[test]
    fn promedio_none_si_vacio() {
        let user = Usuario::new("maria".to_string(), Rol::Comprador);
        assert_eq!(user.promedio_como_comprador(), None);
    }

    #[test]
    fn cambiar_rol_funciona() {
        let mut user = Usuario::new("lucas".to_string(), Rol::Comprador);
        user.set_rol(Rol::Ambos);
        assert_eq!(user.rol, Rol::Ambos);
    }

    #[test]
    fn es_vendedor_ok() {
        let user = Usuario::new("pedro".to_string(), Rol::Vendedor);
        assert!(user.es_vendedor());
        assert!(!user.es_comprador());
    }

    #[test]
    fn es_comprador_ok() {
        let user = Usuario::new("pedro".to_string(), Rol::Comprador);
        assert!(user.es_comprador());
        assert!(!user.es_vendedor());
    }

     fn calificar_comprador_puntaje_invalido() {
        let mut user = Usuario::new("Ana".to_string(), Rol::Vendedor);
        let result = user.calificar_como_comprador(account(1), 6, 10);
        assert_eq!(result, Err(ErrorMarketplace::PuntajeInvalido));

        let result = user.calificar_como_comprador(account(1), 0, 10);
        assert_eq!(result, Err(ErrorMarketplace::PuntajeInvalido));
    }

    #[test]
    fn calificar_comprador_duplicado_falla() {
        let mut user = Usuario::new("Ana".to_string(), Rol::Vendedor);
        let _ = user.calificar_como_comprador(account(1), 5, 10);
        let result = user.calificar_como_comprador(account(1), 3, 10);
        assert_eq!(result, Err(ErrorMarketplace::YaCalificado));
    }

    #[test]
    fn calificar_comprador_diferente_usuario_funciona() {
        let mut user = Usuario::new("Ana".to_string(), Rol::Vendedor);
        let _ = user.calificar_como_comprador(account(1), 4, 10);
        let result = user.calificar_como_comprador(account(2), 5, 10);
        assert!(result.is_ok());
        assert_eq!(user.calificaciones_como_comprador.len(), 2);
    }

    #[test]
    fn calificar_vendedor_puntaje_invalido() {
        let mut user = Usuario::new("Leo".to_string(), Rol::Comprador);
        let result = user.calificar_como_vendedor(account(3), 0, 99);
        assert_eq!(result, Err(ErrorMarketplace::PuntajeInvalido));
    }

    #[test]
    fn calificar_vendedor_ok() {
        let mut user = Usuario::new("Leo".to_string(), Rol::Comprador);
        let result = user.calificar_como_vendedor(account(3), 5, 99);
        assert!(result.is_ok());
    }

}