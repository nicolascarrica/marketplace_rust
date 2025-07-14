#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod market_place {
    use ink::prelude::string::String;
    use ink::storage::Mapping;

    /// Enums
    
    /// Representa los roles posibles que puede tener un usuario dentro del marketplace.
    #[derive(
        Debug,
        Clone,
        Copy,
        PartialEq,
        Eq,
        ink::scale::Encode,
        ink::scale::Decode,
    )]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout, scale_info::TypeInfo))]
    pub enum Rol {
        Comprador,
        Vendedor,
        Ambos,
    }

    /// Categorías posibles para los productos.
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        ink::scale::Encode,
        ink::scale::Decode,
    )]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout, scale_info::TypeInfo))]
    pub enum Categoria {
        Tecnologia,
        Indumentaria,
        Hogar,
        Alimentos,
        Otros,
    }

    /// Representa los estados posibles de una orden de compra.
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        ink::scale::Encode,
        ink::scale::Decode,
    )]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout, scale_info::TypeInfo))]
    pub enum EstadoOrden {
        Pendiente,
        Enviado,
        Recibido,
        Cancelada,
    }


    /// Errores del Marketplace
    /// Define los posibles errores que pueden ocurrir en el marketplace.
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[derive(Debug, PartialEq)]

    pub enum ErrorMarketplace {
        UsuarioYaRegistrado,
        UsuarioNoExiste,
        ProductoNoExiste,
        PublicacionNoExiste,
        StockInsuficiente,
        RolInvalido,
        OrdenNoExiste,
        PrecioInvalido,
        NombreInvalido,
        SaldoInsuficiente,
        TransferenciaFallida,
    }

    /// Implementación del trait `core::fmt::Display` para el enum `ErrorMarketplace`.
    /// Permite mostrar mensajes de error legibles para el usuario.
    /// Requiere el feature "std" para poder usar `core::fmt::Display`.
    #[cfg(feature = "std")]
    impl core::fmt::Display for ErrorMarketplace {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let msg = match self {
                ErrorMarketplace::UsuarioYaRegistrado => "El usuario ya está registrado",
                ErrorMarketplace::UsuarioNoExiste => "El usuario no existe",
                ErrorMarketplace::ProductoNoExiste => "El producto no existe",
                ErrorMarketplace::PublicacionNoExiste => "La publicación no existe",
                ErrorMarketplace::StockInsuficiente => "No hay suficiente stock",
                ErrorMarketplace::RolInvalido => "Rol no autorizado para esta acción",
                ErrorMarketplace::OrdenNoExiste => "La orden no existe",
                ErrorMarketplace::PrecioInvalido => "Precio o cantidad no válidos",
                ErrorMarketplace::NombreInvalido => "Nombre del producto inválido",
                ErrorMarketplace::SaldoInsuficiente => "Saldo insuficiente para la compra",
                ErrorMarketplace::TransferenciaFallida => "No se pudo transferir al vendedor",
            };
            write!(f, "{msg}")
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
    )]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout, scale_info::TypeInfo))]
    pub struct Usuario {
        username: String,
        rol: Rol,
        id: AccountId,
        verificacion: bool,
    }
    impl Usuario {
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
    )]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout, scale_info::TypeInfo))]
    pub struct Producto {
        id: u32,
        nombre: String,
        descripcion: String,
        precio: u128,
        categoria: Categoria,
    }
    impl Producto {
        pub fn new(
            id: u32,
            nombre: String,
            descripcion: String,
            precio: u128,
            categoria: Categoria,
        ) -> Self {
            Self {
                id,
                nombre,
                descripcion,
                precio,
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
        
    )]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout, scale_info::TypeInfo))]
    pub struct Publicacion {
        id: u32,
        producto_id: u32,
        vendedor: AccountId,
        stock_publicacion: u32,
    }

    impl Publicacion {
        pub fn new(id: u32, vendedor: AccountId, producto_id: u32, stock_publicacion: u32) -> Self {
            Self {
                id,
                producto_id,
                vendedor,
                stock_publicacion,
            }
        }
    }

    #[derive(
        Debug,
        PartialEq,
        Eq,
        ink::scale::Encode,
        ink::scale::Decode,
    )]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout, scale_info::TypeInfo))]
    pub struct Orden {
        id: u32,
        comprador: AccountId,
        vendedor: AccountId,
        producto_id: u32,
        cantidad: u32,
        total: u128,
        estado: EstadoOrden,
    }

    impl Orden {
        pub fn new(id: u32, comprador: AccountId, vendedor: AccountId, producto_id: u32, cantidad: u32, total: u128) -> Self {
            Self {
                id,
                comprador,
                vendedor,
                producto_id,
                cantidad,
                total,
                estado: EstadoOrden::Pendiente,
            }
        }
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct MarketPlace {
        usuarios: Mapping<AccountId, Usuario>,
        productos: Mapping<u32, Producto>,
        stock: Mapping<(u32, AccountId), u32>,
        publicaciones: Mapping<u32, Publicacion>,
        ordenes: Mapping<u32, Orden>,
        contador_productos: u32,
        contador_publicaciones: u32,
        contador_ordenes: u32,
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
                stock: Mapping::default(),
                publicaciones: Mapping::default(),
                ordenes: Mapping::default(),
                contador_productos: 0,
                contador_publicaciones: 0,
                contador_ordenes: 0,
            }
        }

        #[ink(message)]
        pub fn registrar_usuario(&mut self, username: String, rol: Rol) -> Result<(), ErrorMarketplace> {
            let caller = self.env().caller();
            if self.usuarios.contains(&caller) {
                return Err(ErrorMarketplace::UsuarioYaRegistrado);
            }
            self.usuarios.insert(caller, &Usuario::new(username, rol, caller));
            Ok(())
        }

        #[ink(message)]
        pub fn agregar_producto(
            &mut self,
            nombre: String,
            descripcion: String,
            precio: u128,
            categoria: Categoria,
            cantidad: u32,
        ) -> Result<u32, ErrorMarketplace> {
            let caller = self.env().caller();
            let usuario = match self.usuarios.get(caller) {
                Some(user) => user,
                None => return Err(ErrorMarketplace::UsuarioNoExiste),
            };

            if usuario.rol == Rol::Comprador {
                return Err(ErrorMarketplace::RolInvalido);
            }
            if nombre.trim().is_empty() || precio == 0 || cantidad == 0 {
                return Err(ErrorMarketplace::PrecioInvalido);
            }

            self.contador_productos = self.contador_productos.saturating_add(1);
            let producto = Producto::new(self.contador_productos, nombre, descripcion, precio, categoria);
            self.productos.insert(producto.id, &producto);
            self.stock.insert((producto.id, caller), &cantidad);
            Ok(producto.id)
        }


        #[ink(message)]
        pub fn publicar_producto(&mut self, producto_id: u32, cantidad: u32) -> Result<u32, ErrorMarketplace> {
            let caller = self.env().caller();

            // Verificar existencia del usuario
            let _usuario = self.usuarios.get(caller).ok_or(ErrorMarketplace::UsuarioNoExiste)?;

            // Verificar existencia del producto
            let _producto = self.productos.get(producto_id).ok_or(ErrorMarketplace::ProductoNoExiste)?;

            // Verificar stock disponible en depósito
            let clave = (producto_id, caller);
            let stock_opcion = self.stock.get(clave);
            let stock_deposito = match stock_opcion {
                Some(valor) => valor,
                None => return Err(ErrorMarketplace::StockInsuficiente),
            };

            if cantidad == 0 || stock_deposito < cantidad {
                return Err(ErrorMarketplace::StockInsuficiente);
            }

            // Descontar del depósito
            let nuevo_stock = stock_deposito.saturating_sub(cantidad);
            self.stock.insert(clave, &nuevo_stock);

            // Crear publicación
            self.contador_publicaciones = self.contador_publicaciones.saturating_add(1);
            let publicacion = Publicacion {
                id: self.contador_publicaciones,
                producto_id,
                vendedor: caller,
                stock_publicacion: cantidad,
            };
            self.publicaciones.insert(publicacion.id, &publicacion);
        
            Ok(publicacion.id)
        }

        #[ink(message, payable)]
        pub fn crear_orden(&mut self, publicacion_id: u32, cantidad: u32) -> Result<(), ErrorMarketplace> {
            let comprador = self.env().caller();
            let pago = self.env().transferred_value();

            let _comprador_data = self.usuarios.get(&comprador).ok_or(ErrorMarketplace::UsuarioNoExiste)?;
            let mut publicacion = self.publicaciones.get(&publicacion_id).ok_or(ErrorMarketplace::PublicacionNoExiste)?;
            let vendedor = publicacion.vendedor;
            let _vendedor_data = self.usuarios.get(&vendedor).ok_or(ErrorMarketplace::UsuarioNoExiste)?;

            if cantidad == 0 || publicacion.stock_publicacion < cantidad {
                return Err(ErrorMarketplace::StockInsuficiente);
            }

            let producto = self.productos.get(&publicacion.producto_id).ok_or(ErrorMarketplace::ProductoNoExiste)?;

            let total = producto.precio.checked_mul(cantidad as u128).ok_or(ErrorMarketplace::PrecioInvalido)?;
            if pago < total {
                return Err(ErrorMarketplace::SaldoInsuficiente);
            }

            // Descontamos stock de la publicación
            publicacion.stock_publicacion = publicacion.stock_publicacion.saturating_sub(cantidad);

            self.publicaciones.insert(publicacion_id, &publicacion);

            self.contador_ordenes = self.contador_ordenes.saturating_add(1);

            let orden = Orden::new(self.contador_ordenes, comprador, vendedor, producto.id, cantidad, total);
            self.ordenes.insert(self.contador_ordenes, &orden);

            // Transferir dinero al vendedor
            self.env().transfer(vendedor, total).map_err(|_| ErrorMarketplace::TransferenciaFallida)?;

            Ok(())
        }
    }
}

/// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
/// module and test functions are marked with a `#[test]` attribute.
/// The below code is technically just normal Rust code.

#[cfg(test)]
mod tests {
    use crate::market_place::{Categoria, ErrorMarketplace, MarketPlace, Rol};

    use super::*;
    use ink::env::test;
    use ink::primitives::AccountId;

    /// Función helper para crear un AccountId desde un array de bytes
    fn account_id_from_bytes(bytes: [u8; 32]) -> AccountId {
        AccountId::from(bytes)
    }

    /// Función helper para configurar el entorno de pruebas
    fn setup_test_env() -> (MarketPlace, AccountId, AccountId) {
        let mut marketplace = MarketPlace::new();
        let alice = account_id_from_bytes([1; 32]);
        let bob = account_id_from_bytes([2; 32]);
        
        // Configurar Alice como caller por defecto
        test::set_caller::<ink::env::DefaultEnvironment>(alice);
        
        (marketplace, alice, bob)
    }

    // ===== TESTS DE REGISTRO DE USUARIOS =====

    #[ink::test]
    fn test_registrar_usuario_exitoso() {
        let (mut marketplace, alice, _) = setup_test_env();
        
        let resultado = marketplace.registrar_usuario("Alice".to_string(), Rol::Vendedor);
        assert!(resultado.is_ok());
    }

