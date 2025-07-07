#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod market_place {
    use ink::{storage::Mapping, xcm::{v2::Junction::AccountId32, v3::Junction::AccountId32}};
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct MarketPlace {
        /// Stores a single `bool` value on the storage.
        value: bool,
        usuarios: Mapping<AccountId, Usuario>, //aca seria id?
        productos:Mapping<AccountId, Balance>, //no deberia ser un vec donde solo guarde los productos, porque el producto ya tiene su stock
        ordenes:Mapping<AccountId, Balance>, //lo mismo
        publicaciones:Mapping<AccountId, Publicacion>,
        productos_por_usuario:Mapping<AccountId, Producto>,
        contador_ordenes: u64,
        contador_productos:u64,
        //aca iria lo de reputacion, creo
    }

    pub struct Usuario{
        username:String,
        rol:Rol,
        id:AccountId32,
        calificaciones: Vec<Calificacion>, //creo que seria solo uno, porque ya el usuario sabe quien es, por ende no tiene sentido tener 2 vec o no??
        verificacion:bool,
    }

    pub struct Calificacion{
        id:AccountId32, //o usuario para verificar que solo califico una vez y no mas 
        puntaje:u8,
        id_orden:u64,
    }

    pub struct Producto{
        id:AccountId32,
        nombre:String,
        descripcion:String,
        precio:u32,
        stock:u32,
        categoria:Categoria,

    }

    pub struct Publicacion{
        id:AccountId32, //quien lo publica
        productos:Mapping<u32, Producto>, //la cantidad de productos y el producto, tenia algo mas?? 

    }

    pub struct Orden{
        id:u64,
        productos: Mapping<u32, Producto>, //lo dejamos asi??
        estado:Estado,
    }

    pub enum Rol {
        Comprador,
        Vendedor,
        Ambos
    }

    pub enum Categoria {
        Tecnologia,
        Indumentaria,
        Hogar,
        Alimentos,
        Otros
    }

    pub enum Estado {
        Pendiente,
        Enviado,
        Recibido,
        Cancelada
    }


    impl MarketPlace {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            //Self { value: init_value }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    //TEST PRIVADOS (DE LAS FUNCIONES AUXILIARES POR ASI DECIRLO)
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
            let mut MarketPlace = MarketPlace::new(false);
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
    //TEST DE LAS FUNCIONES QUE DESPLEGAMOS EN EL CONTRATO COMO REGISTRAR USUARIO
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
