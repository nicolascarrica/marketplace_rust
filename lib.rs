#![cfg_attr(not(feature = "std"), no_std, no_main)]
mod types;
#[ink::contract]

mod market_place {
    use crate::types::orden::Orden;
    use crate::types::producto::Producto;
    use crate::types::usuario::Usuario;
    use crate::types::errores::ErrorMarketplace;
    use crate::types::enums::Rol;
   use ink::{
        storage::Mapping,
        xcm::{v2::Junction::AccountId32, v3::Junction::AccountId32, v4::Junction::AccountKey20},
    };

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct MarketPlace {
        usuarios: Mapping<AccountId, Usuario>,  
        productos: Mapping<u32, Producto>, 
        ordenes: Mapping<u32, Orden>,  
        publicaciones: Mapping<AccountId, Publicacion>,
        productos_por_usuario: Mapping<AccountId, Producto>,
        contador_ordenes: u32,
        contador_productos: u32,
        //aca iria lo de reputacion, creo
    }

    pub struct Usuario {
        username: String,
        rol: Rol,
        id: AccountId,
        calificaciones: Vec<Calificacion>, 
        verificacion: bool,
    }


    pub struct Calificacion {
        id: AccountId, //o usuario para verificar que solo califico una vez y no mas
        puntaje: u8,
        id_orden: u32,
    }

    pub struct Producto {
        id: u32, //id del producto
        nombre: String,
        descripcion: String,
        precio: u32,
        stock: u32,
        categoria: Categoria,
    }


    pub struct Publicacion {
         id_publicacion: u32,       // ID de la publicación
         id_vendedor: AccountId,    // ID del vendedor
         producto: Producto, //lo podriamos poner asi, directamente
         estado: EstadoPublicacion, // Estado de la publicación
         fecha_publicacion: u64,    // Fecha de publicación (timestamp, as UNIX timestamp)
    }

    pub struct Orden {
        id: u32, //id de la orden
        productos: Mapping<u32, Producto>, //(cantidad, producto)
        estado: Estado,
    }

    pub enum Rol {
        Comprador,
        Vendedor,
        Ambos,
    }

    pub enum Categoria {
        Tecnologia,
        Indumentaria,
        Hogar,
        Alimentos,
        Otros,
    }

    pub enum Estado {
        Pendiente,
        Enviado,
        Recibido,
        Cancelada,
    }

    pub enum EstadoPublicacion {
        Activa,
        Pausada,
        Eliminada,
        Agotada,
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


    pub struct Orden {
        id: u32,
        productos: Mapping<u32, Producto>, //lo dejamos asi??
        estado: Estado,
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
                 productos_por_usuario: Mapping::default(),
                 contador_ordenes: 0,
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

        #[ink(message)]
        pub fn modificar_rol(&mut self, nuevo_rol: Rol) -> bool {
            //o que no devuelva nada, todavia no se
            let caller = self.env().caller();
            if self.usuarios.contains(caller) {
                return Err(ErrorMarketplace::UsuarioYaRegistrado);
            }
            let nuevo = Usuario::new(username, rol);
            self.usuarios.insert(caller, &nuevo);
            Ok(())
        }
        
        //

        //Helper verificar usuario exista
        fn verificar_usuario(&self, id: AccountId) -> Result<Usuario, String> {
            self.usuarios
                .get(&id)
                .ok_or_else(|| "Usuario no encontrado".to_string())
        }
        //Helper verificar que el usuario tenga el rol correcto
        fn verificar_rol(&self, id: AccountId) -> Result<(), String> {
            let usuario = self.verificar_usuario(id)?;
            if usuario.rol == Rol::Ambos || usuario.rol == Rol::Vendedor {
                Ok(())
            } else {
                Err(format!(
                    "El usuario no tiene el rol requerido: {:?}",
                    usuario.rol
                ))
            }
        }

        //Helper para validar que el producto tenga un nombre, precio y stock
        fn validacion_producto(
            &self,
            nombre: &String,
            precio: &u128,
            stock: &u32,
        ) -> Result<(), String> {
            if *stock <= 0 {
                return Err("El producto no tiene stock".to_string());
            }
            if *precio <= 0 {
                return Err("El precio del producto debe ser mayor a 0".to_string());
            }
            if nombre.is_empty() || nombre.trim().is_empty() {
                return Err("El nombre del producto no puede estar vacío".to_string());
            }
            Ok(())
        }
        //Helper para obtener una publicacion por id
        fn obtener_publicacion(&self, id_publicacion: u32) -> Result<Publicacion, String> {
            self.publicaciones
                .get(&id_publicacion)
                .ok_or_else(|| "Publicación no encontrada".to_string())
        }
        //Helper para verificar que el usuario es el owner de la publicacion
        fn verificar_owner_publicacion(
            &self,
            id_publicacion: u32,
            id_vendedor: AccountId,
        ) -> Result<(), String> {
            let publicacion = self.obtener_publicacion(id_publicacion)?;
            if publicacion.id_vendedor != id_vendedor {
                return Err("No tienes permisos para modificar esta publicación".to_string());
            }
            Ok(())
        }

        //Publicar producto
        #[ink(message)]
        pub fn publicar_producto(
            &mut self,
            nombre: String,
            descripcion: String,
            precio: u128,
            stock: u32,
            categoria: Categoria,
        ) -> Result<(), String> {
            let id_vendedor = self.env().caller();
            self.verificar_usuario(id_vendedor)?;
            self.verificar_rol(id_vendedor)?;
            self.validacion_producto(&nombre, &precio, &stock)?;

            self.contador_productos += 1;
            let id_publicacion = self.contador_productos;
            let nueva_publicacion = Publicacion::new(
                id_publicacion,
                id_vendedor,
                nombre,
                descripcion,
                stock,
                categoria,
                precio,
                EstadoPublicacion::Activa,
                self.env().block_timestamp(),
            );

            //Guardamos la publicación en el mapping
            self.publicaciones
                .insert(id_publicacion, &nueva_publicacion);
            Ok(())
        }

        /// Actualizar stock de una publicación
        #[ink(message)]
        pub fn actualizar_stock(
            &mut self,
            id_publicacion: u32,
            nuevo_stock: u32,
        ) -> Result<(), String> {
            let caller = self.env().caller();

            //Verificamos que la publicación exista
            let mut publicacion = self.obtener_publicacion(id_publicacion)?;
            //Verificamos que el caller es el dueño de la publicación
            self.verificar_owner_publicacion(id_publicacion, caller)?;
            //Validamos el nuevo stock
            publicacion.stock = nuevo_stock;
            self.publicaciones.insert(id_publicacion, &publicacion);

            Ok(())
        }
        /// Pausar/Activar una publicación
        #[ink(message)]
        pub fn cambiar_estado_publicacion(
            &mut self,
            id_publicacion: u32,
            nuevo_estado: EstadoPublicacion,
        ) -> Result<(), String> {
            let caller = self.env().caller();

            //Verificamos que la publicación exista
            let mut publicacion = self.obtener_publicacion(id_publicacion)?;
            //Verificamos que el caller es el dueño de la publicación
            self.verificar_owner_publicacion(id_publicacion, caller)?;
            //Actualizamos el estado de la publicación
            publicacion.estado = nuevo_estado;
            self.publicaciones.insert(id_publicacion, &publicacion);

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

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let MarketPlace = MarketPlace::default();
            assert_eq!(MarketPlace.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut MarketPlace = MarketPlace::new();
            assert_eq!(MarketPlace.get(), false);
            MarketPlace.flip();
            assert_eq!(MarketPlace.get(), true);
        }
    }

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
}
