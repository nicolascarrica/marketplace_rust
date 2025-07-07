#![cfg_attr(not(feature = "std"), no_std, no_main)]
mod types;
#[ink::contract]
mod market_place {
    use ink::{
        storage::Mapping,
        xcm::{v2::Junction::AccountId32, v3::Junction::AccountId32, v4::Junction::AccountKey20},
    };
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct MarketPlace {
        usuarios: Mapping<AccountId, Usuario>,  //aca seria id?
        productos: Mapping<AccountId, Balance>, //no deberia ser un vec donde solo guarde los productos, porque el producto ya tiene su stock
        ordenes: Mapping<AccountId, Balance>,   //lo mismo
        publicaciones: Mapping<AccountId, Publicacion>,
        productos_por_usuario: Mapping<AccountId, Producto>,
        contador_ordenes: u64,
        contador_productos: u64,
        //aca iria lo de reputacion, creo
    }

    pub struct Usuario {
        username: String,
        rol: Rol,
        id: AccountId,
        calificaciones: Vec<Calificacion>, //creo que seria solo uno, porque ya el usuario sabe quien es, por ende no tiene sentido tener 2 vec o no??
        verificacion: bool,
    }

    pub struct Calificacion {
        id: AccountId, //o usuario para verificar que solo califico una vez y no mas
        puntaje: u8,
        id_orden: u64,
    }

    pub struct Producto {
        id: AccountId,
        nombre: String,
        descripcion: String,
        precio: u32,
        stock: u32,
        categoria: Categoria,
    }

    pub struct Publicacion {
        id: AccountId,                     //quien lo publica
        productos: Mapping<u32, Producto>, //la cantidad de productos y el producto, tenia algo mas??
    }

    pub struct Orden {
        id: u64,
        productos: Mapping<u32, Producto>, //lo dejamos asi??
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

    impl MarketPlace {
        #[ink(constructor)]
        ///aca modificariamos si pasamos a vec o no
        pub fn new() -> Self {
            Self {
                usuarios: Mapping::default(),
                productos: Mapping::default(),
                publicaciones: Mapping::default(),
                productos_por_usuario: Mapping::default(),
                ordenes: Mapping::default(),
                contador_ordenes: 0,
                contador_productos: 0,
            }
        }

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
        pub fn modificar_rol(&mut self, nuevo_rol: Rol) -> bool { //o que no devuelva nada, todavia no se
            let caller = self.env().caller();
            let mut usuario = match self.usuarios.get(caller) {
                Some(u) => u,
                None => return false,
            };

            usuario.rol = nuevo_rol;
            self.usuarios.insert(caller, &usuario); //se debe actualizar en el map
            true
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
