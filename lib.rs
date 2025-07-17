#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod market_place {
    use core::char::CharTryFromError;
    use std::vec;

    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;
    //use ink_e2e::sr25519::PublicKey;
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
        NoAutorizado,
        UsuarioYaRegistrado,
        UsuarioNoExiste,
        RolInvalido,
        ProductoNoExiste,
        StockInsuficiente,
        OrdenNoExiste,
        OrdenCancelada,
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
        NoAutorizado,
        PrecioInvalido,
        NombreInvalido,
        NoHayPublicaciones,
        DescripcionInvalida,
        IDProductoEnUso,
        IDPublicacionEnUso,
        MontoInsuficiente,
        StockDepositoInsuficiente,
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
                ErrorMarketplace::NoAutorizado => "No tiene autorización para realizar esta acción",
                ErrorMarketplace::NoHayPublicaciones => {
                    "No hay publicaciones disponibles de ese vendedor"
                }
                ErrorMarketplace::PublicacionNoExiste => "La publicación solicitada no existe",
                ErrorMarketplace::DescripcionInvalida => "La descripción no puede estar vacía",
                ErrorMarketplace::IDProductoEnUso => "El ID del producto ya está en uso",
                ErrorMarketplace::IDPublicacionEnUso => "El ID de la publicación ya está en uso",
                ErrorMarketplace::MontoInsuficiente => {
                    "El monto dado es insuficiente para cubrir el total de la orden"
                }
                ErrorMarketplace::StockDepositoInsuficiente => {
                    "El stock en deposito es menor al stock a vender"
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
    pub struct Producto {
        id_producto: u32,
        nombre: String,
        descripcion: String,
        categoria: Categoria,
    }
    impl Producto {
        pub fn new(
            id_producto: u32,
            nombre: String,
            descripcion: String,
            categoria: Categoria,
        ) -> Self {
            Self {
                id_producto,
                nombre,
                descripcion,
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
        id_producto: u32,
        precio: u128,
        stock_a_vender: u32,
    }

    impl Publicacion {
        fn new(
            id_publicacion: u32,
            id_vendedor: AccountId,
            id_producto: u32,
            precio: u128,
            stock_a_vender: u32,
        ) -> Self {
            Self {
                id_publicacion,
                id_vendedor,
                id_producto,
                precio,
                stock_a_vender,
            }
        }

        fn verificar_stock(&self, stock_pedido: u32) -> Result<(), ErrorMarketplace> {
            if stock_pedido > self.stock_a_vender {
                return Err(ErrorMarketplace::StockInsuficiente);
            }
            Ok(())
        }

        fn verificar_precio(&self, precio: u128) -> Result<(), ErrorMarketplace> {
            if precio <= 0 {
                return Err(ErrorMarketplace::PrecioInvalido);
            }
            Ok(())
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
        id: u32,
        comprador: AccountId,
        vendedor: AccountId,
        id_producto: u32,   //id del producto que se ordena
        cant_producto: u16, //cantidad de producto que se ordena
        estado: EstadoOrden,
        total: u128,
    }

    use scale_info::TypeInfo;
    #[derive(
        Debug,
        PartialEq,
        Eq,
        ink::scale::Encode,
        ink::scale::Decode,
        scale_info::TypeInfo,
        ink::storage::traits::StorageLayout,
    )]
    pub struct Deposito {
        id_producto: u32,
        id_vendedor: AccountId,
        stock: u32,
    }
    impl Deposito {
        pub fn new(id_producto: u32, id_vendedor: AccountId, stock: u32) -> Self {
            Self {
                id_producto,
                id_vendedor,
                stock,
            }
        }
        pub fn actualizar_stock(&mut self, stock: u32) -> Result<(), ErrorMarketplace> {
            self.stock = stock;
            Ok(())
        }
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct MarketPlace {
        usuarios: Mapping<AccountId, Usuario>, //id_usuario -> Usuario, todos los usuarios
        publicaciones: Mapping<u32, Publicacion>, //id_publicacion -> Publicacion, todas las publicaciones
        publicaciones_por_vendedor: Mapping<AccountId, Vec<u32>>, //vendedor -> [id_publicacion] busqueda rapida
        productos: Mapping<u32, Producto>,                        //id_producto -> Producto
        ordenes: Mapping<u32, Orden>,                             //id_orden -> Orden
        stock_general: Mapping<(AccountId, u32), Deposito>, // (id_vendedor, id_producto) -> Deposito
        //Atributos auxiliares
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
            id_producto: u32,
            cant_producto: u16,
            total: u128,
        ) -> Self {
            Self {
                id,
                comprador,
                vendedor,
                id_producto,
                cant_producto,
                total,
                estado: EstadoOrden::Pendiente,
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
                publicaciones_por_vendedor: Mapping::default(),
                stock_general: Mapping::default(),
                contador_ordenes: 0,
                contador_publicacion: 0,
                contador_productos: 0,
            }
        }

        pub fn registrar_producto(
            &mut self,
            nombre: String,
            descripcion: String,
            categoria: Categoria,
            stock: u32,
        ) -> Result<(), ErrorMarketplace> {
            let caller = self.env().caller();
            //verificar que el usuario sea vendedor
            self.verificar_rol_vendedor(caller)?;

            //validar producto
            self.validar_nombre_producto(&nombre)?;
            //validar descripcion
            self.validar_descripcion(&descripcion)?;

            //obtener nuevo id de producto
            let id_producto = self.obtener_nuevo_id_producto();

            //crear producto
            let nuevo_producto = Producto::new(
                id_producto,
                nombre.clone(),
                descripcion.clone(),
                categoria.clone(),
            );
            //insertar en el map de productos
            self.insertar_producto_en_catalogo(nuevo_producto.clone())?;

            //Inicializar deposito
            self.inicializar_deposito(caller, id_producto, stock)?;
            Ok(())
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

        //Helper verificar que el usuario tenga el rol correcto
        fn verificar_rol_comprador(&self, id: AccountId) -> Result<(), ErrorMarketplace> {
            let usuario = self.verificar_usuario_existe(id)?;
            if usuario.rol == Rol::Ambos || usuario.rol == Rol::Comprador {
                Ok(())
            } else {
                Err(ErrorMarketplace::RolInvalido)
            }
        }

        //Helper validar nombre de producto
        fn validar_nombre_producto(&self, nombre: &String) -> Result<(), ErrorMarketplace> {
            if nombre.is_empty() || nombre.trim().is_empty() {
                return Err(ErrorMarketplace::NombreInvalido);
            }
            Ok(())
        }

        fn validar_descripcion(&self, descripcion: &String) -> Result<(), ErrorMarketplace> {
            if descripcion.is_empty() || descripcion.trim().is_empty() {
                return Err(ErrorMarketplace::DescripcionInvalida);
            }
            Ok(())
        }
        //Helper validar stock de producto
        fn validar_stock_producto(&self, stock: &u32) -> Result<(), ErrorMarketplace> {
            if *stock == 0 {
                return Err(ErrorMarketplace::StockInsuficiente);
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

        //Helper para obtener un producto por id
        fn obtener_producto(&self, id_producto: u32) -> Result<Producto, ErrorMarketplace> {
            self.productos
                .get(&id_producto)
                .ok_or_else(|| ErrorMarketplace::ProductoNoExiste)
        }

        //Helper para obtener un nuevo id de publicacion
        fn obtener_nuevo_id_publicacion(&mut self) -> u32 {
            self.contador_publicacion += 1;
            self.contador_publicacion
        }
        //Helper para obtener nuevo id de producto
        fn obtener_nuevo_id_producto(&mut self) -> u32 {
            self.contador_productos += 1;
            self.contador_productos
        }
        //Helper para verificar que id no este en uso
        fn verificar_id_producto_en_uso(&self, id_producto: u32) -> Result<(), ErrorMarketplace> {
            if self.productos.contains(&id_producto) {
                return Err(ErrorMarketplace::IDProductoEnUso);
            }
            Ok(())
        }
        //Helper para verificar que id de publicacion no este en uso
        fn verificar_id_publicacion_en_uso(
            &self,
            id_publicacion: u32,
        ) -> Result<(), ErrorMarketplace> {
            if self.publicaciones.contains(&id_publicacion) {
                return Err(ErrorMarketplace::IDPublicacionEnUso);
            }
            Ok(())
        }
        //Helper para verificar que id de publicacion no este en uso por un vendedor
        fn verificar_id_publicaciones_por_vendedor_en_uso(
            &self,
            id_vendedor: AccountId,
            id_publicacion: u32,
        ) -> Result<(), ErrorMarketplace> {
            let publicaciones = self.publicaciones_por_vendedor.get(&id_vendedor);
            if let Some(publicaciones) = publicaciones {
                if publicaciones.contains(&id_publicacion) {
                    return Err(ErrorMarketplace::IDPublicacionEnUso);
                }
            }
            Ok(())
        }
        //Helper para insertar producto en el catalogo de productos
        fn insertar_producto_en_catalogo(
            &mut self,
            producto: Producto,
        ) -> Result<(), ErrorMarketplace> {
            // Verificar que el producto no exista
            self.verificar_id_producto_en_uso(producto.id_producto)?;
            // Insertar el producto en el mapping
            self.productos.insert(producto.id_producto, &producto);
            Ok(())
        }

        /// Helper inserta una publicación en el mapping de publicaciones por vendedor.
        fn insertar_publicacion_por_vendedor(
            &mut self,
            id_vendedor: AccountId,
            id_publicacion: u32,
        ) {
            // Obtiene el vector existente o crea uno nuevo si no existe
            let mut publicaciones = self
                .publicaciones_por_vendedor
                .get(&id_vendedor)
                .unwrap_or_else(|| Vec::new());

            publicaciones.push(id_publicacion);

            // Inserta el vector actualizado de vuelta en el mapping
            self.publicaciones_por_vendedor
                .insert(id_vendedor, &publicaciones);
        }

        /// Helper para insertar una publicación en el sistema.
        fn insertar_publicacion(
            &mut self,
            publicacion: Publicacion,
        ) -> Result<(), ErrorMarketplace> {
            // Verificar que la publicación no exista
            self.verificar_id_publicacion_en_uso(publicacion.id_publicacion)?;
            // Insertar la publicación en el mapping
            self.publicaciones
                .insert(publicacion.id_publicacion, &publicacion);
            Ok(())
        }

        ///Helper que devuelve el stock total de un producto en el depósito de un vendedor.
        fn obtener_stock_deposito(
            &self,
            id_vendedor: AccountId,
            id_producto: u32,
        ) -> Result<u32, ErrorMarketplace> {
            self.stock_general
                .get(&(id_vendedor, id_producto))
                .map_or(Err(ErrorMarketplace::ProductoNoExiste), |deposito| {
                    Ok(deposito.stock)
                })
        }

        ///Helper para verificar que tengo suficiente stock en el depósito del vendedor.
        fn validar_stock_deposito(
            &self,
            id_vendedor: AccountId,
            id_producto: u32,
            stock_a_vender: u32,
        ) -> Result<(), ErrorMarketplace> {
            let stock_actual = self.obtener_stock_deposito(id_vendedor, id_producto)?;
            if stock_actual < stock_a_vender {
                return Err(ErrorMarketplace::StockDepositoInsuficiente);
            }
            Ok(())
        }
        /// Helper para validar el precio de un producto.
        fn validar_precio_publicacion(&self, precio: &u128) -> Result<(), ErrorMarketplace> {
            if *precio <= 0 {
                return Err(ErrorMarketplace::PrecioInvalido);
            }
            Ok(())
        }

        /// Helper para actualizar el stock de un producto en el depósito de un vendedor.
        fn actualizar_stock_producto(
            &mut self,
            id_vendedor: AccountId,
            id_producto: u32,
            stock_a_vender: u32,
        ) -> Result<(), ErrorMarketplace> {
            // Obtener el stock actual del depósito
            let stock_actual = self.obtener_stock_deposito(id_vendedor, id_producto)?;
            // Verificar que el stock a vender no exceda el stock actual
            self.validar_stock_deposito(id_vendedor, id_producto, stock_a_vender)?;
            // Actualizar el stock en el depósito
            let nuevo_stock: u32 = stock_actual - stock_a_vender;
            let mut deposito = self
                .stock_general
                .get(&(id_vendedor, id_producto))
                .ok_or(ErrorMarketplace::ProductoNoExiste)?;
            deposito.actualizar_stock(nuevo_stock)?;
            Ok(())
        }
        ///Funcion que modifica el stock de un depósito de un vendedor.
        #[ink(message)]
        pub fn modificar_stock_deposito(
            &mut self,
            id_vendedor: AccountId,
            id_producto: u32,
            stock: u32,
        ) -> Result<(), ErrorMarketplace> {
            // Verificar que el usuario sea vendedor
            self.verificar_rol_vendedor(id_vendedor)?;
            // Verificar que el stock sea válido
            self.validar_stock_producto(&stock)?; //Preguntar al profesor u32 no tiene signo asi que esta validacion es inutil

            // Obtener el depósito del vendedor
            let mut deposito = self
                .stock_general
                .get(&(id_vendedor, id_producto))
                .ok_or(ErrorMarketplace::ProductoNoExiste)?;
            // Actualizar el stock del depósito
            deposito.actualizar_stock(stock)?;
            // Guardar el depósito actualizado en el mapping
            self.stock_general
                .insert((id_vendedor, id_producto), &deposito);
            Ok(())
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
        pub fn inicializar_deposito(
            &mut self,
            id_vendedor: AccountId,
            id_producto: u32,
            stock: u32,
        ) -> Result<(), ErrorMarketplace> {
            // Verificar que el usuario sea vendedor
            self.verificar_rol_vendedor(id_vendedor)?;
            // Verificar que el stock sea válido
            self.validar_stock_producto(&stock)?; // Preguntar al profesor u32 no tiene signo así que esta validación es inútil
                                                  // Crear un nuevo depósito
            let deposito = Deposito::new(id_producto, id_vendedor, stock);

            // Insertar el depósito en el mapping
            self.stock_general
                .insert((id_vendedor, id_producto), &deposito);
            Ok(())
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
        pub fn crear_publicacion(
            &mut self,
            id_producto: u32,
            stock_a_vender: u32,
            precio: u128,
        ) -> Result<(), ErrorMarketplace> {
            let caller = self.env().caller();
            self.verificar_usuario_existe(caller)?;
            self.verificar_rol_vendedor(caller)?;
            //Validar precio
            self.validar_precio_publicacion(&precio)?;
            //Validar stock para vender es menor o igual al stock total del deposito
            self.validar_stock_deposito(caller, id_producto, stock_a_vender)?;

            // Generamos un nuevo ID para la publicación
            let id_publicacion = self.obtener_nuevo_id_publicacion();

            // Creamos una nueva publicación
            let nueva_publicacion = Publicacion::new(
                id_publicacion,
                caller, // id del vendedor
                id_producto,
                precio,
                stock_a_vender,
            );

            //Guardamos la publicación en el mapping de publicaciones
            self.insertar_publicacion(nueva_publicacion.clone())?;

            //Guardamos la publicación en el mapping de publicaciones por vendedor
            self.insertar_publicacion_por_vendedor(caller, id_publicacion);

            // Actualizamos el stock del producto del vendedor
            self.actualizar_stock_producto(caller, id_producto, stock_a_vender)?;
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
            monto_dado: u128,
        ) -> Result<(), ErrorMarketplace> {
            //monto dado es el monto que el comprador me da para pagar la orden
            let caller = self.env().caller();

            // Verificar que el usuario exista
            self.verificar_usuario_existe(caller)?;

            // Verificar que el usuario sea comprador
            self.verificar_rol_comprador(caller)?;

            // Verificar que la publicación exista
            let publicacion = self.obtener_publicacion(id_publicacion)?;

            // Verificar que el stock sea suficiente y asi poder crear la orden
            publicacion.verificar_stock(cant_producto as u32)?;

            let tot_orden = publicacion.precio * cant_producto as u128;

            // Verificar que el monto dado sea suficiente para cubrir el total de la orden
            if monto_dado <= 0 || monto_dado < tot_orden {
                return Err(ErrorMarketplace::MontoInsuficiente);
            }

            // Crear nueva orden
            let nueva_id = self.contador_ordenes;
            let orden = Orden::new(
                nueva_id,
                caller,
                publicacion.id_vendedor,
                publicacion.id_producto,
                cant_producto,
                tot_orden,
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
                if caller != orden.vendedor {
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
                Err(ErrorOrden::OrdenNoExiste)
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
    }
    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        use ink::{env::test, primitives::AccountId};

        use crate::market_place::{
            Categoria, ErrorMarketplace, ErrorOrden, EstadoOrden, MarketPlace, Orden, Producto,
            Rol, Usuario,
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
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account(1));
            contract
                .registrar_usuario("user1".to_string(), Rol::Comprador)
                .ok();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account(2));
            contract
                .registrar_usuario("user2".to_string(), Rol::Vendedor)
                .ok();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account(3));
            contract
                .registrar_usuario("user3".to_string(), Rol::Ambos)
                .ok();
            contract
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
        fn test_marcar_recibida_estado_invalido() {
            let mut orden = Orden::new(1, account(1), account(2), vec![], 100);
            let res = orden.marcar_recibida(account(1));
            assert_eq!(res, Err(ErrorOrden::EstadoInvalido));
        }

        /// Test MarketPlace
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
        #[ink::test]
        fn registro_producto_ok() {
            let mut contract = contract_dummy();
            let nombre = String::from("Producto válido");
            let descripcion = String::from("Descripción del producto");
            let categoria: Categoria = Categoria::Tecnologia;
            let stock: u32 = 10;
            //Usamos el vendedor account(2) para registrar el producto
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account(2));
            let res = contract.registrar_producto(
                nombre.clone(),
                descripcion.clone(),
                categoria.clone(),
                stock,
            );
            assert_eq!(res, Ok(()));
        }
        /// Tests Marketplace Helpers
        #[ink::test]
        fn validacion_producto_nombre_espacios() {
            let mut contract = MarketPlace::new();
            let nombre = String::from("   ");
            let res = contract.validar_nombre_producto(&nombre);
            assert_eq!(res, Err(ErrorMarketplace::NombreInvalido));
        }
        #[ink::test]
        fn validar_descripcion_error_descripcion_invalida() {
            let contract = MarketPlace::new();
            let descripcion_vacia = String::from("");
            let descripcion_espacios = String::from("   ");

            let res_vacia = contract.validar_descripcion(&descripcion_vacia);
            let res_espacios = contract.validar_descripcion(&descripcion_espacios);

            assert_eq!(res_vacia, Err(ErrorMarketplace::DescripcionInvalida));
            assert_eq!(res_espacios, Err(ErrorMarketplace::DescripcionInvalida));
        }
        #[ink::test]
        fn obtener_publicacion_no_existe() {
            let contract = MarketPlace::new();
            let res = contract.obtener_publicacion(42);
            assert_eq!(res, Err(ErrorMarketplace::PublicacionNoExiste));
        }

        #[ink::test]
        fn verificar_rol_vendedor_ok() {
            let contract = contract_dummy();
            let res = contract.verificar_rol_vendedor(account(2));
            assert_eq!(res, Ok(()));
        }
        #[ink::test]
        fn verificar_rol_vendedor_falla_si_no_es_vendedor() {
            let contract = contract_dummy();
            let res = contract.verificar_rol_vendedor(account(1));
            assert_eq!(res, Err(ErrorMarketplace::RolInvalido));
        }
        #[ink::test]
        fn verificar_rol_comprador_ok() {
            let mut contract = MarketPlace::new();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account(1));
            let _ = contract.registrar_usuario("comprador".to_string(), Rol::Comprador);
            let res = contract.verificar_rol_comprador(account(1));
            assert_eq!(res, Ok(()));
        }
        #[ink::test]
        fn verificar_rol_comprador_error_rol_invalido() {
            let mut contract = MarketPlace::new();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account(2));
            let _ = contract.registrar_usuario("vendedor".to_string(), Rol::Vendedor);
            let res = contract.verificar_rol_comprador(account(2));
            assert_eq!(res, Err(ErrorMarketplace::RolInvalido));
        }
        #[ink::test]
        fn verificar_usuario_existe_ok() {
            let mut contract = contract_dummy();
            let usuario = Usuario::new("test".to_string(), Rol::Comprador, account(4));
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account(4));
            let registrar_ok =
                contract.registrar_usuario(usuario.username.clone(), usuario.rol.clone());
            assert_eq!(registrar_ok, Ok(()));
            let res = contract.verificar_usuario_existe(account(4));
            assert_eq!(res, Ok(usuario));
        }

        #[ink::test]
        fn verificar_usuario_existe_falla_si_no_existe() {
            let contract = MarketPlace::new();
            let res = contract.verificar_usuario_existe(account(4));
            assert_eq!(res, Err(ErrorMarketplace::UsuarioNoExiste));
        }
        #[ink::test]
        fn validar_stock_producto_error_stock_insuficiente() {
            let contract = MarketPlace::new();
            let stock = 0u32;
            let res = contract.validar_stock_producto(&stock);
            assert_eq!(res, Err(ErrorMarketplace::StockInsuficiente));
        }
        #[ink::test]
        fn obtener_nuevo_id_publicacion_ok() {
            let mut contract = MarketPlace::new();
            assert_eq!(contract.contador_publicacion, 0);

            let id1 = contract.obtener_nuevo_id_publicacion();
            assert_eq!(id1, 1);
            assert_eq!(contract.contador_publicacion, 1);

            let id2 = contract.obtener_nuevo_id_publicacion();
            assert_eq!(id2, 2);
            assert_eq!(contract.contador_publicacion, 2);
        }
        #[ink::test]
        fn obtener_producto_ok() {
            let mut contract = MarketPlace::new();
            // Creamos y agregamos un producto
            let producto = Producto::new(
                1,
                "Producto".to_string(),
                "Desc".to_string(),
                Categoria::Tecnologia,
            );
            contract.productos.insert(1, &producto);

            let res = contract.obtener_producto(1);
            assert_eq!(res, Ok(producto));
        }

        #[ink::test]
        fn obtener_producto_error_producto_no_existe() {
            let contract = MarketPlace::new();
            let res = contract.obtener_producto(42);
            assert_eq!(res, Err(ErrorMarketplace::ProductoNoExiste));
        }
        #[ink::test]
        fn verificar_id_producto_en_uso_error_id_producto_en_uso() {
            let mut contract = MarketPlace::new();
            // Insertamos un producto con id 1
            let producto = Producto::new(
                1,
                "Producto".to_string(),
                "Desc".to_string(),
                Categoria::Tecnologia,
            );
            contract.productos.insert(1, &producto);

            // Ahora verificamos que el helper detecta el ID en uso
            let res = contract.verificar_id_producto_en_uso(1);
            assert_eq!(res, Err(ErrorMarketplace::IDProductoEnUso));
        }
        #[ink::test]
        fn verificar_id_publicacion_en_uso_ok() {
            let contract = MarketPlace::new();
            // No hay publicaciones, el id no está en uso
            let res = contract.verificar_id_publicacion_en_uso(1);
            assert_eq!(res, Ok(()));
        }

        #[ink::test]
        fn verificar_id_publicacion_en_uso_error_id_publicacion_en_uso() {
            let mut contract = MarketPlace::new();
            // Insertamos una publicación con id 1
            let publicacion = Publicacion::new(1, account(2), 1, 100, 10);
            contract.publicaciones.insert(1, &publicacion);

            // Ahora verificamos que el helper detecta el ID en uso
            let res = contract.verificar_id_publicacion_en_uso(1);
            assert_eq!(res, Err(ErrorMarketplace::IDPublicacionEnUso));
        }
        #[ink::test]
        fn verificar_id_publicaciones_por_vendedor_en_uso_ok() {
            let contract = MarketPlace::new();
            // El vendedor no tiene publicaciones, el id no está en uso
            let res = contract.verificar_id_publicaciones_por_vendedor_en_uso(account(2), 1);
            assert_eq!(res, Ok(()));
        }

        #[ink::test]
        fn verificar_id_publicaciones_por_vendedor_en_uso_error_id_publicacion_en_uso() {
            let mut contract = MarketPlace::new();
            // Simulamos que el vendedor tiene la publicación con id 1
            contract
                .publicaciones_por_vendedor
                .insert(account(2), &vec![1, 2, 3]);

            let res = contract.verificar_id_publicaciones_por_vendedor_en_uso(account(2), 1);
            assert_eq!(res, Err(ErrorMarketplace::IDPublicacionEnUso));
        }

        /// Tests de Impl Publicacion
        #[ink::test]
        fn publicacion_new_ok() {
            let id_publicacion = 1;
            let id_vendedor = account(2);
            let id_producto = 10;
            let precio = 500;
            let stock_a_vender = 20;

            let publicacion = Publicacion::new(
                id_publicacion,
                id_vendedor,
                id_producto,
                precio,
                stock_a_vender,
            );

            assert_eq!(publicacion.id_publicacion, id_publicacion);
            assert_eq!(publicacion.id_vendedor, id_vendedor);
            assert_eq!(publicacion.id_producto, id_producto);
            assert_eq!(publicacion.precio, precio);
            assert_eq!(publicacion.stock_a_vender, stock_a_vender);
        }
        #[ink::test]
        fn verificar_precio_ok() {
            let publicacion = Publicacion::new(1, account(2), 1, 100, 10);
            let res = publicacion.verificar_precio(100);
            assert_eq!(res, Ok(()));
        }

        #[ink::test]
        fn verificar_precio_error_precio_invalido() {
            let publicacion = Publicacion::new(1, account(2), 1, 100, 10);
            let res = publicacion.verificar_precio(0);
            assert_eq!(res, Err(ErrorMarketplace::PrecioInvalido));
        }
        #[ink::test]
        fn verificar_stock_ok() {
            let publicacion = Publicacion::new(1, account(2), 1, 100, 10);
            let res = publicacion.verificar_stock(5);
            assert_eq!(res, Ok(()));
        }

        #[ink::test]
        fn verificar_stock_error_stock_insuficiente() {
            let publicacion = Publicacion::new(1, account(2), 1, 100, 10);
            let res = publicacion.verificar_stock(20);
            assert_eq!(res, Err(ErrorMarketplace::StockInsuficiente));
        }

        ///Test Deposito
        #[ink::test]
        fn actualizar_stock_ok() {
            let mut deposito = Deposito::new(1, account(2), 10);
            let res = deposito.actualizar_stock(20);
            assert_eq!(res, Ok(()));
            assert_eq!(deposito.stock, 20);
        }
        #[ink::test]
        fn orden_new_ok() {
            let id = 1;
            let comprador = account(4);
            let vendedor = account(2);
            let id_producto = 10;
            let cant_producto = 3;
            let total = 1500;

            let orden = Orden::new(id, comprador, vendedor, id_producto, cant_producto, total);

            assert_eq!(orden.id, id);
            assert_eq!(orden.comprador, comprador);
            assert_eq!(orden.vendedor, vendedor);
            assert_eq!(orden.id_producto, id_producto);
            assert_eq!(orden.cant_producto, cant_producto);
            assert_eq!(orden.total, total);
            assert_eq!(orden.estado, EstadoOrden::Pendiente);
        }
        #[ink::test]
        fn insertar_publicacion_ok() {
            let mut contract = MarketPlace::new();
            let publicacion = Publicacion::new(1, account(2), 1, 100, 10);

            let res = contract.insertar_publicacion(publicacion.clone());
            assert_eq!(res, Ok(()));
            // Verifica que la publicación fue insertada
            let guardada = contract.publicaciones.get(&1);
            assert_eq!(guardada, Some(publicacion));
        }

        #[ink::test]
        fn insertar_publicacion_error_id_publicacion_en_uso() {
            let mut contract = MarketPlace::new();
            let publicacion = Publicacion::new(1, account(2), 1, 100, 10);
            // Insertamos primero la publicación
            let _ = contract.insertar_publicacion(publicacion.clone());
            // Intentamos insertar otra con el mismo id
            let otra_publicacion = Publicacion::new(1, account(3), 2, 200, 5);

            let res = contract.insertar_publicacion(otra_publicacion);
            assert_eq!(res, Err(ErrorMarketplace::IDPublicacionEnUso));
        }
        #[ink::test]
        fn obtener_stock_deposito_ok() {
            let mut contract = MarketPlace::new();
            // Insertamos un depósito para el vendedor y producto
            let deposito = Deposito::new(1, account(2), 15);
            contract.stock_general.insert((account(2), 1), &deposito);

            let res = contract.obtener_stock_deposito(account(2), 1);
            assert_eq!(res, Ok(15));
        }

        #[ink::test]
        fn obtener_stock_deposito_error_producto_no_existe() {
            let contract = MarketPlace::new();
            // No existe el depósito para ese vendedor y producto
            let res = contract.obtener_stock_deposito(account(2), 1);
            assert_eq!(res, Err(ErrorMarketplace::ProductoNoExiste));
        }

        #[ink::test]
        fn modificar_stock_deposito_ok() {
            let mut contract = contract_dummy();
            // Registrar el usuario como vendedor
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account(2));
            // Registramos producto y se inicializa el deposito
            let _ = contract.registrar_producto(
                "Producto".to_string(),
                "Descripcion".to_string(),
                Categoria::Tecnologia,
                10,
            );
            // Obtener ID del producto
            let id_producto = 1; // Asumimos que el producto tiene ID 1

            // Modificar el stock
            let res = contract.modificar_stock_deposito(account(2), id_producto, 60);
            assert_eq!(res, Ok(()));
        }

        #[ink::test]
        fn modificar_stock_deposito_error_producto_no_existe() {
            let mut contract = contract_dummy();
            // Registrar el usuario como vendedor
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account(2));
            let _ = contract.registrar_usuario("vendedor".to_string(), Rol::Vendedor);

            // No existe el depósito para ese vendedor y producto
            let res = contract.modificar_stock_deposito(account(2), 1, 25);
            assert_eq!(res, Err(ErrorMarketplace::ProductoNoExiste));
        }
        #[ink::test]
        fn validar_stock_deposito_ok() {
            let mut contract = MarketPlace::new();
            // Insertamos un depósito con stock suficiente
            let deposito = Deposito::new(1, account(2), 20);
            contract.stock_general.insert((account(2), 1), &deposito);

            let res = contract.validar_stock_deposito(account(2), 1, 10);
            assert_eq!(res, Ok(()));
        }

        #[ink::test]
        fn validar_stock_deposito_error_stock_deposito_insuficiente() {
            let mut contract = MarketPlace::new();
            // Insertamos un depósito con stock insuficiente
            let deposito = Deposito::new(1, account(2), 5);
            contract.stock_general.insert((account(2), 1), &deposito);

            let res = contract.validar_stock_deposito(account(2), 1, 10);
            assert_eq!(res, Err(ErrorMarketplace::StockDepositoInsuficiente));
        }
        #[ink::test]
        fn validar_precio_publicacion_ok() {
            let contract = MarketPlace::new();
            let precio = 100u128;
            let res = contract.validar_precio_publicacion(&precio);
            assert_eq!(res, Ok(()));
        }

        #[ink::test]
        fn validar_precio_publicacion_error_precio_invalido() {
            let contract = MarketPlace::new();
            let precio = 0u128;
            let res = contract.validar_precio_publicacion(&precio);
            assert_eq!(res, Err(ErrorMarketplace::PrecioInvalido));
        }
        #[ink::test]
        fn mostrar_publicaciones_propias_ok() {
            let mut contract = contract_dummy();
            // Creamos y agregamos una publicación
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account(2));
            // Registramos producto y se inicializa el deposito
            let _ = contract.registrar_producto(
                "Producto".to_string(),
                "Descripcion".to_string(),
                Categoria::Tecnologia,
                10,
            );
            let id_producto = 1; // Asumimos que el producto tiene ID 1
            let res_pub = contract.crear_publicacion(id_producto, 5, 100);
            assert_eq!(res_pub, Ok(()));
            // Obtenemos la publicación creada

            let res = contract.mostrar_publicaciones_propias(account(2));
            //verificamos que recibimos un vector con una Publicacion de ID 1
            let publicacion_esperada = Publicacion::new(1, account(2), id_producto, 100, 5);
            assert_eq!(res, Ok(vec![publicacion_esperada]));
        }

        #[ink::test]
        fn mostrar_publicaciones_propias_error_no_hay_publicaciones() {
            let contract = MarketPlace::new();
            let vendedor = account(2);

            let res = contract.mostrar_publicaciones_propias(vendedor);
            assert_eq!(res, Err(ErrorMarketplace::NoHayPublicaciones));
        }
        #[ink::test]
        fn obtener_publicaciones_por_vendedor_ok() {
            let mut contract = contract_dummy();
            let vendedor = account(2);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account(2));
            // Registramos producto y se inicializa el deposito
            let _ = contract.registrar_producto(
                "Producto".to_string(),
                "Descripcion".to_string(),
                Categoria::Tecnologia,
                10,
            );

            // Asumimos que el producto tiene ID 1
            let id_producto = 1;
            // Creamos y agregamos una publicación
            let res_pub = contract.crear_publicacion(id_producto, 5, 100);
            assert_eq!(res_pub, Ok(()));
            let res = contract.obtener_publicaciones_por_vendedor(vendedor);
            // Verificamos que recibimos un vector con una Publicacion de ID 1
            let publicacion_esperada = Publicacion::new(1, vendedor, id_producto, 100, 5);
            assert_eq!(res, Ok(vec![publicacion_esperada]));
        }

        #[ink::test]
        fn obtener_publicaciones_por_vendedor_error_no_hay_publicaciones() {
            let contract = MarketPlace::new();
            let vendedor = account(2);

            let res = contract.obtener_publicaciones_por_vendedor(vendedor);
            assert_eq!(res, Err(ErrorMarketplace::NoHayPublicaciones));
        }
        #[ink::test]
        fn crear_orden_ok() {
            let mut contract = contract_dummy();
            // Registrar producto y depósito
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account(2));
            let _ = contract.registrar_producto(
                "Producto".to_string(),
                "Descripcion".to_string(),
                Categoria::Tecnologia,
                10,
            );
            let id_producto = 1;

            // Crear la publicación
            let res = contract.crear_publicacion(id_producto, 5, 100);
            assert_eq!(res, Ok(()));

            // Verificamos que la publicación fue creada correctamente
            let publicaciones = contract.mostrar_publicaciones_propias(account(2));
            let publicacion_esperada = Publicacion::new(1, account(2), id_producto, 100, 5);
            assert_eq!(publicaciones, Ok(vec![publicacion_esperada]));
        }
    }
    //Preguntar, debemos listar en una pub fn todas las publicaciones del contrato?
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
}
*/