    #[ink::test]
    fn test_registrar_usuario_ya_registrado() {
        let (mut marketplace, alice, _) = setup_test_env();
        
        // Primer registro exitoso
        let resultado1 = marketplace.registrar_usuario("Alice".to_string(), Rol::Vendedor);
        assert!(resultado1.is_ok());
        
        // Segundo registro debe fallar
        let resultado2 = marketplace.registrar_usuario("Alice2".to_string(), Rol::Comprador);
        assert!(resultado2.is_err());
        if let Err(error) = resultado2 {
            assert_eq!(error, ErrorMarketplace::UsuarioYaRegistrado);
        }
    }

    #[ink::test]
    fn test_registrar_usuario_diferentes_roles() {
        let (mut marketplace, alice, bob) = setup_test_env();
        
        // Registrar Alice como vendedor
        let resultado1 = marketplace.registrar_usuario("Alice".to_string(), Rol::Vendedor);
        assert!(resultado1.is_ok());
        
        // Cambiar caller a Bob y registrar como comprador
        test::set_caller::<ink::env::DefaultEnvironment>(bob);
        let resultado2 = marketplace.registrar_usuario("Bob".to_string(), Rol::Comprador);
        assert!(resultado2.is_ok());
        
        // Registrar Charlie como ambos
        let charlie = account_id_from_bytes([3; 32]);
        test::set_caller::<ink::env::DefaultEnvironment>(charlie);
        let resultado3 = marketplace.registrar_usuario("Charlie".to_string(), Rol::Ambos);
        assert!(resultado3.is_ok());
    }

