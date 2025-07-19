#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod market_place {
    // use core::char::CharTryFromError;
    use ink::prelude::string::String;
    use ink::prelude::string::ToString;
    // use ink::prelude::vec::Vec;
    use ink::storage::Mapping;
    //use ink_e2e::sr25519::PublicKey;
    //use ink_e2e::subxt_signer::bip39::serde::de::value::Error;

    /// Representa los roles posibles que puede tener un usuario dentro del marketplace.
    ///
    /// # Variantes
    /// - `Comprador`: Solo puede comprar productos
    /// - `Vendedor`: Solo puede vender productos
    /// - `Ambos`: Puede tanto comprar como vender
    #[derive(Debug, Clone, Copy, PartialEq, Eq, ink::scale::Encode, ink::scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum Rol {
        Comprador,
        Vendedor,
        Ambos,
    }

    /// Categorías disponibles para clasificar productos en el marketplace.
    ///
    /// # Variantes
    /// - `Tecnologia`: Dispositivos electrónicos, software, etc.
    /// - `Indumentaria`: Ropa, accesorios, calzado
    /// - `Hogar`: Muebles, decoración, electrodomésticos
    /// - `Alimentos`: Comida, bebidas, productos alimenticios
    /// - `Otros`: Cualquier producto que no encaje en las categorías anteriores
    #[derive(Debug, Clone, PartialEq, Eq, ink::scale::Encode, ink::scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum Categoria {
        Tecnologia,
        Indumentaria,
        Hogar,
        Alimentos,
        Otros,
    }

    /// Estados posibles que puede tener una orden de compra a lo largo de su ciclo de vida.
    ///
    /// # Variantes
    /// - `Pendiente`: Orden creada, esperando procesamiento del vendedor
    /// - `Enviado`: Vendedor ha enviado el producto
    /// - `Recibido`: Comprador ha recibido y confirmado el producto
    /// - `Cancelada`: Orden cancelada por alguna de las partes
    #[derive(Debug, Clone, PartialEq, Eq, ink::scale::Encode, ink::scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum EstadoOrden {
        Pendiente,
        Enviado,
        Recibido,
        Cancelada,
    }

    /// ## Errores del Marketplace
    ///
    /// Define todos los posibles errores que pueden ocurrir durante las operaciones del marketplace.
    /// Cada error incluye una descripción específica del problema encontrado.

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
        OrdenCancelada,
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
        ProductoYaPoseeDeposito,
        Overflow, // Error para manejar overflow en cálculos aritméticos
        DepositoNoEncontrado,
    }
    // Structs

    /// Representa a un usuario registrado en el marketplace.
    ///
    /// # Campos
    /// - `username`: Nombre de usuario único
    /// - `rol`: Rol del usuario (Comprador, Vendedor, Ambos)
    /// - `id`: Identificador único de la cuenta (AccountId)
    /// - `verificacion`: Estado de verificación del usuario
    ///
    #[derive(Debug, Clone, PartialEq, Eq, ink::scale::Encode, ink::scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct Usuario {
        username: String,
        rol: Rol,
        id: AccountId,
        verificacion: bool,
    }
    impl Usuario {
        /// Crea un nuevo usuario verificado.
        ///
        /// # Parámetros
        /// - `username`: Nombre público del usuario.
        /// - `rol`: Rol asignado (Comprador, Vendedor, Ambos).
        /// - `id`: Cuenta (AccountId) asociada al usuario.
        ///
        /// # Retorna
        /// Instancia de `Usuario` con verificación activa.
        pub fn new(username: String, rol: Rol, id: AccountId) -> Self {
            Self {
                username,
                rol,
                id,
                verificacion: true,
            }
        }

        // Helper validar rol vendedor
        /// Valida que el rol del usuario permita actuar como vendedor.
        ///
        /// # Parámetros
        /// - `rol`: Referencia al rol a validar.
        ///
        /// # Retorna
        /// - `Ok(())` si el rol es Vendedor o Ambos.
        /// - `Err(ErrorMarketplace::RolInvalido)` en caso contrario.
        fn validar_rol_vendedor(rol: &Rol) -> Result<(), ErrorMarketplace> {
            if *rol == Rol::Vendedor || *rol == Rol::Ambos {
                Ok(())
            } else {
                Err(ErrorMarketplace::RolInvalido)
            }
        }

        // Helper validar rol comprador
        /// Valida que el rol del usuario permita actuar como comprador.
        ///
        /// # Parámetros
        /// - `rol`: Referencia al rol a validar.
        ///
        /// # Retorna
        /// - `Ok(())` si el rol es Comprador o Ambos.
        /// - `Err(ErrorMarketplace::RolInvalido)` en caso contrario.
        fn validar_rol_comprador(rol: &Rol) -> Result<(), ErrorMarketplace> {
            if *rol == Rol::Comprador || *rol == Rol::Ambos {
                Ok(())
            } else {
                Err(ErrorMarketplace::RolInvalido)
            }
        }
    }

    /// Representa un producto en el marketplace.
    ///
    /// # Campos
    /// - `id`: Identificador único del producto
    /// - `nombre`: Nombre del producto
    /// - `descripcion`: Descripción detallada del producto
    /// - `precio`: Precio del producto en la moneda nativa
    /// - `categoria`: Categoría a la que pertenece el producto
    ///
    #[derive(Debug, Clone, PartialEq, Eq, ink::scale::Encode, ink::scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct Producto {
        id_producto: u32,
        nombre: String,
        descripcion: String,
        categoria: Categoria,
    }
    impl Producto {
        /// Crea una nueva instancia de un producto.
        ///
        /// # Parámetros
        /// - `id_producto`: ID único del producto.
        /// - `nombre`: Nombre del producto.
        /// - `descripcion`: Detalle descriptivo del producto.
        /// - `categoria`: Categoría a la que pertenece.
        ///
        /// # Retorna
        /// Una nueva instancia de `Producto`.
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

        //Helper validar nombre de producto
        /// Verifica si el nombre del producto es válido.
        ///
        /// # Parámetros
        /// - `nombre`: Nombre a validar.
        ///
        /// # Retorna
        /// - `Ok(())` si el nombre es válido.
        /// - `Err(ErrorMarketplace::NombreInvalido)` si está vacío o sólo contiene espacios.
        fn validar_nombre_producto(nombre: &String) -> Result<(), ErrorMarketplace> {
            if nombre.is_empty() || nombre.trim().is_empty() {
                return Err(ErrorMarketplace::NombreInvalido);
            }
            Ok(())
        }

        // Helper normalizar nombre de producto
        /// Normaliza el nombre del producto.
        ///
        /// Convierte a minúsculas y elimina espacios sobrantes.
        ///
        /// # Parámetros
        /// - `nombre`: Nombre a normalizar.
        ///
        /// # Retorna
        /// Una nueva cadena con el nombre formateado.

        fn normalizar_nombre_producto(nombre: &String) -> String {
            // Normaliza el nombre del producto a minúsculas y elimina espacios extra
            nombre.to_lowercase().trim().to_string()
        }

        // Helper validar descripcion de producto
        /// Valida si la descripción del producto es adecuada.
        ///
        /// # Parámetros
        /// - `descripcion`: Descripción a validar.
        ///
        /// # Retorna
        /// - `Ok(())` si la descripción es válida.
        /// - `Err(ErrorMarketplace::DescripcionInvalida)` si está vacía o sólo tiene espacios.
        fn validar_descripcion(descripcion: &String) -> Result<(), ErrorMarketplace> {
            if descripcion.is_empty() || descripcion.trim().is_empty() {
                return Err(ErrorMarketplace::DescripcionInvalida);
            }
            Ok(())
        }
        //Helper validar stock de producto
        /// Valida si el stock de un producto es mayor a cero.
        ///
        /// # Parámetros
        /// - `stock`: Cantidad de stock a validar.
        ///
        /// # Retorna
        /// - `Ok(())` si el stock es mayor a cero.
        /// - `Err(ErrorMarketplace::StockInsuficiente)` si es igual a cero.
        fn validar_stock_producto(stock: &u32) -> Result<(), ErrorMarketplace> {
            if *stock == 0 {
                return Err(ErrorMarketplace::StockInsuficiente);
            }
            Ok(())
        }
    }

    /// Representa una publicación de producto en el marketplace.
    ///
    /// # Campos
    /// - `id`: Identificador único de la publicación
    /// - `producto_id`: ID del producto publicado
    /// - `vendedor`: AccountId del vendedor
    /// - `stock_publicacion`: Cantidad disponible en esta publicación
    /// - `precio`: Precio del producto en la moneda nativa
    #[derive(Debug, Clone, PartialEq, Eq, ink::scale::Encode, ink::scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct Publicacion {
        id_publicacion: u32,
        id_vendedor: AccountId,
        id_producto: u32,
        precio: u128,
        stock_a_vender: u32,
    }

    /// Crea una nueva instancia de una publicación.
    ///
    /// # Parámetros
    /// - `id_publicacion`: Identificador único para la publicación.
    /// - `id_vendedor`: Cuenta del vendedor que publica.
    /// - `id_producto`: Producto asociado a esta publicación.
    /// - `precio`: Precio por unidad del producto.
    /// - `stock_a_vender`: Cantidad disponible para la venta.
    ///
    /// # Retorna
    /// Una nueva instancia de `Publicacion`.
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

        /// Verifica si hay stock suficiente disponible para una orden.
        ///
        /// # Parámetros
        /// - `stock_pedido`: Cantidad que se desea comprar.
        ///
        /// # Retorna
        /// - `Ok(())` si hay suficiente stock disponible.
        /// - `Err(ErrorMarketplace::StockInsuficiente)` si no hay stock suficiente.
        fn verificar_stock(&self, stock_pedido: u32) -> Result<(), ErrorMarketplace> {
            if stock_pedido > self.stock_a_vender {
                return Err(ErrorMarketplace::StockInsuficiente);
            }
            Ok(())
        }
        // Helper para validar el precio de un producto.
        /// Valida que el precio ingresado sea mayor a cero.
        ///
        /// # Parámetros
        /// - `precio`: Precio a validar.
        ///
        /// # Retorna
        /// - `Ok(())` si el precio es válido.
        /// - `Err(ErrorMarketplace::PrecioInvalido)` si el precio es igual a cero.
        fn validar_precio(precio: &u128) -> Result<(), ErrorMarketplace> {
            if *precio == 0 {
                return Err(ErrorMarketplace::PrecioInvalido);
            }
            Ok(())
        }
    }

    /// Representa una orden de compra en el marketplace.
    ///
    /// Cada orden vincula a un comprador con un producto publicado por un vendedor.
    /// Contiene la cantidad solicitada, el monto total y su estado actual.
    ///
    /// # Campos
    /// - `id`: Identificador único de la orden.
    /// - `comprador`: `AccountId` del usuario que realizó la compra.
    /// - `vendedor`: `AccountId` del usuario que publicó el producto.
    /// - `id_producto`: ID del producto incluido en la orden.
    /// - `cant_producto`: Cantidad solicitada del producto.
    /// - `estado`: Estado actual de la orden (Pendiente, Enviado, Recibido, Cancelada).
    /// - `total`: Monto total de la orden (precio * cantidad)
    #[derive(Debug, PartialEq, Eq, ink::scale::Encode, ink::scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct Orden {
        id: u32,
        comprador: AccountId,
        vendedor: AccountId,
        id_producto: u32,   //id del producto que se ordena
        cant_producto: u16, //cantidad de producto que se ordena
        estado: EstadoOrden,
        total: u128,
    }

    /// Representa el depósito de un vendedor para un producto específico.
    ///
    /// Cada vendedor tiene un depósito individual por producto, donde se almacena el stock disponible.
    /// El depósito se inicializa al registrar un nuevo producto o agregar stock de uno ya existente.
    ///
    /// # Campos
    /// - `id_producto`: Identificador único del producto asociado al depósito.
    /// - `id_vendedor`: `AccountId` del vendedor dueño del depósito.
    /// - `stock`: Cantidad de unidades disponibles en el depósito.
    #[derive(Debug, PartialEq, Eq, ink::scale::Encode, ink::scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct Deposito {
        id_producto: u32,
        id_vendedor: AccountId,
        stock: u32,
    }
    impl Deposito {
        /// Crea un nuevo depósito para un producto y un vendedor determinado.
        ///
        /// # Parámetros
        /// - `id_producto`: ID del producto asociado.
        /// - `id_vendedor`: Cuenta del vendedor dueño del depósito.
        /// - `stock`: Cantidad inicial disponible.
        ///
        /// # Retorna
        /// Una nueva instancia de `Deposito`.
        pub fn new(id_producto: u32, id_vendedor: AccountId, stock: u32) -> Self {
            Self {
                id_producto,
                id_vendedor,
                stock,
            }
        }

        pub fn actualizar_stock(&mut self, stock: u32) {
            self.stock = stock;
        }
    }

    /// Contrato principal del marketplace descentralizado.
    ///
    /// Gestiona usuarios, productos, depósitos, publicaciones y órdenes de compra.
    /// Almacena mappings y contadores necesarios para operar el sistema.
    ///
    /// # Campos
    /// - `usuarios`: Mapping de usuarios registrados.
    /// - `productos`: Mapping de productos registrados.
    /// - `publicaciones`: Mapping de publicaciones activas.
    /// - `ordenes`: Mapping de órdenes de compra.
    /// - `stock_general`: Mapping de depósitos por producto y vendedor.
    /// - `contador_ordenes`: ID incremental de órdenes.
    /// - `contador_publicacion`: ID incremental de publicaciones.
    /// - `contador_productos`: ID incremental de productos.
    #[ink(storage)]
    pub struct MarketPlace {
        usuarios: Mapping<AccountId, Usuario>, //id_usuario -> Usuario, todos los usuarios
        publicaciones: Mapping<u32, Publicacion>, //id_publicacion -> Publicacion, todas las publicaciones
        productos: Mapping<u32, Producto>,        //id_producto -> Producto
        ordenes: Mapping<u32, Orden>,             //id_orden -> Orden
        stock_general: Mapping<(AccountId, u32), Deposito>, // (id_vendedor, id_producto) -> Deposito
        //Atributos auxiliares
        contador_ordenes: u32,
        contador_publicacion: u32,
        contador_productos: u32,
        //aca iria lo de reputacion, creo
    }

    impl Orden {
        /// Crea una nueva orden de compra con estado inicial `Pendiente`.
        ///
        /// # Parámetros
        /// - `id`: Identificador único para la orden.
        /// - `comprador`: `AccountId` del usuario que realiza la compra.
        /// - `vendedor`: `AccountId` del vendedor que publicó el producto.
        /// - `id_producto`: ID del producto solicitado.
        /// - `cant_producto`: Cantidad del producto a comprar.
        /// - `total`: Monto total de la orden (precio * cantidad).
        ///
        /// # Retorna
        /// Una nueva instancia de `Orden` con estado `EstadoOrden::Pendiente`.

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
        /// Marca la orden como enviada.
        ///
        /// # Parámetros
        /// - '&mut self': referencia mutable a la orden que se quiere modificar.
        /// - 'caller: AccountId': cuenta que está intentando cambiar el estado (debe ser el vendedor).
        ///
        /// # Comportamiento
        /// 1. Verifica que el 'caller' sea el vendedor asignado a esta orden.
        /// 2. Verifica que la orden no esté en estado cancelado.
        /// 3. Si las validaciones pasan, cambia el estado de la orden a 'Enviado'.
        ///
        /// # Retorna
        /// - 'Ok(())' si la operación se realizó correctamente.
        /// - 'Err(ErrorMarketplace::NoEsVendedor)' si el 'caller' no es el vendedor.
        /// - 'ErrorMarketplace::OrdenCancelada' si el estado de la orden está como cancelada.
        fn marcar_enviada(&mut self, caller: AccountId) -> Result<(), ErrorMarketplace> {
            //validar que quien llame sea vendedor
            if caller != self.vendedor {
                return Err(ErrorMarketplace::NoEsVendedor);
            }
            //validar que la orden no este cancelada
            if self.estado == EstadoOrden::Cancelada {
                return Err(ErrorMarketplace::OrdenCancelada);
            }
            //como la orden se pone por default en estado "Pendiente", no necesito preguntar si esta pendiente para cambiarla(?

            //cambiar el estado a "Enviado"
            self.estado = EstadoOrden::Enviado;
            Ok(())
        }

        /// Marca la orden como recibida.
        ///
        /// # Parámetros
        /// - '&mut self': referencia mutable a la orden que se quiere modificar.
        /// - 'caller: AccountId': cuenta que está intentando cambiar el estado (debe ser el comprador).
        ///
        /// # Comportamiento
        /// 1. Verifica que el 'caller' sea el comprador asignado a esta orden.
        /// 2. Verifica que la orden no esté cancelada.
        /// 3. Verifica que la orden esté en estado 'Enviado' (sólo puede marcarse como recibida si ya fue enviada).
        /// 4. Si las validaciones pasan, cambia el estado de la orden a 'Recibido'.
        ///
        /// # Retorna
        /// - 'Ok(())' si la operación se realizó correctamente.
        /// - 'Err(ErrorMarketplace::NoEsComprador)' si el 'caller' no es el comprador.
        /// - 'Err(ErrorMarketplace::OrdenCancelada)' si el estado de la orden está como cancelada.
        /// - 'Err(ErrorMarketplace::EstadoInvalido)' si la orden no está en estado 'Enviado'.
        fn marcar_recibida(&mut self, caller: AccountId) -> Result<(), ErrorMarketplace> {
            //validar que quien llama sea el comprador
            if caller != self.comprador {
                return Err(ErrorMarketplace::NoEsComprador);
            }
            //validar que la orden no este cancelada
            if self.estado == EstadoOrden::Cancelada {
                return Err(ErrorMarketplace::OrdenCancelada);
            }
            //solo se marca como recibida si ya fue enviada
            if self.estado != EstadoOrden::Enviado {
                return Err(ErrorMarketplace::EstadoInvalido);
            }
            //cambiar el estado a "Recibido"
            self.estado = EstadoOrden::Recibido;
            Ok(())
        }
    }

    impl MarketPlace {
        #[ink(constructor)]
        /// Inicializa un nuevo contrato `MarketPlace` con todos los mappings vacíos y contadores en cero.
        pub fn new() -> Self {
            Self {
                usuarios: Mapping::default(),
                productos: Mapping::default(),
                ordenes: Mapping::default(),
                publicaciones: Mapping::default(),
                stock_general: Mapping::default(),
                contador_ordenes: 0,
                contador_publicacion: 0,
                contador_productos: 0,
            }
        }

        /// Registra un nuevo producto para el vendedor que llama a la función.
        ///
        /// Valida los datos, normaliza el nombre, crea el producto si no existe,
        /// o inicializa un depósito si el producto ya existe pero no para este vendedor.
        ///
        /// # Parámetros
        /// - `nombre`: Nombre del producto.
        /// - `descripcion`: Descripción del producto.
        /// - `categoria`: Categoría del producto.
        /// - `stock`: Stock inicial del producto.
        ///
        /// # Errores
        /// - `RolInvalido`: Si el usuario no es vendedor.
        /// - `ProductoYaPoseeDeposito`: Si el vendedor ya tiene depósito para ese producto.
        /// - Otros errores de validación.
        #[ink(message)]
        pub fn registrar_producto(
            &mut self,
            nombre: String,
            descripcion: String,
            categoria: Categoria,
            stock: u32,
        ) -> Result<(), ErrorMarketplace> {
            let caller = self.env().caller();
            self._registrar_producto(caller, nombre, descripcion, categoria, stock)?;
            Ok(())
        }

        /// Lógica interna para registrar un producto asociado a un vendedor específico.
        ///
        /// Busca si el producto existe por nombre normalizado.
        /// Si no existe, lo crea y crea depósito con el stock dado.
        /// Si existe, verifica que el vendedor no tenga depósito y lo inicializa.
        fn _registrar_producto(
            &mut self,
            id_vendedor: AccountId,
            nombre: String,
            descripcion: String,
            categoria: Categoria,
            stock: u32,
        ) -> Result<(), ErrorMarketplace> {
            self.verificar_rol_vendedor(id_vendedor)?;
            //validar producto
            Producto::validar_nombre_producto(&nombre)?;
            //validar descripcion
            Producto::validar_descripcion(&descripcion)?;
            //Normalizar nombre
            let nombre_normalizado = Producto::normalizar_nombre_producto(&nombre);
            //buscar en catalogo traigo el id del producto si existe con el nombre normalizado
            match self.buscar_producto_por_nombre(&nombre_normalizado) {
                // Si el producto NO existe, lo creamos y asociamos depósito al vendedor
                Err(ErrorMarketplace::ProductoNoExiste) => {
                    let id_producto = self.obtener_nuevo_id_producto()?;
                    let nuevo_producto = Producto::new(
                        id_producto,
                        nombre_normalizado,
                        descripcion.clone(),
                        categoria.clone(),
                    );
                    self.insertar_producto_en_catalogo(nuevo_producto)?;
                    self.inicializar_deposito(id_vendedor, id_producto, stock)?;
                    Ok(())
                }
                // Si el producto existe, verificamos si el vendedor ya tiene depósito
                Ok(id_producto_existente) => {
                    if self
                        .vendedor_tiene_deposito_para_producto(id_vendedor, id_producto_existente)
                    {
                        Err(ErrorMarketplace::ProductoYaPoseeDeposito)
                    } else {
                        // El producto existe pero el vendedor NO tiene depósito, lo inicializamos
                        self.inicializar_deposito(id_vendedor, id_producto_existente, stock)?;
                        Ok(())
                    }
                }
                // Cualquier otro error inesperado lo propagamos
                Err(e) => Err(e),
            }
        }

        //FUNCIONES AUXILIARES
        /// Verifica que el rol actual de un usuario sea diferente al nuevo rol que se quiere asignar.
        ///
        /// Esto previene cambios innecesarios o errores al intentar asignar el mismo rol.
        /// Retorna error `RolYaAsignado` si el rol es idéntico.
        ///
        /// # Parámetros
        /// - `id`: ID del usuario a verificar.
        /// - `nuevo_rol`: Nuevo rol que se desea asignar.
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
        /// Busca un usuario por su `AccountId` y devuelve su struct `Usuario`.
        ///
        /// Retorna error `UsuarioNoExiste` si no está registrado.
        ///
        /// # Parámetros
        /// - `id`: Cuenta del usuario.
        fn verificar_usuario_existe(&self, id: AccountId) -> Result<Usuario, ErrorMarketplace> {
            self.usuarios
                .get(&id)
                .ok_or(ErrorMarketplace::UsuarioNoExiste)
        }

        //Helper verificar que el usuario tenga el rol correcto
        /// Verifica que un usuario tenga rol de vendedor.
        ///
        /// Retorna error si el usuario no existe o si su rol no es vendedor.
        ///
        /// # Parámetros
        /// - `id`: Cuenta del usuario.
        fn verificar_rol_vendedor(&self, id: AccountId) -> Result<(), ErrorMarketplace> {
            let usuario = self.verificar_usuario_existe(id)?;
            Usuario::validar_rol_vendedor(&usuario.rol)?;
            Ok(())
        }

        //Helper verificar que el usuario tenga el rol correcto
        /// Verifica que un usuario tenga rol de comprador.
        ///
        /// Retorna error si el usuario no existe o si su rol no es comprador.
        ///
        /// # Parámetros
        /// - `id`: Cuenta del usuario.
        fn verificar_rol_comprador(&self, id: AccountId) -> Result<(), ErrorMarketplace> {
            let usuario = self.verificar_usuario_existe(id)?;
            Usuario::validar_rol_comprador(&usuario.rol)?;
            Ok(())
        }

        //Helper para obtener una publicacion por id
        /// Devuelve la publicación asociada a un ID dado.
        ///
        /// Retorna error `PublicacionNoExiste` si no se encuentra.
        ///
        /// # Parámetros
        /// - `id_publicacion`: ID de la publicación.
        fn obtener_publicacion(
            &self,
            id_publicacion: u32,
        ) -> Result<Publicacion, ErrorMarketplace> {
            self.publicaciones
                .get(&id_publicacion)
                .ok_or(ErrorMarketplace::PublicacionNoExiste)
        }

        //Helper para verificar si un producto existe y no tiene deposito para el vendedor
        /// Indica si un vendedor tiene un depósito activo para un producto dado.
        ///
        /// # Parámetros
        /// - `id_vendedor`: Cuenta del vendedor.
        /// - `id_producto`: ID del producto.
        fn vendedor_tiene_deposito_para_producto(
            &self,
            id_vendedor: AccountId,
            id_producto: u32,
        ) -> bool {
            self.stock_general.contains(&(id_vendedor, id_producto))
        }

        //Helper para buscar nombre de producto
        /// Busca un producto por nombre normalizado y devuelve su ID.
        ///
        /// Recorre el catálogo de productos registrados.
        ///
        /// Retorna error `ProductoNoExiste` si no encuentra coincidencias.
        ///
        /// # Parámetros
        /// - `nombre`: Nombre normalizado del producto.
        fn buscar_producto_por_nombre(&self, nombre: &String) -> Result<u32, ErrorMarketplace> {
            let nombre_normalizado = Producto::normalizar_nombre_producto(nombre);
            for id in 1..=self.contador_productos {
                if let Some(producto) = self.productos.get(&id) {
                    if Producto::normalizar_nombre_producto(&producto.nombre) == nombre_normalizado
                    {
                        return Ok(id);
                    }
                }
            }
            Err(ErrorMarketplace::ProductoNoExiste)
        }

        //Helper para obtener un nuevo id de publicacion
        /// Incrementa y devuelve un nuevo ID para una publicación.
        ///
        /// Retorna error si el contador excede el límite permitido.
        fn obtener_nuevo_id_publicacion(&mut self) -> Result<u32, ErrorMarketplace> {
            self.contador_publicacion = self
                .contador_publicacion
                .checked_add(1)
                .ok_or(ErrorMarketplace::IDPublicacionEnUso)?;
            Ok(self.contador_publicacion)
        }

        //Helper para obtener nuevo id de producto
        /// Incrementa y devuelve un nuevo ID para un producto.
        ///
        /// Retorna error si el contador excede el límite permitido.
        fn obtener_nuevo_id_producto(&mut self) -> Result<u32, ErrorMarketplace> {
            self.contador_productos = self
                .contador_productos
                .checked_add(1)
                .ok_or(ErrorMarketplace::IDProductoEnUso)?;
            Ok(self.contador_productos)
        }

        //Helper para verificar que id no este en uso
        /// Verifica que un ID de producto no esté registrado en el catálogo.
        ///
        /// Retorna error si el ID ya está en uso.
        fn verificar_id_producto_en_uso(&self, id_producto: u32) -> Result<(), ErrorMarketplace> {
            if self.productos.contains(&id_producto) {
                return Err(ErrorMarketplace::IDProductoEnUso);
            }
            Ok(())
        }
        //Helper para verificar que id de publicacion no este en uso
        /// Verifica que un ID de publicación no esté registrado.
        ///
        /// Retorna error si el ID ya está en uso.
        fn verificar_id_publicacion_en_uso(
            &self,
            id_publicacion: u32,
        ) -> Result<(), ErrorMarketplace> {
            if self.publicaciones.contains(&id_publicacion) {
                return Err(ErrorMarketplace::IDPublicacionEnUso);
            }
            Ok(())
        }

        //Helper para insertar producto en el catalogo de productos
        /// Inserta un producto nuevo en el catálogo.
        ///
        /// Verifica primero que el ID no esté en uso.
        ///
        /// # Parámetros
        /// - `producto`: Producto a insertar.
        ///
        /// # Retorna
        /// - `Ok(())` si la inserción fue exitosa.
        /// - `Err(ErrorMarketplace::IDProductoEnUso)` si el ID del producto ya está en uso.
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

        /// Helper para insertar una publicación en el sistema.
        /// /// Inserta una publicación nueva en el sistema.
        ///
        /// Verifica primero que el ID no esté en uso.
        ///
        /// # Parámetros
        /// - `publicacion`: Publicación a insertar.
        ///
        /// # Retorna
        /// - `Ok(())` si la inserción fue exitosa.
        /// - `Err(ErrorMarketplace::IDPublicacionEnUso)` si el ID de publicación ya está en uso
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
        /// Obtiene el stock disponible de un producto en el depósito de un vendedor.
        ///
        /// Retorna error si el depósito no existe.
        ///
        /// # Parámetros
        /// - `id_vendedor`: Cuenta del vendedor.
        /// - `id_producto`: ID del producto.
        ///
        /// # Retorna
        /// - `Ok(stock)` con la cantidad disponible si existe el depósito.
        /// - `Err(ErrorMarketplace::DepositoNoEncontrado)` si no existe el depósito para el vendedor y producto dados.
        fn obtener_stock_deposito(
            &self,
            id_vendedor: AccountId,
            id_producto: u32,
        ) -> Result<u32, ErrorMarketplace> {
            self.stock_general
                .get(&(id_vendedor, id_producto))
                .map_or(Err(ErrorMarketplace::DepositoNoEncontrado), |deposito| {
                    Ok(deposito.stock)
                })
        }

        ///Helper para verificar que tengo suficiente stock en el depósito del vendedor.
        /// /// Verifica que el stock en depósito sea suficiente para una venta.
        ///
        /// Retorna error si el stock es insuficiente.
        ///
        /// # Parámetros
        /// - `id_vendedor`: Cuenta del vendedor.
        /// - `id_producto`: ID del producto.
        /// - `stock_a_vender`: Cantidad requerida para la venta.
        ///
        /// # Retorna
        /// - `Ok(())` si hay stock suficiente.
        /// - `Err(ErrorMarketplace::StockDepositoInsuficiente)` si no hay stock suficiente.
        fn validar_stock_deposito(
            &self,
            id_vendedor: AccountId,
            id_producto: u32,
            stock_a_vender: u32,
        ) -> Result<(), ErrorMarketplace> {
            // Verificar que el depósito exista
            let stock_actual = self.obtener_stock_deposito(id_vendedor, id_producto)?;
            if stock_actual < stock_a_vender {
                return Err(ErrorMarketplace::StockDepositoInsuficiente);
            }
            Ok(())
        }

        /// Helper para actualizar el stock de un producto en el depósito de un vendedor.
        /// /// Actualiza el stock de un producto en el depósito tras una venta.
        ///
        /// Realiza validaciones y persiste el nuevo stock.
        ///
        /// # Parámetros
        /// - `id_vendedor`: Cuenta del vendedor.
        /// - `id_producto`: ID del producto.
        /// - `stock_a_vender`: Cantidad vendida a descontar.
        ///
        /// # Retorna
        /// - `Ok(())` si la actualización fue exitosa.
        /// - `Err(ErrorMarketplace::ProductoNoExiste)` si no existe el depósito.
        /// - `Err(ErrorMarketplace::StockDepositoInsuficiente)` si el stock es insuficiente.
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

            let nuevo_stock = stock_actual
                .checked_sub(stock_a_vender)
                .ok_or(ErrorMarketplace::StockInsuficiente)?;

            let mut deposito = self
                .stock_general
                .get(&(id_vendedor, id_producto))
                .ok_or(ErrorMarketplace::ProductoNoExiste)?;

            deposito.actualizar_stock(nuevo_stock);

            //se debe volver a insertar para poder actualizar el stock
            self.stock_general
                .insert((id_vendedor, id_producto), &deposito);
            Ok(())
        }

        ///Funcion que modifica el stock de un depósito de un vendedor.
        /// Mensaje público para modificar el stock de un depósito.
        ///
        /// Llama al helper interno con el caller como vendedor.
        ///
        /// # Parámetros
        /// - `nombre_producto`: Nombre del producto.
        /// - `stock`: Nuevo stock a asignar.
        ///
        /// # Retorna
        /// - `Ok(())` si la modificación fue exitosa.
        /// - `Err(ErrorMarketplace)` si ocurre un error de validación o permisos.
        #[ink(message)]
        pub fn modificar_stock_deposito(
            &mut self,
            nombre_producto: String,
            stock: u32,
        ) -> Result<(), ErrorMarketplace> {
            let caller = self.env().caller();
            self._modificar_stock_deposito(caller, nombre_producto, stock)?;
            Ok(())
        }

        /// Helper interno para modificar stock de depósito.
        ///
        /// Verifica que el usuario sea vendedor y que el stock sea válido.
        ///
        /// Actualiza el depósito y persiste cambios.
        ///
        /// # Parámetros
        /// - `id_vendedor`: Cuenta del vendedor.
        /// - `nombre_producto`: Nombre del producto.
        /// - `stock`: Nuevo stock.
        ///
        /// # Retorna
        /// - `Ok(())` si la modificación fue exitosa.
        /// - `Err(ErrorMarketplace::ProductoNoExiste)` si no existe el depósito.
        /// - `Err(ErrorMarketplace::RolInvalido)` si el usuario no es vendedor.
        /// - `Err(ErrorMarketplace::StockInvalido)` si el stock no es válido.
        fn _modificar_stock_deposito(
            &mut self,
            id_vendedor: AccountId,
            nombre_producto: String,
            stock: u32,
        ) -> Result<(), ErrorMarketplace> {
            // Verificar que el usuario sea vendedor
            self.verificar_rol_vendedor(id_vendedor)?;
            // Verificar que el stock sea válido
            Producto::validar_stock_producto(&stock)?;

            // Normalizar el nombre del producto
            let nombre_producto_normalizado =
                Producto::normalizar_nombre_producto(&nombre_producto);

            // validar que exista en el catalogo
            let id_producto = self
                .buscar_producto_por_nombre(&nombre_producto_normalizado)
                .map_err(|_| ErrorMarketplace::ProductoNoExiste)?;

            // Obtener el depósito del vendedor
            let mut deposito = self
                .stock_general
                .get(&(id_vendedor, id_producto))
                .ok_or(ErrorMarketplace::ProductoNoExiste)?;
            // Actualizar el stock del depósito
            deposito.actualizar_stock(stock);
            // Guardar el depósito actualizado en el mapping
            self.stock_general
                .insert((id_vendedor, id_producto), &deposito);
            Ok(())
        }

        //Helper para inicializar un depósito para un vendedor.
        /// Inicializa un depósito para un vendedor y producto con un stock dado.
        ///
        /// Valida rol vendedor y stock válido.
        ///
        /// # Parámetros
        /// - `id_vendedor`: Cuenta del vendedor.
        /// - `id_producto`: ID del producto.
        /// - `stock`: Stock inicial.
        ///
        /// # Retorna
        /// - `Ok(())` si la inicialización fue exitosa.
        /// - `Err(ErrorMarketplace::RolInvalido)` si el usuario no es vendedor.
        /// - `Err(ErrorMarketplace::StockInvalido)` si el stock no es válido.
        fn inicializar_deposito(
            &mut self,
            id_vendedor: AccountId,
            id_producto: u32,
            stock: u32,
        ) -> Result<(), ErrorMarketplace> {
            // Verificar que el usuario sea vendedor
            self.verificar_rol_vendedor(id_vendedor)?;
            // Verificar que el stock sea válido
            Producto::validar_stock_producto(&stock)?;
            // Crear un nuevo depósito
            let deposito = Deposito::new(id_producto, id_vendedor, stock);
            // Insertar el depósito en el mapping
            self.stock_general
                .insert((id_vendedor, id_producto), &deposito);
            Ok(())
        }

        /// Registra un nuevo usuario con un nombre de usuario y rol dado.
        ///
        /// El `caller` es la cuenta que llama a esta función y será usada como ID del usuario.
        ///
        /// # Parámetros
        /// - `username`: Nombre del usuario.
        /// - `rol`: Rol asignado al usuario.
        ///
        /// # Retorna
        /// - `Ok(())` si el usuario fue registrado correctamente.
        /// - `Err(ErrorMarketplace::UsuarioYaRegistrado)` si el usuario ya existe.
        #[ink(message)]
        pub fn registrar_usuario(
            &mut self,
            username: String,
            rol: Rol,
        ) -> Result<(), ErrorMarketplace> {
            let caller = self.env().caller();
            self._registrar_usuario(username, rol, caller)?;
            Ok(())
        }

        /// Helper interno para registrar un usuario.
        ///
        /// # Parámetros
        /// - `username`: Nombre del usuario.
        /// - `rol`: Rol asignado.
        /// - `id`: Cuenta del usuario.
        ///
        /// # Retorna
        /// - `Ok(())` si el usuario fue registrado correctamente.
        /// - `Err(ErrorMarketplace::UsuarioYaRegistrado)` si ya existe un usuario con el mismo ID.
        fn _registrar_usuario(
            &mut self,
            username: String,
            rol: Rol,
            id: AccountId,
        ) -> Result<(), ErrorMarketplace> {
            // Verificar si el usuario ya está registrado
            if self.usuarios.contains(&id) {
                return Err(ErrorMarketplace::UsuarioYaRegistrado);
            }
            // Crear un nuevo usuario
            let nuevo_usuario = Usuario::new(username, rol, id);
            // Insertar el usuario en el mapping
            self.usuarios.insert(id, &nuevo_usuario);
            Ok(())
        }

        /// Modifica el rol del usuario que llama a esta función.
        ///
        /// # Parámetros
        /// - `nuevo_rol`: Nuevo rol a asignar.
        ///
        /// # Retorna
        /// - `Ok(())` si el rol fue modificado exitosamente.
        /// - `Err(ErrorMarketplace)` si el usuario no existe o si el nuevo rol es igual al actual.
        #[ink(message)]
        pub fn modificar_rol(&mut self, nuevo_rol: Rol) -> Result<(), ErrorMarketplace> {
            let caller = self.env().caller();
            self._modificar_rol(caller, nuevo_rol)?;
            Ok(())
        }

        /// Helper interno para modificar el rol de un usuario.
        ///
        /// # Parámetros
        /// - `id_usuario`: Cuenta del usuario.
        /// - `nuevo_rol`: Nuevo rol a asignar.
        ///
        /// # Retorna
        /// - `Ok(())` si el rol fue modificado exitosamente.
        /// - `Err(ErrorMarketplace::UsuarioNoExiste)` si el usuario no existe.
        /// - `Err(ErrorMarketplace::RolYaAsignado)` si el nuevo rol es igual al actual.
        fn _modificar_rol(
            &mut self,
            id_usuario: AccountId,
            nuevo_rol: Rol,
        ) -> Result<(), ErrorMarketplace> {
            // Verifica si el usuario existe
            let mut usuario = self.verificar_usuario_existe(id_usuario)?;
            // Verifica que el nuevo rol sea diferente
            self.verificar_rol_es_diferente(id_usuario, nuevo_rol)?;
            // Actualiza el rol
            usuario.rol = nuevo_rol;
            self.usuarios.insert(id_usuario, &usuario);
            Ok(())
        }

        //Publicar producto
        /// Crea una nueva publicación para un producto dado, con stock y precio.
        ///
        /// El caller debe ser un vendedor registrado.
        ///
        /// # Parámetros
        /// - `nombre_producto`: Nombre del producto a publicar.
        /// - `stock_a_vender`: Cantidad de producto a vender en esta publicación.
        /// - `precio`: Precio unitario.
        ///
        /// # Retorna
        /// - `Ok(())` si la publicación fue creada exitosamente.
        /// - `Err(ErrorMarketplace)` en caso de errores de validación o permisos.
        #[ink(message)]
        pub fn crear_publicacion(
            &mut self,
            nombre_producto: String,
            stock_a_vender: u32,
            precio: u128,
        ) -> Result<(), ErrorMarketplace> {
            let caller = self.env().caller();
            //llamar helper de crear publicacion
            self._crear_publicacion(nombre_producto, caller, stock_a_vender, precio)?;
            Ok(())
        }

        /// Helper interno para crear una publicación.
        ///
        /// Realiza validaciones de existencia de usuario, rol, stock y precio.
        ///
        /// # Parámetros
        /// - `nombre_producto`: Nombre del producto.
        /// - `id_vendedor`: Cuenta del vendedor.
        /// - `stock_a_vender`: Cantidad a vender.
        /// - `precio`: Precio unitario.
        ///
        /// # Retorna
        /// - `Ok(())` si la publicación fue creada exitosamente.
        /// - `Err(ErrorMarketplace)` si ocurre algún error en las validaciones o inserciones.
        fn _crear_publicacion(
            &mut self,
            nombre_producto: String,
            id_vendedor: AccountId,
            stock_a_vender: u32,
            precio: u128,
        ) -> Result<(), ErrorMarketplace> {
            self.verificar_usuario_existe(id_vendedor)?;
            self.verificar_rol_vendedor(id_vendedor)?;
            //Validar precio
            Publicacion::validar_precio(&precio)?;
            //normalizar nombre de producto
            let nombre_producto_normalizado =
                Producto::normalizar_nombre_producto(&nombre_producto);

            // validar que exista en el catalogo
            let id_producto = self
                .buscar_producto_por_nombre(&nombre_producto_normalizado)
                .map_err(|_| ErrorMarketplace::ProductoNoExiste)?;

            //Validar deposito si existe y si cant a vender es menor al stock total del deposito
            self.validar_stock_deposito(id_vendedor, id_producto, stock_a_vender)?;

            // Generamos un nuevo ID para la publicación
            let id_publicacion = self.obtener_nuevo_id_publicacion()?;

            // Creamos una nueva publicación
            let nueva_publicacion = Publicacion::new(
                id_publicacion,
                id_vendedor, // id del vendedor
                id_producto,
                precio,
                stock_a_vender,
            );

            //Guardamos la publicación en el mapping de publicaciones
            self.insertar_publicacion(nueva_publicacion.clone())?;

            // Actualizamos el stock del producto del vendedor
            self.actualizar_stock_producto(id_vendedor, id_producto, stock_a_vender)?;
            Ok(())
        }

        /// Crea una orden de compra para una publicación con la cantidad y monto dado.
        ///
        /// El caller debe ser un comprador registrado.
        ///
        /// # Parámetros
        /// - `id_publicacion`: ID de la publicación a comprar.
        /// - `cant_producto`: Cantidad de producto a comprar.
        /// - `monto_dado`: Monto que el comprador entrega para pagar.
        ///
        /// # Retorna
        /// - `Ok(())` si la orden fue creada correctamente.
        /// - `Err(ErrorMarketplace)` si hay errores en validaciones o permisos.
        #[ink(message)]
        pub fn crear_orden(
            &mut self,
            id_publicacion: u32,
            cant_producto: u16,
            monto_dado: u128,
        ) -> Result<(), ErrorMarketplace> {
            //monto dado es el monto que el comprador me da para pagar la orden
            let caller = self.env().caller();
            self._crear_orden(caller, id_publicacion, cant_producto, monto_dado)?;
            Ok(())
        }

        fn _crear_orden(
            &mut self,
            id_comprador: AccountId,
            id_publicacion: u32,
            cant_producto: u16,
            monto_dado: u128,
        ) -> Result<(), ErrorMarketplace> {
            // Verificar que el usuario exista
            self.verificar_usuario_existe(id_comprador)?;

            // Verificar que el usuario sea comprador
            self.verificar_rol_comprador(id_comprador)?;

            // Verificar que la publicación exista
            let publicacion = self.obtener_publicacion(id_publicacion)?;

            // Verificar que el stock sea suficiente y asi poder crear la orden
            publicacion.verificar_stock(cant_producto as u32)?;

            let tot_orden = match publicacion.precio.checked_mul(cant_producto as u128) {
                Some(valor) => valor,
                None => return Err(ErrorMarketplace::Overflow),
            };
            // Verificar que el monto dado sea suficiente para cubrir el total de la orden
            if monto_dado < tot_orden {
                return Err(ErrorMarketplace::MontoInsuficiente);
            }

            // Crear nueva orden
            let nueva_id = self.contador_ordenes;
            let orden = Orden::new(
                nueva_id,
                id_comprador,
                publicacion.id_vendedor,
                publicacion.id_producto,
                cant_producto,
                tot_orden,
            );

            self.ordenes.insert(nueva_id, &orden);

            self.contador_ordenes = self
                .contador_ordenes
                .checked_add(1)
                .ok_or(ErrorMarketplace::Overflow)?;

            Ok(())
        }

        // Busca la orden con el ID dado dentro del Mapping ordenes
        /// Función privada que marca una orden como enviada.
        ///
        /// # Parámetros
        /// - '&mut self': referencia mutable al Marketplace.
        /// - 'caller: AccountId': cuenta que realiza la acción (debe ser vendedor).
        /// - 'id_orden: u32': identificador único de la orden a modificar.
        ///
        /// # Comportamiento
        /// 1. Obtiene la orden con el 'id_orden'.
        /// 2. Llama al método 'marcar_enviada' de la orden pasándole el `caller`.
        /// 3. Si la operación es exitosa, actualiza la orden en el mapping.
        ///
        /// # Retorna
        /// - 'Ok(())' si la orden fue marcada como enviada correctamente.
        /// - 'Err(ErrorMarketplace::OrdenNoExiste)' si no existe la orden con el 'id_orden' dado.
        /// - Propaga otros errores que retorne 'marcar_enviada'.
        fn _marcar_orden_como_enviada(
            &mut self,
            caller: AccountId,
            id_orden: u32,
        ) -> Result<(), ErrorMarketplace> {
            if let Some(mut orden) = self.ordenes.get(id_orden) {
                match orden.marcar_enviada(caller) {
                    Ok(()) => {
                        self.ordenes.insert(id_orden, &orden);
                        Ok(())
                    }
                    Err(e) => Err(e),
                }
            } else {
                Err(ErrorMarketplace::OrdenNoExiste)
            }
        }

        /// Método público para marcar una orden como enviada.
        ///
        /// # Parámetros
        /// - '&mut self': referencia mutable al Marketplace.
        /// - 'id_orden: u32': identificador único de la orden a modificar.
        ///
        /// # Comportamiento
        /// 1. Obtiene automáticamente el 'caller' de la llamada.
        /// 2. Llama a la función privada '_marcar_orden_como_enviada'.
        ///
        /// # Retorna
        /// - 'Ok(())' si la orden fue marcada como enviada correctamente.
        /// - Propaga errores desde '_marcar_orden_como_enviada'.
        #[ink(message)]
        pub fn marcar_orden_como_enviada(&mut self, id_orden: u32) -> Result<(), ErrorMarketplace> {
            let caller = self.env().caller();
            self._marcar_orden_como_enviada(caller, id_orden)
        }

        /// Función privada que marca una orden como recibida.
        ///
        /// # Parámetros
        /// - '&mut self': referencia mutable al Marketplace.
        /// - 'caller: AccountId': cuenta que realiza la acción (debe ser comprador).
        /// - 'id_orden: u32': identificador único de la orden a modificar.
        ///
        /// # Comportamiento
        /// 1. Obtiene la orden con el 'id_orden'.
        /// 2. Llama al método 'marcar_recibida' de la orden pasándole el 'caller'.
        /// 3. Si la operación es exitosa, actualiza la orden en el mapping.
        ///
        /// # Retorna
        /// - 'Ok(())' si la orden fue marcada como recibida correctamente.
        /// - 'Err(ErrorMarketplace::OrdenNoExiste)' si no existe la orden con el 'id_orden' dado.
        /// - Propaga otros errores que retorne 'marcar_recibida'.
        fn _marcar_orden_como_recibida(
            &mut self,
            caller: AccountId,
            id_orden: u32,
        ) -> Result<(), ErrorMarketplace> {
            if let Some(mut orden) = self.ordenes.get(id_orden) {
                match orden.marcar_recibida(caller) {
                    Ok(()) => {
                        self.ordenes.insert(id_orden, &orden);
                        Ok(())
                    }
                    Err(e) => Err(e),
                }
            } else {
                Err(ErrorMarketplace::OrdenNoExiste)
            }
        }

        /// Método público para marcar una orden como recibida.
        ///
        /// # Parámetros
        /// - '&mut self': referencia mutable al Marketplace.
        /// - 'id_orden: u32': identificador único de la orden a modificar.
        ///
        /// # Comportamiento
        /// 1. Obtiene automáticamente el 'caller' de la llamada.
        /// 2. Llama a la función privada '_marcar_orden_como_recibida'.
        ///
        /// # Retorna
        /// - 'Ok(())' si la orden fue marcada como recibida correctamente.
        /// - Propaga errores desde '_marcar_orden_como_recibida'.
        #[ink(message)]
        pub fn marcar_orden_como_recibida(
            &mut self,
            id_orden: u32,
        ) -> Result<(), ErrorMarketplace> {
            let caller = self.env().caller();
            self._marcar_orden_como_recibida(caller, id_orden)
        }
    }
    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        use ink::primitives::AccountId;

        use crate::market_place::{
            Categoria, ErrorMarketplace, EstadoOrden, MarketPlace, Orden, Producto, Rol, Usuario,
        };

        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        fn account(id: u8) -> AccountId {
            AccountId::from([id; 32])
        }

        fn nuevo_contrato() -> MarketPlace {
            MarketPlace::new()
        }

        fn set_caller(caller: AccountId) {
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(caller);
        }

        fn contract_dummy() -> MarketPlace {
            let mut contract = nuevo_contrato();
            set_caller(account(1));
            contract
                .registrar_usuario("user1".to_string(), Rol::Comprador)
                .ok();
            set_caller(account(2));
            contract
                .registrar_usuario("user2".to_string(), Rol::Vendedor)
                .ok();
            set_caller(account(3));
            contract
                .registrar_usuario("user3".to_string(), Rol::Ambos)
                .ok();
            contract
        }
        #[test]
        fn test_orden_enviada_ok() {
            let mut orden = Orden::new(1, account(1), account(2), 10, 3, 300);
            let res = orden.marcar_enviada(account(2));
            assert_eq!(res, Ok(()));
            assert_eq!(orden.estado, EstadoOrden::Enviado);
        }

        #[test]
        fn test_orden_enviada_no_autorizado() {
            let mut orden = Orden::new(1, account(1), account(2), 10, 3, 300);
            let res = orden.marcar_enviada(account(3));
            assert_eq!(res, Err(ErrorMarketplace::NoEsVendedor));
        }

        #[test]
        fn test_orden_recibida_ok() {
            let mut orden = Orden::new(1, account(1), account(2), 10, 3, 300);
            orden.estado = EstadoOrden::Enviado;
            let res = orden.marcar_recibida(account(1));
            assert_eq!(res, Ok(()));
            assert_eq!(orden.estado, EstadoOrden::Recibido);
        }

        #[test]
        fn test_marcar_recibida_estado_invalido() {
            let mut orden = Orden::new(1, account(1), account(2), 10, 3, 300);
            let res = orden.marcar_recibida(account(1));
            assert_eq!(res, Err(ErrorMarketplace::EstadoInvalido));
        }

        #[test]
        fn test_orden_recibida_error() {
            let mut orden = Orden::new(1, account(1), account(2), 10, 3, 300);
            orden.estado = EstadoOrden::Cancelada;
            let res = orden.marcar_recibida(account(1));
            assert_eq!(res, Err(ErrorMarketplace::OrdenCancelada));
        }

        #[test]
        fn test_marcar_recibida_no_autorizado() {
            let mut orden = Orden::new(1, account(1), account(2), 10, 3, 300);
            orden.estado = EstadoOrden::Enviado;
            let res = orden.marcar_recibida(account(3)); // caller NO es el comprador
            assert_eq!(res, Err(ErrorMarketplace::NoEsComprador));
        }

        #[ink::test]
        fn test_marcar_orden_como_enviada_ok() {
            let mut contrato = contract_dummy();

            let orden = Orden::new(1, account(1), account(2), 10, 3, 300);
            contrato.ordenes.insert(1, &orden);

            let res = contrato._marcar_orden_como_enviada(account(2), 1);
            assert_eq!(res, Ok(()));

            let actualizada = contrato.ordenes.get(1).unwrap();
            assert_eq!(actualizada.estado, EstadoOrden::Enviado);
        }

        #[ink::test]
        fn test_marcar_orden_como_enviada_orden_no_existe() {
            let mut contrato = contract_dummy();

            let res = contrato._marcar_orden_como_enviada(account(2), 999);
            assert_eq!(res, Err(ErrorMarketplace::OrdenNoExiste));
        }

        #[ink::test]
        fn test_marcar_orden_como_enviada_no_es_vendedor() {
            let mut contrato = contract_dummy();

            let orden = Orden::new(1, account(1), account(2), 10, 3, 300);
            contrato.ordenes.insert(1, &orden);

            let res = contrato._marcar_orden_como_enviada(account(3), 1);
            assert_eq!(res, Err(ErrorMarketplace::NoEsVendedor));
        }

        #[ink::test]
        fn test_marcar_orden_como_enviada_cancelada() {
            let mut contrato = contract_dummy();

            let mut orden = Orden::new(1, account(1), account(2), 10, 3, 300);
            orden.estado = EstadoOrden::Cancelada;
            contrato.ordenes.insert(1, &orden);

            let res = contrato._marcar_orden_como_enviada(account(2), 1);
            assert_eq!(res, Err(ErrorMarketplace::OrdenCancelada));
        }

        #[ink::test]
        fn test_marcar_orden_como_recibida_ok() {
            let mut contrato = contract_dummy();

            let mut orden = Orden::new(1, account(1), account(2), 10, 3, 300);
            orden.estado = EstadoOrden::Enviado;
            contrato.ordenes.insert(1, &orden);

            let res = contrato._marcar_orden_como_recibida(account(1), 1);
            assert_eq!(res, Ok(()));

            let actualizada = contrato.ordenes.get(1).unwrap();
            assert_eq!(actualizada.estado, EstadoOrden::Recibido);
        }

        #[ink::test]
        fn test_marcar_orden_como_recibida_orden_no_existe() {
            let mut contrato = contract_dummy();

            let res = contrato._marcar_orden_como_recibida(account(1), 999);
            assert_eq!(res, Err(ErrorMarketplace::OrdenNoExiste));
        }

        #[ink::test]
        fn test_marcar_orden_como_recibida_no_es_comprador() {
            let mut contrato = contract_dummy();

            let mut orden = Orden::new(1, account(1), account(2), 10, 3, 300);
            orden.estado = EstadoOrden::Enviado;
            contrato.ordenes.insert(1, &orden);

            let res = contrato._marcar_orden_como_recibida(account(3), 1);
            assert_eq!(res, Err(ErrorMarketplace::NoEsComprador));
        }

        #[ink::test]
        fn test_marcar_orden_como_recibida_estado_invalido() {
            let mut contrato = contract_dummy();

            let orden = Orden::new(1, account(1), account(2), 10, 3, 300); // estado: Pendiente
            contrato.ordenes.insert(1, &orden);

            let res = contrato._marcar_orden_como_recibida(account(1), 1);
            assert_eq!(res, Err(ErrorMarketplace::EstadoInvalido));
        }

        /// Test MarketPlace
        #[ink::test]
        fn registrar_usuario_ok() {
            let mut contrato = nuevo_contrato();
            set_caller(account(1));
            let account_id = account(1);
            let res = contrato._registrar_usuario("nico".to_string(), Rol::Comprador, account_id);
            assert_eq!(res, Ok(()));
        }

        #[ink::test]
        fn registrar_usuario_ya_existente_falla() {
            let mut contrato = contract_dummy();
            set_caller(account(1));
            let _ = contrato.registrar_usuario("nico".to_string(), Rol::Comprador);
            let res = contrato.registrar_usuario("nico".to_string(), Rol::Vendedor);

            assert_eq!(res, Err(ErrorMarketplace::UsuarioYaRegistrado));
        }

        #[ink::test]
        fn modificar_rol_ok() {
            let mut contrato = nuevo_contrato();
            set_caller(account(2));
            let _ = contrato.registrar_usuario("ana".to_string(), Rol::Comprador);
            let id_vendedor = account(2);
            let res = contrato._modificar_rol(id_vendedor, Rol::Ambos);
            assert_eq!(res, Ok(()));
        }

        #[ink::test]
        fn modificar_rol_igual_falla() {
            let mut contrato = nuevo_contrato();
            set_caller(account(3));
            let _ = contrato.registrar_usuario("luis".to_string(), Rol::Vendedor);
            let id_vendedor = account(3);
            let res = contrato._modificar_rol(id_vendedor, Rol::Vendedor);
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
            set_caller(account(2));
            let id_vendedor = account(2);
            let res = contract._registrar_producto(
                id_vendedor,
                nombre.clone(),
                descripcion.clone(),
                categoria.clone(),
                stock,
            );
            assert_eq!(res, Ok(()));
            if let Some(producto_guardado) = contract.productos.get(&1) {
                assert_eq!(
                    producto_guardado.nombre,
                    Producto::normalizar_nombre_producto(&nombre)
                );
                assert_eq!(producto_guardado.descripcion, descripcion);
                assert_eq!(producto_guardado.categoria, categoria);
            } else {
                // Si no se guardó el producto, fallamos el test
                panic!("El producto no fue insertado en el catálogo");
            }
        }
        #[ink::test]
        fn registrar_producto_existente_sin_deposito_inicializa_ok() {
            let mut contract = contract_dummy();
            let nombre = String::from("Producto existente");
            let descripcion = String::from("Descripción");
            let categoria = Categoria::Tecnologia;
            let stock = 10;
            let id_vendedor = account(2);

            // Primero, registrar el producto con otro vendedor (account(3))
            set_caller(account(3));
            let _ = contract._registrar_producto(
                account(3),
                nombre.clone(),
                descripcion.clone(),
                categoria.clone(),
                stock,
            );

            // Ahora, registrar el mismo producto con id_vendedor (account(2)), que NO tiene depósito aún
            set_caller(id_vendedor);
            let res = contract._registrar_producto(
                id_vendedor,
                nombre.clone(),
                descripcion.clone(),
                categoria.clone(),
                stock,
            );
            assert_eq!(res, Ok(()));

            // Verifica que el depósito fue inicializado para el vendedor actual
            let id_producto = 1; // El primer producto registrado tiene id 1
            let deposito = contract.stock_general.get(&(id_vendedor, id_producto));
            assert!(deposito.is_some());
            assert_eq!(deposito.unwrap().stock, stock);
        }

        #[ink::test]
        fn buscar_producto_por_nombre_ok() {
            let mut contract = nuevo_contrato();
            // Insertar un producto en el mapping
            let producto = Producto::new(
                1,
                "celular".to_string(),
                "Un celular moderno".to_string(),
                Categoria::Tecnologia,
            );
            contract.productos.insert(1, &producto);
            contract.contador_productos = 1;

            // Buscar por el nombre (sin normalizar, la función lo normaliza)
            let res = contract.buscar_producto_por_nombre(&"Celular".to_string());
            assert_eq!(res, Ok(1));
        }
        #[ink::test]
        fn registrar_producto_existente_con_deposito_falla() {
            let mut contract = contract_dummy();
            let nombre = String::from("Producto existente");
            let descripcion = String::from("Descripción");
            let categoria = Categoria::Tecnologia;
            let stock = 10;
            let id_vendedor = account(2);

            // Registrar el producto con el vendedor (account(2)), lo que también inicializa el depósito
            set_caller(id_vendedor);
            let prod1 = contract._registrar_producto(
                id_vendedor,
                nombre.clone(),
                descripcion.clone(),
                categoria.clone(),
                stock,
            );
            assert_eq!(prod1, Ok(()));

            // Intentar registrar el mismo producto nuevamente con el mismo vendedor
            let res = contract._registrar_producto(
                id_vendedor,
                nombre.clone(),
                descripcion.clone(),
                categoria.clone(),
                stock,
            );
            assert_eq!(res, Err(ErrorMarketplace::ProductoYaPoseeDeposito));
        }
        #[ink::test]
        fn crear_publicacion_ok() {
            let mut contract = contract_dummy();
            let id_vendedor = account(2);
            let id_producto = 1;
            let stock_inicial = 10;
            let stock_a_vender = 5;
            let precio = 100;

            // Simula depósito inicial para el vendedor y producto
            let deposito = Deposito::new(id_producto, id_vendedor, stock_inicial);
            contract
                .stock_general
                .insert((id_vendedor, id_producto), &deposito);
            // Registramos el producto
            let _ = contract._registrar_producto(
                id_vendedor,
                "Producto de prueba".to_string(),
                "Descripción de prueba".to_string(),
                Categoria::Tecnologia,
                stock_inicial,
            );

            // Ejecuta la función
            let res = contract._crear_publicacion(
                "Producto de prueba".to_string(),
                id_vendedor,
                stock_a_vender,
                precio,
            );
            assert_eq!(res, Ok(()));

            // Verifica que la publicación fue insertada
            if let Some(publicacion) = contract.publicaciones.get(&1) {
                assert_eq!(publicacion.id_vendedor, id_vendedor);
                assert_eq!(publicacion.id_producto, id_producto);
                assert_eq!(publicacion.precio, precio);
                assert_eq!(publicacion.stock_a_vender, stock_a_vender);
            } else {
                panic!("La publicación no fue insertada en el mapping");
            }

            // Verifica que el stock fue actualizado
            if let Some(deposito_actualizado) =
                contract.stock_general.get(&(id_vendedor, id_producto))
            {
                assert_eq!(deposito_actualizado.stock, stock_inicial - stock_a_vender);
            } else {
                panic!("El depósito no fue actualizado");
            }
        }

        #[ink::test]
        fn crear_publicacion_usuario_no_existe() {
            let mut contract = contract_dummy();
            let id_vendedor = account(99); // No existe
            let nombre_producto = "Producto de prueba".to_string();
            let stock_a_vender = 5;
            let precio = 100;

            let res =
                contract._crear_publicacion(nombre_producto, id_vendedor, stock_a_vender, precio);
            assert_eq!(res, Err(ErrorMarketplace::UsuarioNoExiste));
        }

        #[ink::test]
        fn crear_publicacion_rol_invalido() {
            let mut contract = contract_dummy();
            let id_vendedor = account(1); // Comprador, no vendedor
            let nombre_producto = "Producto de prueba".to_string();
            let stock_a_vender = 5;
            let precio = 100;

            let res =
                contract._crear_publicacion(nombre_producto, id_vendedor, stock_a_vender, precio);
            assert_eq!(res, Err(ErrorMarketplace::RolInvalido));
        }

        #[ink::test]
        fn crear_publicacion_precio_invalido() {
            let mut contract = contract_dummy();
            let id_vendedor = account(2);
            let id_producto = 1;
            let nombre_producto = "Producto de prueba".to_string();
            let stock_a_vender = 5;
            let precio = 0; // Precio inválido

            let deposito = Deposito::new(id_producto, id_vendedor, 10);
            contract
                .stock_general
                .insert((id_vendedor, id_producto), &deposito);

            let res =
                contract._crear_publicacion(nombre_producto, id_vendedor, stock_a_vender, precio);
            assert_eq!(res, Err(ErrorMarketplace::PrecioInvalido));
        }

        #[ink::test]
        fn crear_publicacion_stock_insuficiente() {
            let mut contract = contract_dummy();
            let id_vendedor = account(2);
            let nombre_producto = "Producto de prueba".to_string();
            let descripcion = "Descripcion de prueba".to_string();
            let categoria = Categoria::Tecnologia;
            let stock_a_vender = 15; // Más que el stock disponible
            let precio = 100;

            //Registramos el producto y creamos un depósito con stock 10
            set_caller(id_vendedor);
            let _ = contract._registrar_producto(
                id_vendedor,
                nombre_producto.clone(),
                descripcion.clone(),
                categoria.clone(),
                10, // Stock inicial
            );

            let res =
                contract._crear_publicacion(nombre_producto, id_vendedor, stock_a_vender, precio);
            assert_eq!(res, Err(ErrorMarketplace::StockDepositoInsuficiente));
        }

        #[ink::test]
        fn crear_publicacion_falla_producto_no_existe() {
            let mut contrato = contract_dummy();
            let vendedor = account(2);

            let nombre_producto = "Producto X".to_string(); // No existe
            let stock_a_vender = 10;
            let precio = 1000;

            let result =
                contrato._crear_publicacion(nombre_producto, vendedor, stock_a_vender, precio);

            assert_eq!(result, Err(ErrorMarketplace::ProductoNoExiste));
        }

        /// Tests Marketplace Helpers
        #[ink::test]
        fn validacion_producto_nombre_espacios() {
            let nombre = String::from("   ");
            let res = Producto::validar_nombre_producto(&nombre);
            assert_eq!(res, Err(ErrorMarketplace::NombreInvalido));
        }
        #[ink::test]
        fn validar_descripcion_error_descripcion_invalida() {
            let descripcion_vacia = String::from("");
            let descripcion_espacios = String::from("   ");

            let res_vacia = Producto::validar_descripcion(&descripcion_vacia);
            let res_espacios = Producto::validar_descripcion(&descripcion_espacios);

            assert_eq!(res_vacia, Err(ErrorMarketplace::DescripcionInvalida));
            assert_eq!(res_espacios, Err(ErrorMarketplace::DescripcionInvalida));
        }
        #[ink::test]
        fn obtener_publicacion_no_existe() {
            let contract = nuevo_contrato();
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
            let mut contract = nuevo_contrato();
            set_caller(account(1));
            let _ = contract.registrar_usuario("comprador".to_string(), Rol::Comprador);
            let res = contract.verificar_rol_comprador(account(1));
            assert_eq!(res, Ok(()));
        }
        #[ink::test]
        fn verificar_rol_comprador_error_rol_invalido() {
            let mut contract = nuevo_contrato();
            set_caller(account(2));
            let _ = contract.registrar_usuario("vendedor".to_string(), Rol::Vendedor);
            let res = contract.verificar_rol_comprador(account(2));
            assert_eq!(res, Err(ErrorMarketplace::RolInvalido));
        }
        #[ink::test]
        fn verificar_usuario_existe_ok() {
            let mut contract = contract_dummy();
            let usuario = Usuario::new("test".to_string(), Rol::Comprador, account(4));
            set_caller(account(4));
            let registrar_ok =
                contract.registrar_usuario(usuario.username.clone(), usuario.rol.clone());
            assert_eq!(registrar_ok, Ok(()));
            let res = contract.verificar_usuario_existe(account(4));
            assert_eq!(res, Ok(usuario));
        }

        #[ink::test]
        fn verificar_usuario_existe_falla_si_no_existe() {
            let contract = nuevo_contrato();
            let res = contract.verificar_usuario_existe(account(4));
            assert_eq!(res, Err(ErrorMarketplace::UsuarioNoExiste));
        }
        #[ink::test]
        fn validar_stock_producto_error_stock_insuficiente() {
            let stock = 0u32;
            let res = Producto::validar_stock_producto(&stock);
            assert_eq!(res, Err(ErrorMarketplace::StockInsuficiente));
        }
        #[ink::test]
        fn obtener_nuevo_id_publicacion_ok() {
            let mut contract = nuevo_contrato();
            assert_eq!(contract.contador_publicacion, 0);

            let id1 = contract.obtener_nuevo_id_publicacion();
            assert_eq!(id1, Ok(1));
            assert_eq!(contract.contador_publicacion, 1);

            let id2 = contract.obtener_nuevo_id_publicacion();
            assert_eq!(id2, Ok(2));
            assert_eq!(contract.contador_publicacion, 2);
        }
        #[ink::test]
        fn verificar_id_producto_en_uso_error_id_producto_en_uso() {
            let mut contract = nuevo_contrato();
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
            let contract = nuevo_contrato();
            // No hay publicaciones, el id no está en uso
            let res = contract.verificar_id_publicacion_en_uso(1);
            assert_eq!(res, Ok(()));
        }

        #[ink::test]
        fn verificar_id_publicacion_en_uso_error_id_publicacion_en_uso() {
            let mut contract = nuevo_contrato();
            // Insertamos una publicación con id 1
            let publicacion = Publicacion::new(1, account(2), 1, 100, 10);
            contract.publicaciones.insert(1, &publicacion);

            // Ahora verificamos que el helper detecta el ID en uso
            let res = contract.verificar_id_publicacion_en_uso(1);
            assert_eq!(res, Err(ErrorMarketplace::IDPublicacionEnUso));
        }

        // Helper de insertar producto en catalogo
        #[ink::test]
        fn insertar_producto_en_catalogo_ok() {
            let mut contrato = nuevo_contrato();

            let producto = Producto::new(
                1,
                "Celular".to_string(),
                "Descripcion del celular".to_string(),
                Categoria::Tecnologia,
            );

            let res = contrato.insertar_producto_en_catalogo(producto.clone());
            assert!(res.is_ok());

            match contrato.productos.get(&1) {
                Some(producto_guardado) => {
                    assert_eq!(producto_guardado.id_producto, 1);
                    assert_eq!(producto_guardado.nombre, "Celular");
                    assert_eq!(producto_guardado.descripcion, "Descripcion del celular");
                    assert_eq!(producto_guardado.categoria, Categoria::Tecnologia);
                }
                None => panic!("El producto no fue insertado en el catálogo"),
            }
        }

        #[ink::test]
        fn insertar_producto_en_catalogo_producto_ya_existe() {
            let mut contrato = nuevo_contrato();
            let producto = Producto::new(
                1,
                "Comida".to_string(),
                "Descripcion de la comida".to_string(),
                Categoria::Alimentos,
            );

            assert!(contrato
                .insertar_producto_en_catalogo(producto.clone())
                .is_ok());
            let res = contrato.insertar_producto_en_catalogo(producto);
            assert_eq!(res, Err(ErrorMarketplace::IDProductoEnUso));
        }

        //Helper de actualizar stock de deposito
        #[ink::test]
        fn actualizar_stock_producto_ok() {
            let mut contrato = contract_dummy();
            let vendedor = account(2);
            let id_producto = 1;
            let stock_inicial = 100;
            let stock_a_vender = 30;

            let deposito = Deposito::new(id_producto, vendedor, stock_inicial);
            // Simular el estado inicial
            contrato
                .stock_general
                .insert((vendedor, id_producto), &deposito);

            // Ejecutar función
            let result = contrato.actualizar_stock_producto(vendedor, id_producto, stock_a_vender);
            // Verificar que fue exitoso
            assert_eq!(result, Ok(()));

            // Verificar que el nuevo stock es correcto
            match contrato.stock_general.get(&(vendedor, id_producto)) {
                Some(deposito) => {
                    assert_eq!(deposito.stock, (stock_inicial - stock_a_vender));
                }
                None => panic!("El depósito no fue actualizado"),
            }
        }

        #[ink::test]
        fn actualizar_stock_producto_stock_insuficiente() {
            let mut contrato = contract_dummy();
            let vendedor = account(2);
            let id_producto = 1;
            let stock_inicial = 10;
            let stock_a_vender = 20;

            let deposito = Deposito::new(id_producto, vendedor, stock_inicial);
            // Simular el estado inicial
            contrato
                .stock_general
                .insert((vendedor, id_producto), &deposito);

            let result = contrato.actualizar_stock_producto(vendedor, id_producto, stock_a_vender);

            assert_eq!(result, Err(ErrorMarketplace::StockDepositoInsuficiente));
        }

        #[ink::test]
        fn actualizar_stock_producto_producto_no_existe() {
            let mut contrato = contract_dummy();
            let vendedor = account(2);
            let id_producto = 45;
            let stock_a_vender = 5;

            // No se inserta el producto en el depósito

            let result = contrato.actualizar_stock_producto(vendedor, id_producto, stock_a_vender);

            assert_eq!(result, Err(ErrorMarketplace::DepositoNoEncontrado));
        }

        #[ink::test]
        fn actualizar_stock_producto_falla_validacion_de_stock() {
            let mut contrato = contract_dummy();
            let vendedor = account(2);
            let id_producto = 1;
            let stock_inicial = 50;
            let stock_a_vender = 999; //valor invalido para la logica

            let deposito = Deposito::new(id_producto, vendedor, stock_inicial);

            contrato
                .stock_general
                .insert((vendedor, id_producto), &deposito);

            let result = contrato.actualizar_stock_producto(vendedor, id_producto, stock_a_vender);

            assert_eq!(result, Err(ErrorMarketplace::StockDepositoInsuficiente));
        }

        //Helper de vendedor tiene deposito
        #[ink::test]
        fn vendedor_tiene_deposito_para_producto_ambos_v_y_f() {
            let mut contrato = nuevo_contrato();
            let vendedor = account(2);
            let id_producto = 1;

            // No se tiene deposito
            assert_eq!(
                contrato.vendedor_tiene_deposito_para_producto(vendedor, id_producto),
                false
            );

            let deposito = Deposito::new(id_producto, vendedor, 100);
            contrato
                .stock_general
                .insert((vendedor, id_producto), &deposito);

            // Ya se hizo el deposito
            assert_eq!(
                contrato.vendedor_tiene_deposito_para_producto(vendedor, id_producto),
                true
            );
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
            let mut contract = nuevo_contrato();
            let publicacion = Publicacion::new(1, account(2), 1, 100, 10);

            let res = contract.insertar_publicacion(publicacion.clone());
            assert_eq!(res, Ok(()));
            // Verifica que la publicación fue insertada
            let guardada = contract.publicaciones.get(&1);
            assert_eq!(guardada, Some(publicacion));
        }

        #[ink::test]
        fn insertar_publicacion_error_id_publicacion_en_uso() {
            let mut contract = nuevo_contrato();
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
            let mut contract = nuevo_contrato();
            // Insertamos un depósito para el vendedor y producto
            let deposito = Deposito::new(1, account(2), 15);
            contract.stock_general.insert((account(2), 1), &deposito);

            let res = contract.obtener_stock_deposito(account(2), 1);
            assert_eq!(res, Ok(15));
        }

        #[ink::test]
        fn obtener_stock_deposito_error_deposito_no_encontrado() {
            let contract = nuevo_contrato();
            // No existe el depósito para ese vendedor y producto
            let res = contract.obtener_stock_deposito(account(2), 1);
            assert_eq!(res, Err(ErrorMarketplace::DepositoNoEncontrado));
        }

        #[ink::test]
        fn modificar_stock_deposito_ok() {
            let mut contract = contract_dummy();
            // Registrar el usuario como vendedor
            set_caller(account(2));
            let id_vendedor = account(2);
            // Registramos producto y se inicializa el deposito
            let _ = contract._registrar_producto(
                id_vendedor,
                "Producto".to_string(),
                "Descripcion".to_string(),
                Categoria::Tecnologia,
                10,
            );
        

            // Modificar el stock
            let res = contract._modificar_stock_deposito(account(2), "Producto".to_string(), 60);
            assert_eq!(res, Ok(()));
        }

        #[ink::test]
        fn modificar_stock_deposito_error_producto_no_existe() {
            let mut contract = contract_dummy();
            // Registrar el usuario como vendedor
            set_caller(account(2));
            let _ = contract.registrar_usuario("vendedor".to_string(), Rol::Vendedor);

            // No existe el depósito para ese vendedor y producto
            let res = contract._modificar_stock_deposito(account(2), "Producto".to_string(), 25);
            assert_eq!(res, Err(ErrorMarketplace::ProductoNoExiste));
        }

        #[ink::test]
        fn validar_stock_deposito_ok() {
            let mut contract = nuevo_contrato();
            // Insertamos un depósito con stock suficiente
            let deposito = Deposito::new(1, account(2), 20);
            contract.stock_general.insert((account(2), 1), &deposito);

            let res = contract.validar_stock_deposito(account(2), 1, 10);
            assert_eq!(res, Ok(()));
        }

        #[ink::test]
        fn validar_stock_deposito_error_stock_deposito_insuficiente() {
            let mut contract = nuevo_contrato();
            // Insertamos un depósito con stock insuficiente
            let deposito = Deposito::new(1, account(2), 5);
            contract.stock_general.insert((account(2), 1), &deposito);

            let res = contract.validar_stock_deposito(account(2), 1, 10);
            assert_eq!(res, Err(ErrorMarketplace::StockDepositoInsuficiente));
        }
        #[ink::test]
        fn validar_precio_ok() {
            let precio = 100u128;
            let res = Publicacion::validar_precio(&precio);
            assert_eq!(res, Ok(()));
        }

        #[ink::test]
        fn validar_precio_error_precio_invalido() {
            let precio = 0u128;
            let res = Publicacion::validar_precio(&precio);
            assert_eq!(res, Err(ErrorMarketplace::PrecioInvalido));
        }

        //Test de crear orden
        #[ink::test]
        fn crear_orden_usuario_no_existe() {
            let mut contrato = contract_dummy();
            let comprador = account(99); //no registrado en el contrato

            let result = contrato._crear_orden(comprador, 0, 1, 100);
            assert_eq!(result, Err(ErrorMarketplace::UsuarioNoExiste));
        }

        #[ink::test]
        fn crear_orden_rol_incorrecto() {
            let mut contrato = contract_dummy();
            let rol = account(2);

            let result = contrato._crear_orden(rol, 0, 1, 100);

            assert_eq!(result, Err(ErrorMarketplace::RolInvalido));
        }

        #[ink::test]
        fn crear_orden_publicacion_no_existe() {
            let mut contrato = contract_dummy();
            let comprador = account(1);

            let result = contrato._crear_orden(comprador, 0, 1, 100);
            assert_eq!(result, Err(ErrorMarketplace::PublicacionNoExiste));
        }

        #[ink::test]
        fn crear_orden_stock_insuficiente() {
            let mut contrato = contract_dummy();
            let comprador = account(1);
            let vendedor = account(2);

            let producto = Producto::new(
                1,
                "Libro".to_string(),
                "Descripcion del libro".to_string(),
                Categoria::Otros,
            );

            let publicacion = Publicacion::new(0, vendedor, producto.id_producto, 200, 1);

            contrato.publicaciones.insert(0, &publicacion);

            let result = contrato._crear_orden(comprador, 0, 2, 200);
            assert_eq!(result, Err(ErrorMarketplace::StockInsuficiente));
        }

        #[ink::test]
        fn crear_orden_overflow_en_precio_total() {
            let mut contrato = contract_dummy();
            let comprador = account(1);
            let vendedor = account(2);

            let producto = Producto::new(
                1,
                "Comida".to_string(),
                "Descripcion de la comida".to_string(),
                Categoria::Alimentos,
            );

            let publicacion = Publicacion::new(
                0,
                vendedor,
                producto.id_producto,
                u128::MAX, // Precio máximo para provocar overflow
                100,
            );

            contrato.publicaciones.insert(0, &publicacion);

            let result = contrato._crear_orden(comprador, 0, 2, u128::MAX);
            assert_eq!(result, Err(ErrorMarketplace::Overflow));
        }

        #[ink::test]
        fn crear_orden_monto_insuficiente() {
            let mut contrato = contract_dummy();
            let comprador = account(1);
            let vendedor = account(2);

            let producto = Producto::new(
                1,
                "Mesa".to_string(),
                "Descripcion de la mesa".to_string(),
                Categoria::Hogar,
            );

            let publicacion = Publicacion::new(0, vendedor, producto.id_producto, 200, 5);

            contrato.publicaciones.insert(0, &publicacion);

            let result = contrato._crear_orden(comprador, 0, 2, 150); // Se espera 400
            assert_eq!(result, Err(ErrorMarketplace::MontoInsuficiente));
        }

        #[ink::test]
        fn crear_orden_valida() {
            let mut contrato = contract_dummy();
            let comprador = account(1);
            let vendedor = account(2);

            let producto = Producto::new(
                1,
                "Ropa".to_string(),
                "Descripcion de la ropa".to_string(),
                Categoria::Indumentaria,
            );

            let publicacion = Publicacion::new(0, vendedor, producto.id_producto, 200, 10);

            contrato.publicaciones.insert(0, &publicacion);

            let result = contrato._crear_orden(comprador, 0, 2, 500);
            assert!(result.is_ok());

            match contrato.ordenes.get(&0) {
                Some(orden) => {
                    assert_eq!(orden.id, 0);
                    assert_eq!(orden.comprador, comprador);
                    assert_eq!(orden.vendedor, vendedor);
                    assert_eq!(orden.total, 400); // 200 * 2
                }
                None => panic!("La orden no fue creada correctamente"),
            }
        }

        #[ink::test]
        fn test_overflow_contador_ordenes() {
            let mut contrato = contract_dummy();
            let comprador = account(1);
            let vendedor = account(2);

            contrato.contador_ordenes = u32::MAX;

            let producto = Producto::new(
                1,
                "Ropa".to_string(),
                "Descripcion de la ropa".to_string(),
                Categoria::Indumentaria,
            );

            let publicacion = Publicacion::new(0, vendedor, producto.id_producto, 200, 10);

            contrato.publicaciones.insert(0, &publicacion);
            let result = contrato._crear_orden(comprador, 0, 2, 500);

            assert_eq!(result, Err(ErrorMarketplace::Overflow));
        }
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
}
*/
