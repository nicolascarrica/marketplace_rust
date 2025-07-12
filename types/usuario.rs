use ink::prelude::vec::Vec;
use ink::primitives::AccountId;
use ink::scale::{Decode, Encode};
use ink::storage::traits::StorageLayout;

use crate::types::enums::Rol;
use crate::types::errores::ErrorMarketplace;

/// Representa un usuario de la plataforma.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, StorageLayout)]
#[cfg_attr(feature = "std", derive(ink::scale_info::TypeInfo))]
pub struct Usuario {
    pub username: String,
    pub rol: Rol,
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
            verificado: false,
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
}