    // ===== TESTS DE AGREGAR PRODUCTO =====

    #[ink::test]
    fn test_agregar_producto_exitoso() {
        let (mut marketplace, alice, _) = setup_test_env();
        
        // Registrar usuario como vendedor
        let reg_result = marketplace.registrar_usuario("Alice".to_string(), Rol::Vendedor);
        assert!(reg_result.is_ok());
        
        let resultado = marketplace.agregar_producto(
            "Laptop".to_string(),
            "Laptop gaming".to_string(),
            100000,
            Categoria::Tecnologia,
            10
        );
        
        assert!(resultado.is_ok());
        if let Ok(producto_id) = resultado {
            assert_eq!(producto_id, 1); // Primer producto debe tener ID 1
        }
    }

    #[ink::test]
    fn test_agregar_producto_usuario_no_existe() {
        let (mut marketplace, _, _) = setup_test_env();
        
        // No registrar usuario
        let resultado = marketplace.agregar_producto(
            "Laptop".to_string(),
            "Laptop gaming".to_string(),
            100000,
            Categoria::Tecnologia,
            10
        );
        
        assert!(resultado.is_err());
        if let Err(error) = resultado {
            assert_eq!(error, ErrorMarketplace::UsuarioNoExiste);
        }
    }

    #[ink::test]
    fn test_agregar_producto_rol_invalido() {
        let (mut marketplace, alice, _) = setup_test_env();
        
        // Registrar usuario como comprador
        let reg_result = marketplace.registrar_usuario("Alice".to_string(), Rol::Comprador);
        assert!(reg_result.is_ok());
        
        let resultado = marketplace.agregar_producto(
            "Laptop".to_string(),
            "Laptop gaming".to_string(),
            100000,
            Categoria::Tecnologia,
            10
        );
        
        assert!(resultado.is_err());
        if let Err(error) = resultado {
            assert_eq!(error, ErrorMarketplace::RolInvalido);
        }
    }

    #[ink::test]
    fn test_agregar_producto_nombre_vacio() {
        let (mut marketplace, alice, _) = setup_test_env();
        
        let reg_result = marketplace.registrar_usuario("Alice".to_string(), Rol::Vendedor);
        assert!(reg_result.is_ok());
        
        let resultado = marketplace.agregar_producto(
            "".to_string(),
            "Descripción".to_string(),
            100000,
            Categoria::Tecnologia,
            10
        );
        
        assert!(resultado.is_err());
        if let Err(error) = resultado {
            assert_eq!(error, ErrorMarketplace::PrecioInvalido);
        }
    }

