#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod market_place {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;
    //use ink_e2e::subxt_signer::bip39::serde::de::value::Error;

    /// Enums
    #[derive(
        Debug,
        Clone,
        Copy,
        PartialEq,
        Eq,
        ink::scale::Encode,
        ink::scale::Decode,
        ink::storage::traits::StorageLayout,
    )]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Rol {
        Comprador,
        Vendedor,
        Ambos,
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        ink::scale::Encode,
        ink::scale::Decode,
        ink::storage::traits::StorageLayout,
    )]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Categoria {
        Tecnologia,
        Indumentaria,
        Hogar,
        Alimentos,
        Otros,
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        ink::scale::Encode,
        ink::scale::Decode,
        ink::storage::traits::StorageLayout,
    )]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum EstadoOrden {
        Pendiente,
        Enviado,
        Recibido,
        Cancelada,
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        ink::scale::Encode,
        ink::scale::Decode,
        ink::storage::traits::StorageLayout,
    )]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum ErrorOrden {
        NoEsVendedor,
        NoEsComprador,
        EstadoInvalido,
        CancelacionNoSolicitada,
        CancelacionYaPendiente,
        OrdenCancelada,
        NoAutorizado,
        UsuarioYaRegistrado,
        UsuarioNoExiste,
        RolInvalido,
        ProductoNoExiste,
        StockInsuficiente,
        OrdenNoExiste,
        YaCalificado,
        PuntajeInvalido,
        CancelacionPendiente,
        CalificacionDuplicada,
    }

    //Enum Errores
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[derive(Debug, PartialEq)]

    pub enum ErrorMarketplace {
        UsuarioYaRegistrado,
        UsuarioNoExiste,
        RolInvalido,
        RolYaAsignado,
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
        PrecioInvalido,
        NombreInvalido,
    }
    #[cfg(feature = "std")]
    impl core::fmt::Display for ErrorMarketplace {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            // use ink_e2e::subxt_signer::bip39::serde::de::value::Error;

            let mensaje = match self {
                ErrorMarketplace::UsuarioYaRegistrado => "El usuario ya está registrado",
                ErrorMarketplace::UsuarioNoExiste => "El usuario no fue encontrado",
                ErrorMarketplace::RolYaAsignado => "El rol ya está asignado a este usuario",
                ErrorMarketplace::RolInvalido => "Rol inválido",
                ErrorMarketplace::ProductoNoExiste => "Producto no encontrado",
                ErrorMarketplace::StockInsuficiente => "No hay stock suficiente",
                ErrorMarketplace::OrdenNoExiste => "La orden no existe",
                ErrorMarketplace::NoEsComprador => {
                    "Solo los compradores pueden realizar esta acción"
                }
                ErrorMarketplace::PrecioInvalido => {
                    "El precio debe ser mayor a cero y no puede estar vacío"
                }
                ErrorMarketplace::NombreInvalido => "El nombre no puede estar vacío",
                ErrorMarketplace::NoEsVendedor => "Solo los vendedores pueden realizar esta acción",
                ErrorMarketplace::EstadoInvalido => "El estado no es válido para esta acción",
                ErrorMarketplace::YaCalificado => "El usuario ya fue calificado para esta orden",
                ErrorMarketplace::PuntajeInvalido => "Puntaje inválido. Debe estar entre 1 y 5",
                ErrorMarketplace::NoAutorizado => "No tiene autorización para realizar esta acción",
                ErrorMarketplace::CancelacionPendiente => {
                    "La orden ya tiene una solicitud de cancelación pendiente"
                }
                ErrorMarketplace::CalificacionDuplicada => {
                    "La calificación ya fue registrada para esta orden"
                }
            };
            write!(f, "{mensaje}")
        }
    }
    // Structs
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        ink::scale::Encode,
        ink::scale::Decode,
        ink::storage::traits::StorageLayout,
    )]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Usuario {
        username: String,
        rol: Rol,
        id: AccountId,
        calificaciones: Vec<Calificacion>,
        verificacion: bool,
    }
    impl Usuario {
        pub fn new(username: String, rol: Rol, id: AccountId) -> Self {
            Self {
                username,
                rol,
                id,
                calificaciones: Vec::new(),
                verificacion: true,
            }
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        ink::scale::Encode,
        ink::scale::Decode,
        ink::storage::traits::StorageLayout,
    )]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Calificacion {
        pub id: AccountId,
        pub puntaje: u8,
        pub id_orden: u32,
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
                id: calificador,
                puntaje,
                id_orden: orden_id as u32,
            }
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        ink::scale::Encode,
        ink::scale::Decode,
        ink::storage::traits::StorageLayout,
    )]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Producto {
        id: u32,
        nombre: String,
        descripcion: String,
        precio: u128,
        stock: u32,
        categoria: Categoria,
    }
    impl Producto {
        pub fn new(
            id: u32,
            nombre: String,
            descripcion: String,
            precio: u128,
            stock: u32,
            categoria: Categoria,
        ) -> Self {
            Self {
                id,
                nombre,
                descripcion,
                precio,
                stock,
                categoria,
            }
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        ink::scale::Encode,
        ink::scale::Decode,
        ink::storage::traits::StorageLayout,
    )]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Publicacion {
        id_publicacion: u32,
        id_vendedor: AccountId,
        producto: Producto,
    }
    impl Publicacion {
        fn new(id_publicacion: u32, id_vendedor: AccountId, producto: Producto) -> Self {
            Self {
                id_publicacion,
                id_vendedor,
                producto: Producto::new(
                    producto.id,
                    producto.nombre,
                    producto.descripcion,
                    producto.precio,
                    producto.stock,
                    producto.categoria,
                ),
            }
        }
    }
    #[derive(
        Debug,
        PartialEq,
        Eq,
        ink::scale::Encode,
        ink::scale::Decode,
        ink::storage::traits::StorageLayout,
    )]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Orden {
        pub id: u32,
        pub comprador: AccountId,
        pub vendedor: AccountId,
        pub productos: Vec<(u32, Producto)>,
        pub estado: EstadoOrden,
        pub total: u128,
        pub pendiente_cancelacion: bool,
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct MarketPlace {
        usuarios: Mapping<AccountId, Usuario>,
        productos: Mapping<u32, Producto>,
        ordenes: Mapping<u32, Orden>,
        publicaciones: Mapping<u32, Publicacion>,
        productos_por_usuario: Mapping<AccountId, Producto>,
        contador_ordenes: u32,
        contador_publicacion: u32,
        contador_productos: u32,
        //aca iria lo de reputacion, creo
    }
    impl Orden {
        pub fn new(
            id: u32,
            comprador: AccountId,
            vendedor: AccountId,
            productos: Vec<(u32, Producto)>,
            total: u128,
        ) -> Self {
            Self {
                id,
                comprador,
                vendedor,
                productos,
                total,
                estado: EstadoOrden::Pendiente,
                pendiente_cancelacion: false,
            }
        }
        pub fn marcar_enviada(&mut self, vendedor: AccountId) -> Result<(), ErrorOrden> {
            //validar que la orden no este cancelada
            if self.estado == EstadoOrden::Cancelada {
                return Err(ErrorOrden::OrdenCancelada);
            }
            //validar que quien llame sea vendedor
            if vendedor != self.vendedor {
                return Err(ErrorOrden::NoEsVendedor);
            }
            //validar que la orden este en estado "Pendiente" para poder marcarla como enviada
            if self.estado != EstadoOrden::Pendiente {
                return Err(ErrorOrden::EstadoInvalido);
            }
            //cambiar el estado a "Enviado"
            self.estado = EstadoOrden::Enviado;
            Ok(())
        }
        pub fn marcar_recibida(&mut self, comprador: AccountId) -> Result<(), ErrorOrden> {
            //validar que la orden no este cancelada
            if self.estado == EstadoOrden::Cancelada {
                return Err(ErrorOrden::OrdenCancelada);
            }
            //validar que quien llama sea el comprador
            if comprador != self.comprador {
                return Err(ErrorOrden::NoEsComprador);
            }
            //solo se marca como recibida si ya fue enviada
            if self.estado != EstadoOrden::Enviado {
                return Err(ErrorOrden::EstadoInvalido);
            }
            //cambiar el estado a "Recibido"
            self.estado = EstadoOrden::Recibido;
            Ok(())
        }
        pub fn solicitar_cancelacion(&mut self, usuario: AccountId) -> Result<(), ErrorOrden> {
            //validar que la orden no este ya cancelada
            if self.estado == EstadoOrden::Cancelada {
                return Err(ErrorOrden::OrdenCancelada);
            }
            //validar que quien solicita sea comprador o vendedor
            if usuario != self.comprador && usuario != self.vendedor {
                return Err(ErrorOrden::NoAutorizado);
            }
            //verificar si antes ya se pidio cancelar
            if self.pendiente_cancelacion {
                return Err(ErrorOrden::CancelacionYaPendiente);
            }
            //marcar como pendiente la cancelacion
            self.pendiente_cancelacion = true;
            Ok(())
        }
        pub fn confirmar_cancelacion(&mut self, usuario: AccountId) -> Result<(), ErrorOrden> {
            //verificar si la orden ya fue cancelada
            if self.estado == EstadoOrden::Cancelada {
                return Err(ErrorOrden::OrdenCancelada);
            }
            //solo un comprador o vendedor puede confirmar la cancelacion
            if usuario != self.comprador && usuario != self.vendedor {
                return Err(ErrorOrden::NoAutorizado);
            }
            //no se puede confirmar la cancelacion si no hay una cancelacion pendiente
            if !self.pendiente_cancelacion {
                return Err(ErrorOrden::CancelacionNoSolicitada);
            }
            //cambiar el estado a "Cancelada" y limpiar el flag
            self.estado = EstadoOrden::Cancelada;
            self.pendiente_cancelacion = false;
            Ok(())
        }
    }
    impl MarketPlace {
        /// Crea una nueva instancia del contrato MarketPlace.
        /// # Retorna
        /// Un contrato con mapas vacíos y contadores en cero.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                usuarios: Mapping::default(),
                productos: Mapping::default(),
                ordenes: Mapping::default(),
                publicaciones: Mapping::default(),
                productos_por_usuario: Mapping::default(),
                contador_ordenes: 0,
                contador_publicacion: 0,
                contador_productos: 0,
            }
        }
        /// Registra un nuevo usuario en el sistema con su rol.
        /// # Parámetros
        /// - `username`: nombre público visible del usuario.
        /// - `rol`: rol del usuario (Comprador, Vendedor o Ambos).
        /// # Retorna
        /// - `Ok(())` si el registro fue exitoso.
        /// - `Err(ErrorMarketplace::UsuarioYaRegistrado)` si el usuario ya existe.

        #[ink(message)]

        pub fn registrar_usuario(&mut self, username: String, rol: Rol) -> Result<(), String> {
            ///deberiamos ver como manejar el error
            let caller = self.env().caller(); //id

            if self.usuarios.contains(&caller) {
                return Err(String::from("El usuario ya está registrado"));
            }

            let nuevo_usuario = Usuario::new(username, rol, caller);
            self.usuarios.insert(caller, &nuevo_usuario);

            Ok(()) //no devuelve nada porque solo inserta en el map de sistema
        }
        fn verificar_rol_es_diferente(
            &self,
            id: AccountId,
            nuevo_rol: Rol,
        ) -> Result<(), ErrorMarketplace> {
            let usuario = self.verificar_usuario_existe(id)?;
            if usuario.rol == nuevo_rol {
                Err(ErrorMarketplace::RolYaAsignado)
            } else {
                Ok(())
            }
        }
        #[ink(message)]
        pub fn modificar_rol(&mut self, nuevo_rol: Rol) -> Result<(), ErrorMarketplace> {
            //o que no devuelva nada, todavia no se
            let caller = self.env().caller();

            // Verifica si el usuario existe
            let mut usuario = self.verificar_usuario_existe(caller)?;

            // Verifica que el nuevo rol sea diferente
            self.verificar_rol_es_diferente(caller, nuevo_rol)?;

            // Actualiza el rol
            usuario.rol = nuevo_rol;

            self.usuarios.insert(caller, &usuario);
            Ok(())
        }

        //Helper verificar usuario exista
        fn verificar_usuario_existe(&self, id: AccountId) -> Result<Usuario, ErrorMarketplace> {
            self.usuarios
                .get(&id)
                .ok_or(ErrorMarketplace::UsuarioNoExiste)
        }
        //Helper verificar que el usuario tenga el rol correcto
        fn verificar_rol_vendedor(&self, id: AccountId) -> Result<(), ErrorMarketplace> {
            let usuario = self.verificar_usuario_existe(id)?;
            if usuario.rol == Rol::Ambos || usuario.rol == Rol::Vendedor {
                Ok(())
            } else {
                Err(ErrorMarketplace::RolInvalido)
            }
        }

        //Helper para validar que el producto tenga un nombre, precio y stock
        fn validacion_producto(
            &self,
            nombre: &String,
            precio: &u128,
            stock: &u32,
        ) -> Result<(), ErrorMarketplace> {
            if *stock <= 0 {
                return Err(ErrorMarketplace::StockInsuficiente);
            }
            if *precio <= 0 {
                return Err(ErrorMarketplace::PrecioInvalido);
            }
            if nombre.is_empty() || nombre.trim().is_empty() {
                return Err(ErrorMarketplace::NombreInvalido);
            }
            Ok(())
        }
        //Helper para obtener una publicacion por id
        fn obtener_publicacion(
            &self,
            id_publicacion: u32,
        ) -> Result<Publicacion, ErrorMarketplace> {
            self.publicaciones
                .get(&id_publicacion)
                .ok_or_else(|| ErrorMarketplace::ProductoNoExiste)
        }
        //Helper para verificar que el usuario es el owner de la publicacion
        fn verificar_owner_publicacion(
            &self,
            id_publicacion: u32,
            id_vendedor: AccountId,
        ) -> Result<(), ErrorMarketplace> {
            let publicacion = self.obtener_publicacion(id_publicacion)?;
            if publicacion.id_vendedor != id_vendedor {
                return Err(ErrorMarketplace::NoAutorizado);
            }
            Ok(())
        }
        //Helper para obtener un nuevo id de publicacion
        fn obtener_nuevo_id_publicacion(&mut self) -> u32 {
            self.contador_publicacion += 1;
            self.contador_publicacion
        }

        //Publicar producto
        #[ink(message)]
        pub fn publicar_producto(&mut self, producto: Producto) -> Result<(), ErrorMarketplace> {
            let caller = self.env().caller();
            self.verificar_usuario_existe(caller)?;
            self.verificar_rol_vendedor(caller)?;
            self.validacion_producto(&producto.nombre, &producto.precio, &producto.stock)?;

            // Generamos un nuevo ID para la publicación
            let id_publicacion = self.obtener_nuevo_id_publicacion();

            // Creamos una nueva publicación
            let nueva_publicacion = Publicacion::new(id_publicacion, caller, producto.clone());

            //Guardamos la publicación en el mapping
            self.publicaciones
                .insert(id_publicacion, &nueva_publicacion);
            Ok(())
        }
        /// Obtener lista de publicaciones de un vendedor por su ID
        #[ink(message)]
        pub fn obtener_publicaciones_por_vendedor(&self, vendedor: AccountId) -> Vec<Publicacion> {
            (1..=self.contador_productos)
                .filter_map(|i| self.publicaciones.get(&i))
                .filter(|publicacion| publicacion.id_vendedor == vendedor)
                .collect()
        }
    }
}

/// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
/// module and test functions are marked with a `#[test]` attribute.
/// The below code is technically just normal Rust code.
#[cfg(test)]
mod tests {
    use ink::primitives::AccountId;

    use crate::market_place::{
        Calificacion, ErrorOrden, EstadoOrden, MarketPlace, Orden, Rol, Usuario,
    };

    /// Imports all the definitions from the outer scope so we can use them here.
    use super::*;

    /// We test if the default constructor does its job.
    /*     #[ink::test]
    fn default_works() {
        let MarketPlace = MarketPlace::default();
        assert_eq!(MarketPlace.get(), false);
    }

    /// We test a simple use case of our contract.
    #[ink::test]
    fn it_works() {
        let mut market_place = MarketPlace::new();
        assert_eq!(market_place.get(), false);
        market_place.flip();
        assert_eq!(market_place.get(), true);
    } */
    fn account(id: u8) -> AccountId {
        AccountId::from([id; 32])
    }
    fn userComprador() -> Usuario {
        Usuario::new("user1".to_string(), Rol::Comprador, account(1))
    }
    fn userVendedor() -> Usuario {
        Usuario::new("user2".to_string(), Rol::Vendedor, account(2))
    }
    fn userAmbos() -> Usuario {
        Usuario::new("user3".to_string(), Rol::Ambos, account(3))
    }
    #[test]
    fn crear_calificacion_ok() {
        let calif = Calificacion::new(account(1), 5, 42);
        assert_eq!(calif.id, account(1));
        assert_eq!(calif.puntaje, 5);
        assert_eq!(calif.id_orden, 42);
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
        assert_eq!(res, Err(ErrorOrden::NoAutorizado));
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
    fn test_solicitar_cancelacion_ok() {
        let mut orden = Orden::new(1, account(1), account(2), vec![], 100);
        let res = orden.solicitar_cancelacion(account(1));
        assert_eq!(res, Ok(()));
        assert!(orden.pendiente_cancelacion);
    }

    #[test]
    fn test_confirmar_cancelacion_ok() {
        let mut orden = Orden::new(1, account(1), account(2), vec![], 100);
        orden.pendiente_cancelacion = true;
        let res = orden.confirmar_cancelacion(account(2));
        assert_eq!(res, Ok(()));
        assert_eq!(orden.estado, EstadoOrden::Cancelada);
    }

    #[test]
    fn test_confirmar_cancelacion_sin_solicitar() {
        let mut orden = Orden::new(1, account(1), account(2), vec![], 100);
        let res = orden.confirmar_cancelacion(account(2));
        assert_eq!(res, Err(ErrorOrden::CancelacionPendiente));
    }

    #[test]
    fn test_marcar_recibida_estado_invalido() {
        let mut orden = Orden::new(1, account(1), account(2), vec![], 100);
        let res = orden.marcar_recibida(account(1));
        assert_eq!(res, Err(ErrorOrden::EstadoInvalido));
    }

    #[test]
    fn crear_usuario_funciona() {
        let user = Usuario::new("nico".to_string(), Rol::Ambos, account(1));
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
/*
/// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
///
/// When running these you need to make sure that you:
/// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
/// - Are running a Substrate node which contains `pallet-contracts` in the background
#[cfg(all(test, feature = "e2e-tests"))]
mod e2e_tests {
    /// Imports all the definitions from the outer scope so we can use them here.
    use super::*;

    /// A helper function used for calling contract messages.
    use ink_e2e::ContractsBackend;

    /// The End-to-End test `Result` type.
    type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    /// We test that we can upload and instantiate the contract using its default constructor.
    #[ink_e2e::test]
    async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        // Given
        let mut constructor = MarketPlaceRef::default();

        // When
        let contract = client
            .instantiate("MarketPlace", &ink_e2e::alice(), &mut constructor)
            .submit()
            .await
            .expect("instantiate failed");
        let call_builder = contract.call_builder::<MarketPlace>();

        // Then
        let get = call_builder.get();
        let get_result = client.call(&ink_e2e::alice(), &get).dry_run().await?;
        assert!(matches!(get_result.return_value(), false));

        Ok(())
    }

    /// We test that we can read and write a value from the on-chain contract.
    #[ink_e2e::test]
    async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        // Given
        let mut constructor = MarketPlaceRef::new(false);
        let contract = client
            .instantiate("MarketPlace", &ink_e2e::bob(), &mut constructor)
            .submit()
            .await
            .expect("instantiate failed");
        let mut call_builder = contract.call_builder::<MarketPlace>();

        let get = call_builder.get();
        let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
        assert!(matches!(get_result.return_value(), false));

        // When
        let flip = call_builder.flip();
        let _flip_result = client
            .call(&ink_e2e::bob(), &flip)
            .submit()
            .await
            .expect("flip failed");

        // Then
        let get = call_builder.get();
        let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
        assert!(matches!(get_result.return_value(), true));

        Ok(())
    }
} */
