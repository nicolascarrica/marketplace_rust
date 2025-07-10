#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod market_place {
    use std::vec;

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
        PublicacionNoExiste,
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
        NoHayPublicaciones,
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
                ErrorMarketplace::NoHayPublicaciones => {
                    "No hay publicaciones disponibles de ese vendedor"
                }
                ErrorMarketplace::PublicacionNoExiste => "La publicación solicitada no existe",
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
        /// Registra un nuevo usuario en el sistema con su rol.
        /// # Parámetros
        /// - `username`: nombre público visible del usuario.
        /// - `rol`: rol del usuario (Comprador, Vendedor o Ambos).
        /// # Retorna
        /// - `Ok(())` si el registro fue exitoso.
        /// - `Err(ErrorMarketplace::UsuarioYaRegistrado)` si el usuario ya existe.
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
        pub cancelacion_solicitada_por: Option<Rol>,
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct MarketPlace {
        usuarios: Mapping<AccountId, Usuario>,
        productos: Mapping<u32, Producto>,
        ordenes: Mapping<u32, Orden>,
        publicaciones: Mapping<u32, Publicacion>, //id_publicacion -> Publicacion
        publicaciones_por_vendedor: Mapping<AccountId, Vec<u32>>,
        productos_por_usuario: Mapping<AccountId, Vec<u32>>,
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
                cancelacion_solicitada_por: None,
            }
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
                publicaciones_por_vendedor: Mapping::default(),
                contador_ordenes: 0,
                contador_publicacion: 0,
                contador_productos: 0,
            }
        }

        //FUNCIONES AUXILIARES
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
                .ok_or_else(|| ErrorMarketplace::PublicacionNoExiste)
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

        fn mostrar_publicaciones_propias(
            &self,
            id: AccountId,
        ) -> Result<Vec<Publicacion>, ErrorMarketplace> {
            match self.publicaciones_por_vendedor.get(&id) {
                Some(ids) => {
                    let mut publicaciones = Vec::new();
                    for id_pub in ids {
                        if let Some(publicacion) = self.publicaciones.get(&id_pub) {
                            publicaciones.push(publicacion.clone());
                        }
                    }
                    Ok(publicaciones)
                }
                None => Err(ErrorMarketplace::NoHayPublicaciones),
            }
        }

        #[ink(message)]
        pub fn registrar_usuario(&mut self, username: String, rol: Rol) -> Result<(), String> {
            //deberiamos ver como manejar el error
            let caller = self.env().caller(); //id

            if self.usuarios.contains(&caller) {
                return Err(String::from("El usuario ya está registrado"));
            }

            let nuevo_usuario = Usuario::new(username, rol, caller);
            self.usuarios.insert(caller, &nuevo_usuario);

            Ok(()) //no devuelve nada porque solo inserta en el map de sistema
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
        pub fn obtener_publicaciones_por_vendedor(
            &self,
            vendedor: AccountId,
        ) -> Result<Vec<Publicacion>, ErrorMarketplace> {
            self.mostrar_publicaciones_propias(vendedor)
        }

        //Ordenar producto
        #[ink(message)]
        pub fn crear_orden(
            &mut self,
            id_publicacion: u32,
            cant_producto: u16,
        ) -> Result<(), ErrorMarketplace> {
            let caller = self.env().caller();

            // Verificar que el usuario exista
            let usuario = self.verificar_usuario_existe(caller)?;

            // Solo compradores o ambos pueden comprar
            if usuario.rol == Rol::Vendedor {
                return Err(ErrorMarketplace::RolInvalido);
            }

            // Verificar que la publicación exista
            let publicacion = self.obtener_publicacion(id_publicacion)?;

            // Crear nueva orden
            let nueva_id = self.contador_ordenes;
            let orden = Orden::new(
                nueva_id,
                caller,
                publicacion.id_vendedor,
                vec![(cant_producto, publicacion.producto.clone())],
                publicacion.producto.precio,
            );

            self.ordenes.insert(nueva_id, &orden);
            self.contador_ordenes += 1;

            Ok(())
        }

        #[ink(message)]
        pub fn marcar_orden_como_enviada(&mut self, id_orden: u32) -> Result<(), ErrorOrden> {
            let caller = self.env().caller();
            // Busca la orden con el ID dado dentro del Mapping ordenes
            if let Some(mut orden) = self.ordenes.get(id_orden) {
                //El método get de Mapping te devuelve una copia de la orden
                //validar que quien llame sea vendedor
                if caller != self.vendedor {
                    return Err(ErrorOrden::NoEsVendedor);
                }
                //validar que la orden no este cancelada
                if orden.estado == EstadoOrden::Cancelada {
                    return Err(ErrorOrden::OrdenCancelada);
                }
                //como la orden se pone por default en estado "Pendiente", no necesito preguntar si esta pendiente para cambiarla(?

                //cambiar el estado de la orden a "Enviado"
                orden.estado = EstadoOrden::Enviado;
                // Guarda nuevamente la orden modificada en el Mapping para que persista en el contrato
                self.ordenes.insert(id_orden, &orden);
                Ok(())
            } else {
                Err(ErrorOrden::OrdenNiExiste)
            }
        }

        #[ink(message)]
        pub fn marcar_orden_como_recibida(&mut self, id_orden: u32) -> Result<(), ErrorOrden> {
            let caller = self.env().caller();
            // Busca la orden con el ID dado dentro del Mapping ordenes
            if let Some(mut orden) = self.ordenes.get(id_orden) {
                //El método get de Mapping te devuelve una copia de la orden
                //validar que quien llame sea comprador
                if caller != orden.comprador {
                    return Err(ErrorOrden::NoEsComprador);
                }
                //validar que la orden no este cancelada
                if orden.estado == EstadoOrden::Cancelada {
                    return Err(ErrorOrden::OrdenCancelada);
                }
                // Solo puede marcarse como recibida si fue enviada previamente
                if orden.estado != EstadoOrden::Enviado {
                    return Err(ErrorOrden::EstadoInvalido);
                }
                //cambiar el estado de la orden a "Recibido"
                orden.estado = EstadoOrden::Recibido;
                // Guarda nuevamente la orden modificada en el Mapping para que persista en el contrato
                self.ordenes.insert(id_orden, &orden);
                Ok(())
            } else {
                Err(ErrorOrden::OrdenNoExiste)
            }
        }

        #[ink(message)]
        pub fn solicitar_cancelacion(&mut self, id_orden: u32) -> Result<(), ErrorOrden> {
            let caller = self.env().caller();
            // Busca la orden con el ID dado dentro del Mapping ordenes
            if let Some(mut orden) = self.ordenes.get(id_orden) {
                //El método get de Mapping te devuelve una copia de la orden
                // Solo el comprador o el vendedor pueden solicitar la cancelación
                if caller != orden.comprador && caller != orden.vendedor {
                    return Err(ErrorOrden::NoAutorizado);
                }
                //validar que la orden no este cancelada
                if orden.estado == EstadoOrden::Cancelada {
                    return Err(ErrorOrden::OrdenCancelada);
                }
                // Determina si quien llama es comprador o vendedor
                let rol_llamada = if caller == orden.comprador {
                    Rol::Comprador
                } else {
                    Rol::Vendedor
                };

                match orden.cancelacion_solicitada_por {
                    //nadie solicito la cancelacion antes => se guarda quien la solicita y se deja pendiente
                    None => {
                        orden.pendiente_cancelacion = true;
                        // Guarda nuevamente la orden modificada en el Mapping para que persista en el contrato
                        orden.cancelacion_solicitada_por = Some(rol_llamada);
                        self.ordenes.insert(id_orden, &orden);
                        Ok(())
                    }
                    //alguien ya la habia solicitado la cancelacion => se confirma la cancelacion
                    Some(previo) if previo != rol_llamada => {
                        //ambos acordaron => cancelar directamente
                        orden.estado = EstadoOrden::Cancelada;
                        //ya se proceso la cancelacion reseteo las variables
                        orden.pendiente_cancelacion = false;
                        orden.cancelacion_solicitada_por = None;
                        // Guarda nuevamente la orden modificada en el Mapping para que persista en el contrato
                        self.ordenes.insert(id_orden, &orden);
                        Ok(())
                    }
                    //el mismo usuario ya habia solicitado la cancelacion => no puede repetir la solicitud
                    Some(_) => Err(ErrorOrden::CancelacionYaPendiente),
                }
            } else {
                Err(ErrorOrden::OrdenNoExiste)
            }
        }

        #[ink(message)]
        pub fn mostrar_productos_propios(&self, id: AccountId) -> Vec<Producto> {
            self.productos_por_usuario
                .get(&id)
                .map_or(Vec::new(), |ids_producto| {
                    ids_producto
                        .iter()
                        .filter_map(|id_producto| {
                            self.productos.get(id_producto).map(|p| p.clone())
                        })
                        .collect()
                })
        }
    }
}

/// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
/// module and test functions are marked with a `#[test]` attribute.
/// The below code is technically just normal Rust code.
#[cfg(test)]
mod tests {
    use ink::{env::test, primitives::AccountId};

    use crate::market_place::{
        Calificacion, Categoria, ErrorMarketplace, ErrorOrden, EstadoOrden, MarketPlace, Orden, Producto, Rol, Usuario
    };

    /// Imports all the definitions from the outer scope so we can use them here.
    use super::*;


    fn account(id: u8) -> AccountId {
        AccountId::from([id; 32])
    }

    fn nuevo_contrato() -> MarketPlace {
        MarketPlace::new()
    }

    fn contract_dummy() -> MarketPlace {
        let mut contract = MarketPlace::new();
        // Registramos algunos usuarios con diferentes roles
        contract.registrar_usuario("user1".to_string(), Rol::Comprador);
        contract.registrar_usuario("user2".to_string(), Rol::Vendedor);
        contract.registrar_usuario("user3".to_string(), Rol::Ambos);
        contract
    }
    fn producto_dummy() -> Producto {
        Producto::new(
            1,
            "Producto 1".to_string(),
            "Descripción del producto 1".to_string(),
            100,
            10,
            Categoria::Tecnologia,
        )
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
        assert_eq!(res, Err(ErrorOrden::NoEsVendedor));
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
        assert_eq!(res, Err(ErrorOrden::CancelacionNoSolicitada));
    }