    #[ink::test]
    fn test_agregar_producto_precio_cero() {
        let (mut marketplace, alice, _) = setup_test_env();
        
        let reg_result = marketplace.registrar_usuario("Alice".to_string(), Rol::Vendedor);
        assert!(reg_result.is_ok());
        
        let resultado = marketplace.agregar_producto(
            "Laptop".to_string(),
            "Laptop gaming".to_string(),
            0,
            Categoria::Tecnologia,
            10
        );
        
        assert!(resultado.is_err());
        if let Err(error) = resultado {
            assert_eq!(error, ErrorMarketplace::PrecioInvalido);
        }
    }

    #[ink::test]
    fn test_agregar_producto_cantidad_cero() {
        let (mut marketplace, alice, _) = setup_test_env();
        
        let reg_result = marketplace.registrar_usuario("Alice".to_string(), Rol::Vendedor);
        assert!(reg_result.is_ok());
        
        let resultado = marketplace.agregar_producto(
            "Laptop".to_string(),
            "Laptop gaming".to_string(),
            100000,
            Categoria::Tecnologia,
            0
        );
        
        assert!(resultado.is_err());
        if let Err(error) = resultado {
            assert_eq!(error, ErrorMarketplace::PrecioInvalido);
        }
    }

    #[ink::test]
    fn test_agregar_producto_nombre_solo_espacios() {
        let (mut marketplace, alice, _) = setup_test_env();
        
        let reg_result = marketplace.registrar_usuario("Alice".to_string(), Rol::Vendedor);
        assert!(reg_result.is_ok());
        
        let resultado = marketplace.agregar_producto(
            "   ".to_string(),
            "Descripción".to_string(),
            100000,
            Categoria::Tecnologia,
            10
        );
        
        assert!(resultado.is_err());
        if let Err(error) = resultado {
            assert_eq!(error, ErrorMarketplace::PrecioInvalido);
        }
    }

    #[ink::test]
    fn test_agregar_multiples_productos() {
        let (mut marketplace, alice, _) = setup_test_env();
        
        let reg_result = marketplace.registrar_usuario("Alice".to_string(), Rol::Vendedor);
        assert!(reg_result.is_ok());
        
        let resultado1 = marketplace.agregar_producto(
            "Laptop".to_string(),
            "Laptop gaming".to_string(),
            100000,
            Categoria::Tecnologia,
            10
        );
        assert!(resultado1.is_ok());
        if let Ok(producto_id) = resultado1 {
            assert_eq!(producto_id, 1);
        }
        
        let resultado2 = marketplace.agregar_producto(
            "Mouse".to_string(),
            "Mouse gaming".to_string(),
            5000,
            Categoria::Tecnologia,
            20
        );
        assert!(resultado2.is_ok());
        if let Ok(producto_id) = resultado2 {
            assert_eq!(producto_id, 2);
        }
    }

    #[ink::test]
    fn test_agregar_producto_rol_ambos() {
        let (mut marketplace, alice, _) = setup_test_env();
        
        let reg_result = marketplace.registrar_usuario("Alice".to_string(), Rol::Ambos);
        assert!(reg_result.is_ok());
        
        let resultado = marketplace.agregar_producto(
            "Laptop".to_string(),
            "Laptop gaming".to_string(),
            100000,
            Categoria::Tecnologia,
            10
        );
        
        assert!(resultado.is_ok());
        if let Ok(producto_id) = resultado {
            assert_eq!(producto_id, 1);
        }
    }

    // ===== TESTS DE PUBLICAR PRODUCTO =====

    #[ink::test]
    fn test_publicar_producto_exitoso() {
        let (mut marketplace, alice, _) = setup_test_env();
        
        // Registrar usuario y agregar producto
        let reg_result = marketplace.registrar_usuario("Alice".to_string(), Rol::Vendedor);
        assert!(reg_result.is_ok());
        
        let resultado_producto = marketplace.agregar_producto(
            "Laptop".to_string(),
            "Laptop gaming".to_string(),
            100000,
            Categoria::Tecnologia,
            10
        );
        assert!(resultado_producto.is_ok());
        
        if let Ok(producto_id) = resultado_producto {
            // Publicar producto
            let resultado = marketplace.publicar_producto(producto_id, 5);
            assert!(resultado.is_ok());
            if let Ok(publicacion_id) = resultado {
                assert_eq!(publicacion_id, 1); // Primera publicación debe tener ID 1
            }
        }
    }

