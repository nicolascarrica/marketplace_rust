#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod reportes_view {
    use ink::env::call::FromAccountId;
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::prelude::collections::BTreeMap;
    use marketplace::market_place::MarketPlaceRef;
    use marketplace::market_place::Categoria;
    use marketplace::market_place::Orden;
    use marketplace::market_place::Producto;
    use marketplace::market_place::EstadoOrden;

    #[ink(storage)]
    pub struct ReportesView {
        marketplace: MarketPlaceRef,
    }

    impl ReportesView {
        #[ink(constructor)]
        pub fn new(marketplace_address: AccountId) -> Self {
            let marketplace = MarketPlaceRef::from_account_id(marketplace_address);
            Self { marketplace }
        }

        /// Consultar top 5 vendedores con mejor reputación.
        ///
        /// #Retorna
        /// -Vec<(AccountId, PuntajeTotal)>: 
        ///     -AccountId es la cuenta del vendedor,
        ///     -PuntajeTotal es la suma de todas las calificaciones recibidas.
        #[ink(message)]
        pub fn top_5_vendedores(&self) -> Vec<(AccountId, u32)> {
            // Como no podemos iterar sobre los mappings del otro contrato directamente,
            // y no hay una lista de todos los vendedores expuesta,
            // necesitamos una forma de obtener los vendedores.
            // Una opción es iterar sobre todas las órdenes y extraer los vendedores únicos.
            // Esto es ineficiente pero dado las limitaciones de acceso es una opción.
            // O mejor, iterar sobre las órdenes y acumular reputación si pudiéramos acceder a ella.
            // Pero MarketPlace tiene `get_reputacion_vendedor`.
            // Así que iteramos órdenes -> obtenemos vendedores -> obtenemos su reputación.
            
            let cantidad_ordenes = self.marketplace.get_cantidad_ordenes();
            let mut vendedores_set = Vec::new();

            for i in 0..cantidad_ordenes {
                if let Some(orden) = self.marketplace.get_orden(i) {
                    if !vendedores_set.contains(&orden.vendedor) {
                        vendedores_set.push(orden.vendedor);
                    }
                }
            }

            let mut vendedores_reputacion: Vec<(AccountId, u32)> = Vec::new();
            for vendedor in vendedores_set {
                if let Some((puntaje_total, _)) = self.marketplace.get_reputacion_vendedor(vendedor) {
                    vendedores_reputacion.push((vendedor, puntaje_total));
                }
            }

            // Ordenar por puntaje descendente
            vendedores_reputacion.sort_by(|a, b| b.1.cmp(&a.1));

            // Retornar top 5
            vendedores_reputacion.into_iter().take(5).collect()
        }

        /// Consultar top 5 compradores con mejor reputación.
        /// 
        /// #Retorna
        /// -Vec<(AccountId, PuntajeTotal)>:   
        ///   -AccountId es la cuenta del comprador,
        ///   -PuntajeTotal es la suma de todas las calificaciones recibidas.
        #[ink(message)]
        pub fn top_5_compradores(&self) -> Vec<(AccountId, u32)> {
            let cantidad_ordenes = self.marketplace.get_cantidad_ordenes();
            let mut compradores_set = Vec::new();

            for i in 0..cantidad_ordenes {
                if let Some(orden) = self.marketplace.get_orden(i) {
                    if !compradores_set.contains(&orden.comprador) {
                        compradores_set.push(orden.comprador);
                    }
                }
            }

            let mut compradores_reputacion: Vec<(AccountId, u32)> = Vec::new();
            for comprador in compradores_set {
                if let Some((puntaje_total, _)) = self.marketplace.get_reputacion_comprador(comprador) {
                    compradores_reputacion.push((comprador, puntaje_total));
                }
            }

            compradores_reputacion.sort_by(|a, b| b.1.cmp(&a.1));
            compradores_reputacion.into_iter().take(5).collect()
        }

        /// Ver productos más vendidos. Orden
        /// 
        /// #Retorna
        /// -Vec<(NombreProducto, CantidadVendida)>
        ///    -NombreProducto es el nombre del producto,
        ///    -CantidadVendida es la suma de las cantidades vendidas en órdenes completadas.
        #[ink(message)]
        pub fn productos_mas_vendidos(&self) -> Vec<(String, u32)> {
            let cantidad_ordenes = self.marketplace.get_cantidad_ordenes();
            let mut ventas_por_producto: BTreeMap<u32, u32> = BTreeMap::new();

            for i in 0..cantidad_ordenes {
                if let Some(orden) = self.marketplace.get_orden(i) {
                    // Consideramos solo órdenes completadas (Recibido)
                    if orden.estado == EstadoOrden::Recibido {
                         let count = ventas_por_producto.entry(orden.id_producto).or_insert(0);
                            *count = count
                                .checked_add(orden.cant_producto as u32)
                                .unwrap_or(*count);

                    }
                }
            }

            let mut ranking: Vec<(String, u32)> = Vec::new();
            for (id_producto, cantidad) in ventas_por_producto {
                if let Some(producto) = self.marketplace.get_producto(id_producto) {
                    ranking.push((producto.nombre, cantidad));
                }
            }

            ranking.sort_by(|a, b| b.1.cmp(&a.1));
            ranking
        }

        /// Estadísticas por categoría: total de ventas, calificación promedio.
        /// 
        /// #Retorna
        /// -Vec<(Categoria, TotalVentas, CalificacionPromedio)>
        ///   -Categoria es la categoría del producto,
        ///   -TotalVentas es la suma de los totales de órdenes (u128) en esa categoría,
        ///   -CalificacionPromedio es la reputación promedio (u8) de los vendedores en esa categoría.
        /// Simplificación: Promedio de reputación de vendedores únicos que tienen ventas en esa categoría.
        #[ink(message)]
        pub fn estadisticas_por_categoria(&self) -> Vec<(Categoria, u128, u8)> {
             let cantidad_ordenes = self.marketplace.get_cantidad_ordenes();
             // Map: Categoria -> (TotalVentas, Set<Vendedores>)
             // Como no tenemos Set, usaremos Vec y dedup.
             
             // Estructura temporal: Categoria -> (MontoTotal, Vec<Vendedor>)
             // Categoria no implementa Copy/Clone trivialmente para usar como key en BTreeMap a veces,
             // pero es un enum simple, debería.
             // Pero BTreeMap necesita Ord. Categoria deriva PartialEq, Eq. Necesita PartialOrd, Ord.
             // Asumiremos que podemos iterar y agrupar manualmente o usar un vector de acumuladores.
             
             // Dado que son pocas categorías (5), podemos usar un vector fijo o mapear manualmente.
             
             let mut stats: Vec<(Categoria, u128, Vec<AccountId>)> = Vec::new();
             // Inicializar con las categorías conocidas si quisiéramos, o dinámicamente.
             
             for i in 0..cantidad_ordenes {
                 if let Some(orden) = self.marketplace.get_orden(i) {
                     if orden.estado != EstadoOrden::Cancelada {
                         if let Some(producto) = self.marketplace.get_producto(orden.id_producto) {
                             let categoria = producto.categoria;
                             
                             // Buscar si ya existe la categoría en stats
                             let mut found = false;
                             for stat in &mut stats {
                                if stat.0 == categoria {
                                    stat.1 = stat.1
                                        .checked_add(orden.total)
                                        .unwrap_or(stat.1);

                                    if !stat.2.contains(&orden.vendedor) {
                                        stat.2.push(orden.vendedor);
                                    }
                                    found = true;
                                    break;
                                }

                             }
                             
                             if !found {
                                 let mut vendedores = Vec::new();
                                 vendedores.push(orden.vendedor);
                                 stats.push((categoria, orden.total, vendedores));
                             }
                         }
                     }
                 }
             }
             
             let mut resultado: Vec<(Categoria, u128, u8)> = Vec::new();
             
             for (cat, total_ventas, vendedores) in stats {
                let mut suma_reputacion: u32 = 0;
                let mut count_vendedores: u32 = 0;

                for vendedor in vendedores {
                    if let Some((cant_calif, puntaje_tot)) =
                        self.marketplace.get_reputacion_vendedor(vendedor)
                    {
                        if cant_calif > 0 {
                            // promedio individual del vendedor
                            let promedio_vendedor = puntaje_tot
                                .checked_div(cant_calif)
                                .unwrap_or(0);

                            // suma segura
                            suma_reputacion = suma_reputacion
                                .checked_add(promedio_vendedor)
                                .unwrap_or(suma_reputacion);

                            count_vendedores = count_vendedores
                                .checked_add(1)
                                .unwrap_or(count_vendedores);
                        }
                    }
                }

                let promedio_cat = suma_reputacion
                    .checked_div(count_vendedores)
                    .unwrap_or(0) as u8;
                 
                 resultado.push((cat, total_ventas, promedio_cat));
             }
             
             resultado
        }

        /// Cantidad de órdenes por usuario.
        /// 
        /// #Parametro
        /// -usuario: AccountId del usuario (comprador o vendedor).
        /// 
        /// #Retorna
        /// -u32: Cantidad de órdenes donde el usuario es comprador o vendedor.
        #[ink(message)]
        pub fn cantidad_ordenes_usuario(&self, usuario: AccountId) -> u32 {
            let cantidad_ordenes = self.marketplace.get_cantidad_ordenes();
            let mut count: u32 = 0;
            for i in 0..cantidad_ordenes {
                if let Some(orden) = self.marketplace.get_orden(i) {
                    if orden.comprador == usuario || orden.vendedor == usuario {
                        count = count.checked_add(1).unwrap_or(count);
                    }
                }
            }
            count
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::test;
        use ink::ToAccountId;

        fn account(id: u8) -> AccountId {
            AccountId::from([id; 32])
        }

        fn set_caller(caller: AccountId) {
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(caller);
        }
                #[ink::test]
        fn new_works() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            // En unit tests, la llamada a contratos externos fallará si se intenta ejecutar.
            // Solo probamos la instanciación aquí.
            let _reportes = ReportesView::new(accounts.alice);
        }

/*        #[ink::test]
        fn test_new_works_y_guarda_address() {
            let marketplace_addr = account(1);

            let reportes = ReportesView::new(marketplace_addr);

            assert_eq!(
                reportes.marketplace.to_account_id(),
                marketplace_addr
            );
        }

        #[ink::test]
        fn test_top_5_vendedores_sin_ordenes() {
            let reportes = ReportesView::new(account(1));

            let resultado = reportes.top_5_vendedores();

            assert!(resultado.is_empty());
        }

        #[ink::test]
        fn test_top_5_compradores_sin_ordenes() {
            let reportes = ReportesView::new(account(1));

            let resultado = reportes.top_5_compradores();

            assert!(resultado.is_empty());
        }

        #[ink::test]
        fn test_productos_mas_vendidos_sin_ordenes() {
            let reportes = ReportesView::new(account(1));

            let resultado = reportes.productos_mas_vendidos();

            assert!(resultado.is_empty());
        }

        #[ink::test]
        fn test_estadisticas_por_categoria_sin_datos() {
            let reportes = ReportesView::new(account(1));

            let resultado = reportes.estadisticas_por_categoria();

            assert!(resultado.is_empty());
        }

        #[ink::test]
        fn test_cantidad_ordenes_usuario_sin_ordenes() {
            let reportes = ReportesView::new(account(1));

            let cantidad = reportes.cantidad_ordenes_usuario(account(2));

            assert_eq!(cantidad, 0);
        }

        #[ink::test]
        fn test_llamadas_no_panican() {
            let reportes = ReportesView::new(account(1));

            reportes.top_5_vendedores();
            reportes.top_5_compradores();
            reportes.productos_mas_vendidos();
            reportes.estadisticas_por_categoria();
            reportes.cantidad_ordenes_usuario(account(2));
        } */
    }
}