    #[test]
    fn test_marcar_recibida_estado_invalido() {
        let mut orden = Orden::new(1, account(1), account(2), vec![], 100);
        let res = orden.marcar_recibida(account(1));
        assert_eq!(res, Err(ErrorOrden::EstadoInvalido));
    }

    #[ink::test]
    fn registrar_usuario_ok() {
        let mut contrato = MarketPlace::new();
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account(1));
        let res = contrato.registrar_usuario("nico".to_string(), Rol::Comprador);
        assert_eq!(res, Ok(()));
    }

    #[ink::test]
    fn registrar_usuario_ya_existente_falla() {
        let mut contrato = nuevo_contrato();
        test::set_caller::<ink::env::DefaultEnvironment>(account(1));
        let _ = contrato.registrar_usuario("nico".to_string(), Rol::Comprador);
        let res = contrato.registrar_usuario("nico".to_string(), Rol::Vendedor);

        assert_eq!(res, Err("El usuario ya está registrado".to_string()));
    }

    #[ink::test]
    fn modificar_rol_ok() {
        let mut contrato = nuevo_contrato();
        test::set_caller::<ink::env::DefaultEnvironment>(account(2));
        let _ = contrato.registrar_usuario("ana".to_string(), Rol::Comprador);

        let res = contrato.modificar_rol(Rol::Ambos);
        assert_eq!(res, Ok(()));
    }

    #[ink::test]
    fn modificar_rol_igual_falla() {
        let mut contrato = nuevo_contrato();
        test::set_caller::<ink::env::DefaultEnvironment>(account(3));
        let _ = contrato.registrar_usuario("luis".to_string(), Rol::Vendedor);

        let res = contrato.modificar_rol(Rol::Vendedor);
        assert_eq!(res, Err(ErrorMarketplace::RolYaAsignado));
    }

    
    // #[test]
    // fn verificar_rol_vendedor_ok() {
    //     let contract = contract_dummy();
    //     let res = contract.verificar_rol_vendedor(account(1));
    //     assert_eq!(res, Ok(()));
    // }
    // #[test]
    // fn verificar_rol_vendedor_falla_si_no_es_vendedor() {
    //     let contract = contract_dummy();
    //     let res = contract.verificar_rol_vendedor(account(2));
    //     assert_eq!(res, Err(market_place::ErrorMarketplace::RolInvalido));
    // }
    // #[test]
    // fn verificar_usuario_existe_ok() {
    //     let contract = contract_dummy();
    //     let usuario = Usuario::new("test".to_string(), Rol::Comprador, account(4));

    //     let res = contract.verificar_usuario_existe(account(4));
    //     assert_eq!(res, Ok(usuario));
    // }

    // #[test]
    // fn verificar_usuario_existe_falla_si_no_existe() {
    //     let contract = MarketPlace::new();
    //     let user4 = Usuario::new("test".to_string(), Rol::Comprador, account(4));
    //     let res = contract.verificar_usuario_existe(account(4));
    //     assert_eq!(res, Err(ErrorMarketplace::UsuarioNoExiste));
    // }

    // fn validacion_producto_ok() {
    //     let contract = MarketPlace::new();
    //     let nombre = String::from("Producto válido");
    //     let precio = 100u128;
    //     let stock = 10u32;
    //     let res = contract.validacion_producto(&nombre, &precio, &stock);
    //     assert_eq!(res, Ok(()));
    // }
    // #[test]
    // fn validacion_producto_stock_insuficiente() {
    //     let contract = MarketPlace::new();
    //     let nombre = String::from("Producto");
    //     let precio = 100u128;
    //     let stock = 0u32;
    //     let res = contract.validacion_producto(&nombre, &precio, &stock);
    //     assert_eq!(res, Err(ErrorMarketplace::StockInsuficiente));
    // }
    // #[test]
    // fn validacion_producto_precio_invalido() {
    //     let contract = MarketPlace::new();
    //     let nombre = String::from("Producto");
    //     let precio = 0u128;
    //     let stock = 5u32;
    //     let res = contract.validacion_producto(&nombre, &precio, &stock);
    //     assert_eq!(res, Err(ErrorMarketplace::PrecioInvalido));
    // }
    // #[test]
    // fn validacion_producto_nombre_vacio() {
    //     let contract = MarketPlace::new();
    //     let nombre = String::from("");
    //     let precio = 100u128;
    //     let stock = 5u32;
    //     let res = contract.validacion_producto(&nombre, &precio, &stock);
    //     assert_eq!(res, Err(ErrorMarketplace::NombreInvalido));
    // }
    // #[test]
    // fn validacion_producto_nombre_espacios() {
    //     let contract = MarketPlace::new();
    //     let nombre = String::from("   ");
    //     let precio = 100u128;
    //     let stock = 5u32;
    //     let res = contract.validacion_producto(&nombre, &precio, &stock);
    //     assert_eq!(res, Err(ErrorMarketplace::NombreInvalido));
    // }
    // #[test]
    // fn obtener_publicacion_no_existe() {
    //     let contract = MarketPlace::new();
    //     let res = contract.obtener_publicacion(42);
    //     assert_eq!(res, Err(ErrorMarketplace::PublicacionNoExiste));
    // }
    // #[test]
    // fn obtener_publicacion_ok() {
    //     let mut contract = contract_dummy();
    //     let user2: AccountId = account(2);
    //     ink::env::test::set_caller::<ink::env::DefaultEnvironment>(user2);
    //     let producto = producto_dummy();
    //     let publicacion1 = contract.publicar_producto(producto.clone());

    //     let res = contract.obtener_publicacion(publicacion1.id);
    //     assert_eq!(res, Ok(publicacion1));
    // }
    // #[test]
    // fn verificar_owner_publicacion_ok() {
    //     let mut contract = contract_dummy();
    //     let vendedor: AccountId = account(2);
    //     ink::env::test::set_caller::<ink::env::DefaultEnvironment>(vendedor);
    //     let producto = producto_dummy();
    //     // Simula publicar el producto
    //     contract.publicar_producto(producto.clone());
    //     // El id_publicacion será 1 porque es el primero
    //     let res = contract.verificar_owner_publicacion(1, vendedor);
    //     assert_eq!(res, Ok(()));
    // }
    // #[test]
    // fn verificar_owner_publicacion_publicacion_no_existe() {
    //     let mut contract = contract_dummy();
    //     let vendedor: AccountId = account(2);
    //     ink::env::test::set_caller::<ink::env::DefaultEnvironment>(vendedor);
    //     let res = contract.verificar_owner_publicacion(99, vendedor);
    //     assert_eq!(res, Err(ErrorMarketplace::PublicacionNoExiste));
    // }
    // #[test]
    // fn verificar_owner_publicacion_no_autorizado() {
    //     let mut contract = contract_dummy();
    //     let vendedor: AccountId = account(2);
    //     ink::env::test::set_caller::<ink::env::DefaultEnvironment>(vendedor);
    //     // Simula que otro usuario intenta verificar la publicación
    //     let otro: AccountId = account(20);
    //     let producto = producto_dummy();
    //     contract.publicar_producto(producto.clone());
    //     // El id_publicacion será 1, pero el vendedor es diferente
    //     let res = contract.verificar_owner_publicacion(1, otro);
    //     assert_eq!(res, Err(ErrorMarketplace::NoAutorizado));
    // }

    // #[test]
    // fn publicar_producto_ok() {
    //     let mut contract = contract_dummy();
    //     let vendedor = account(2);
    //     // Simula el caller como vendedor
    //     ink::env::test::set_caller::<ink::env::DefaultEnvironment>(vendedor);
    //     let producto = producto_dummy();
    //     let res = contract.publicar_producto(producto.clone());
    //     assert_eq!(res, Ok(()));
    //     // Verifica que la publicación fue guardada
    //     let publicacion = contract.obtener_publicacion(1);
    //     assert_eq!(publicacion.id_vendedor, vendedor);
    //     assert_eq!(publicacion.producto, producto);
    // }
    // #[test]
    // fn publicar_producto_falla_si_usuario_no_existe() {
    //     let mut contract = contract_dummy();
    //     let vendedor = account(2);
    //     ink::env::test::set_caller::<ink::env::DefaultEnvironment>(vendedor);

    //     let producto = producto_dummy();
    //     let res = contract.publicar_producto(producto);
    //     assert_eq!(res, Err(ErrorMarketplace::UsuarioNoExiste));
    // }

    // #[test]
    // fn publicar_producto_falla_si_no_es_vendedor() {
    //     let mut contract = contract_dummy();
    //     let comprador = account(1);
    //     ink::env::test::set_caller::<ink::env::DefaultEnvironment>(comprador);
    //     let producto = producto_dummy();
    //     // Intentar publicar un producto como comprador
    //     let res = contract.publicar_producto(producto);
    //     assert_eq!(res, Err(ErrorMarketplace::RolInvalido));
    // }
    // #[test]
    // fn publicar_producto_falla_si_producto_invalido() {
    //     let mut contract = contract_dummy();
    //     let vendedor = account(2);
    //     ink::env::test::set_caller::<ink::env::DefaultEnvironment>(vendedor);

    //     // Intentar publicar un producto con nombre vacío y precio inválido
    //     // Esto debería fallar por stock primero, luego por precio y nombre
    //     let producto = Producto::new(
    //         1,
    //         "".to_string(), // nombre vacío
    //         "Desc".to_string(),
    //         0, // precio inválido
    //         0, // stock inválido
    //         Categoria::Tecnologia,
    //     );
    //     let res = contract.publicar_producto(producto);
    //     assert_eq!(res, Err(ErrorMarketplace::StockInsuficiente)); // Falla por stock primero
    // }
 
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