    #[ink::test]
    fn test_publicar_producto_usuario_no_existe() {
        let (mut marketplace, _, _) = setup_test_env();
        
        let resultado = marketplace.publicar_producto(1, 5);
        assert!(resultado.is_err());
        if let Err(error) = resultado {
            assert_eq!(error, ErrorMarketplace::UsuarioNoExiste);
        }
    }

    #[ink::test]
    fn test_publicar_producto_no_existe() {
        let (mut marketplace, alice, _) = setup_test_env();
        
        let reg_result = marketplace.registrar_usuario("Alice".to_string(), Rol::Vendedor);
        assert!(reg_result.is_ok());
        
        let resultado = marketplace.publicar_producto(999, 5);
        assert!(resultado.is_err());
        if let Err(error) = resultado {
            assert_eq!(error, ErrorMarketplace::ProductoNoExiste);
        }
    }

    #[ink::test]
    fn test_publicar_producto_stock_insuficiente() {
        let (mut marketplace, alice, _) = setup_test_env();
        
        let reg_result = marketplace.registrar_usuario("Alice".to_string(), Rol::Vendedor);
        assert!(reg_result.is_ok());
        
        let resultado_producto = marketplace.agregar_producto(
            "Laptop".to_string(),
            "Laptop gaming".to_string(),
            100000,
            Categoria::Tecnologia,
            10
        );
        assert!(resultado_producto.is_ok());
        
        if let Ok(producto_id) = resultado_producto {
            // Intentar publicar más stock del disponible
            let resultado = marketplace.publicar_producto(producto_id, 15);
            assert!(resultado.is_err());
            if let Err(error) = resultado {
                assert_eq!(error, ErrorMarketplace::StockInsuficiente);
            }
        }
    }

    #[ink::test]
    fn test_publicar_producto_cantidad_cero() {
        let (mut marketplace, alice, _) = setup_test_env();
        
        let reg_result = marketplace.registrar_usuario("Alice".to_string(), Rol::Vendedor);
        assert!(reg_result.is_ok());
        
        let resultado_producto = marketplace.agregar_producto(
            "Laptop".to_string(),
            "Laptop gaming".to_string(),
            100000,
            Categoria::Tecnologia,
            10
        );
        assert!(resultado_producto.is_ok());
        
        if let Ok(producto_id) = resultado_producto {
            let resultado = marketplace.publicar_producto(producto_id, 0);
            assert!(resultado.is_err());
            if let Err(error) = resultado {
                assert_eq!(error, ErrorMarketplace::StockInsuficiente);
            }
        }
    }

    #[ink::test]
    fn test_publicar_producto_multiples_publicaciones() {
        let (mut marketplace, alice, _) = setup_test_env();
        
        let reg_result = marketplace.registrar_usuario("Alice".to_string(), Rol::Vendedor);
        assert!(reg_result.is_ok());
        
        let resultado_producto = marketplace.agregar_producto(
            "Laptop".to_string(),
            "Laptop gaming".to_string(),
            100000,
            Categoria::Tecnologia,
            10
        );
        assert!(resultado_producto.is_ok());
        
        if let Ok(producto_id) = resultado_producto {
            // Primera publicación
            let resultado1 = marketplace.publicar_producto(producto_id, 3);
            assert!(resultado1.is_ok());
            if let Ok(pub_id) = resultado1 {
                assert_eq!(pub_id, 1);
            }
            
            // Segunda publicación
            let resultado2 = marketplace.publicar_producto(producto_id, 4);
            assert!(resultado2.is_ok());
            if let Ok(pub_id) = resultado2 {
                assert_eq!(pub_id, 2);
            }
            
            // Tercera publicación debería fallar (3 + 4 + 5 = 12 > 10)
            let resultado3 = marketplace.publicar_producto(producto_id, 5);
            assert!(resultado3.is_err());
            if let Err(error) = resultado3 {
                assert_eq!(error, ErrorMarketplace::StockInsuficiente);
            }
        }
    }

    // ===== TESTS DE CREAR ORDEN =====

    #[ink::test]
    fn test_crear_orden_exitoso() {
        let (mut marketplace, alice, bob) = setup_test_env();
        
        // Registrar vendedor y agregar producto
        let reg_result = marketplace.registrar_usuario("Alice".to_string(), Rol::Vendedor);
        assert!(reg_result.is_ok());
        
        let resultado_producto = marketplace.agregar_producto(
            "Laptop".to_string(),
            "Laptop gaming".to_string(),
            100000,
            Categoria::Tecnologia,
            10
        );
        assert!(resultado_producto.is_ok());
        
        if let Ok(producto_id) = resultado_producto {
            // Publicar producto
            let resultado_pub = marketplace.publicar_producto(producto_id, 5);
            assert!(resultado_pub.is_ok());
            
            if let Ok(publicacion_id) = resultado_pub {
                // Cambiar a comprador y registrar
                test::set_caller::<ink::env::DefaultEnvironment>(bob);
                let reg_comp = marketplace.registrar_usuario("Bob".to_string(), Rol::Comprador);
                assert!(reg_comp.is_ok());
                
                // Configurar pago suficiente
                test::set_value_transferred::<ink::env::DefaultEnvironment>(200000);
                
                // Crear orden
                let resultado = marketplace.crear_orden(publicacion_id, 2);
                assert!(resultado.is_ok());
            }
        }
    }

    #[ink::test]
    fn test_crear_orden_comprador_no_existe() {
        let (mut marketplace, alice, bob) = setup_test_env();
        
        // Registrar vendedor y agregar producto
        let reg_result = marketplace.registrar_usuario("Alice".to_string(), Rol::Vendedor);
        assert!(reg_result.is_ok());
        
        let resultado_producto = marketplace.agregar_producto(
            "Laptop".to_string(),
            "Laptop gaming".to_string(),
            100000,
            Categoria::Tecnologia,
            10
        );
        assert!(resultado_producto.is_ok());
        
        if let Ok(producto_id) = resultado_producto {
            let resultado_pub = marketplace.publicar_producto(producto_id, 5);
            assert!(resultado_pub.is_ok());
            
            if let Ok(publicacion_id) = resultado_pub {
                // Cambiar a comprador NO registrado
                test::set_caller::<ink::env::DefaultEnvironment>(bob);
                test::set_value_transferred::<ink::env::DefaultEnvironment>(200000);
                
                let resultado = marketplace.crear_orden(publicacion_id, 2);
                assert!(resultado.is_err());
                if let Err(error) = resultado {
                    assert_eq!(error, ErrorMarketplace::UsuarioNoExiste);
                }
            }
        }
    }

    #[ink::test]
    fn test_crear_orden_publicacion_no_existe() {
        let (mut marketplace, alice, bob) = setup_test_env();
        
        // Registrar usuarios
        let reg_vendedor = marketplace.registrar_usuario("Alice".to_string(), Rol::Vendedor);
        assert!(reg_vendedor.is_ok());
        
        test::set_caller::<ink::env::DefaultEnvironment>(bob);
        let reg_comprador = marketplace.registrar_usuario("Bob".to_string(), Rol::Comprador);
        assert!(reg_comprador.is_ok());
        
        test::set_value_transferred::<ink::env::DefaultEnvironment>(200000);
        
        let resultado = marketplace.crear_orden(999, 2);
        assert!(resultado.is_err());
        if let Err(error) = resultado {
            assert_eq!(error, ErrorMarketplace::PublicacionNoExiste);
        }
    }

    #[ink::test]
    fn test_crear_orden_stock_insuficiente() {
        let (mut marketplace, alice, bob) = setup_test_env();
        
        // Setup completo
        let reg_vendedor = marketplace.registrar_usuario("Alice".to_string(), Rol::Vendedor);
        assert!(reg_vendedor.is_ok());
        
        let resultado_producto = marketplace.agregar_producto(
            "Laptop".to_string(),
            "Laptop gaming".to_string(),
            100000,
            Categoria::Tecnologia,
            10
        );
        assert!(resultado_producto.is_ok());
        
        if let Ok(producto_id) = resultado_producto {
            let resultado_pub = marketplace.publicar_producto(producto_id, 3);
            assert!(resultado_pub.is_ok());
            
            if let Ok(publicacion_id) = resultado_pub {
                test::set_caller::<ink::env::DefaultEnvironment>(bob);
                let reg_comprador = marketplace.registrar_usuario("Bob".to_string(), Rol::Comprador);
                assert!(reg_comprador.is_ok());
                
                test::set_value_transferred::<ink::env::DefaultEnvironment>(500000);
                
                // Intentar comprar más stock del disponible
                let resultado = marketplace.crear_orden(publicacion_id, 5);
                assert!(resultado.is_err());
                if let Err(error) = resultado {
                    assert_eq!(error, ErrorMarketplace::StockInsuficiente);
                }
            }
        }
    }

    #[ink::test]
    fn test_crear_orden_cantidad_cero() {
        let (mut marketplace, alice, bob) = setup_test_env();
        
        // Setup completo
        let reg_vendedor = marketplace.registrar_usuario("Alice".to_string(), Rol::Vendedor);
        assert!(reg_vendedor.is_ok());
        
        let resultado_producto = marketplace.agregar_producto(
            "Laptop".to_string(),
            "Laptop gaming".to_string(),
            100000,
            Categoria::Tecnologia,
            10
        );
        assert!(resultado_producto.is_ok());
        
        if let Ok(producto_id) = resultado_producto {
            let resultado_pub = marketplace.publicar_producto(producto_id, 5);
            assert!(resultado_pub.is_ok());
            
            if let Ok(publicacion_id) = resultado_pub {
                test::set_caller::<ink::env::DefaultEnvironment>(bob);
                let reg_comprador = marketplace.registrar_usuario("Bob".to_string(), Rol::Comprador);
                assert!(reg_comprador.is_ok());
                
                test::set_value_transferred::<ink::env::DefaultEnvironment>(200000);
                
                let resultado = marketplace.crear_orden(publicacion_id, 0);
                assert!(resultado.is_err());
                if let Err(error) = resultado {
                    assert_eq!(error, ErrorMarketplace::StockInsuficiente);
                }
            }
        }
    }

    #[ink::test]
    fn test_crear_orden_saldo_insuficiente() {
        let (mut marketplace, alice, bob) = setup_test_env();
        
        // Setup completo
        let reg_vendedor = marketplace.registrar_usuario("Alice".to_string(), Rol::Vendedor);
        assert!(reg_vendedor.is_ok());
        
        let resultado_producto = marketplace.agregar_producto(
            "Laptop".to_string(),
            "Laptop gaming".to_string(),
            100000,
            Categoria::Tecnologia,
            10
        );
        assert!(resultado_producto.is_ok());
        
        if let Ok(producto_id) = resultado_producto {
            let resultado_pub = marketplace.publicar_producto(producto_id, 5);
            assert!(resultado_pub.is_ok());
            
            if let Ok(publicacion_id) = resultado_pub {
                test::set_caller::<ink::env::DefaultEnvironment>(bob);
                let reg_comprador = marketplace.registrar_usuario("Bob".to_string(), Rol::Comprador);
                assert!(reg_comprador.is_ok());
                
                test::set_value_transferred::<ink::env::DefaultEnvironment>(150000); // Insuficiente para 2 productos
                
                let resultado = marketplace.crear_orden(publicacion_id, 2);
                assert!(resultado.is_err());
                if let Err(error) = resultado {
                    assert_eq!(error, ErrorMarketplace::SaldoInsuficiente);
                }
            }
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
} */
