#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod reportes_view {
    use ink::env::call::FromAccountId;
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::prelude::collections::BTreeMap;
    use market_place::market_place::MarketPlaceRef;
    use market_place::market_place::Categoria;
    use market_place::market_place::Orden;
    use market_place::market_place::Producto;
    use market_place::market_place::EstadoOrden;

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
                if let Some((_, puntaje_total)) = self.marketplace.get_reputacion_vendedor(vendedor) {
                    vendedores_reputacion.push((vendedor, puntaje_total));
                }
            }

            // Ordenar por puntaje descendente
            vendedores_reputacion.sort_by(|a, b| b.1.cmp(&a.1));

            // Retornar top 5
            vendedores_reputacion.into_iter().take(5).collect()
        }

        /// Consultar top 5 compradores con mejor reputación.
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
                if let Some((_, puntaje_total)) = self.marketplace.get_reputacion_comprador(comprador) {
                    compradores_reputacion.push((comprador, puntaje_total));
                }
            }

            compradores_reputacion.sort_by(|a, b| b.1.cmp(&a.1));
            compradores_reputacion.into_iter().take(5).collect()
        }

        /// Ver productos más vendidos.
        #[ink(message)]
        pub fn productos_mas_vendidos(&self) -> Vec<(String, u32)> {
            let cantidad_ordenes = self.marketplace.get_cantidad_ordenes();
            let mut ventas_por_producto: BTreeMap<u32, u32> = BTreeMap::new();

            for i in 0..cantidad_ordenes {
                if let Some(orden) = self.marketplace.get_orden(i) {
                    // Consideramos solo órdenes completadas (Recibido) o también Enviado?
                    // El requerimiento dice "productos más vendidos", usualmente implica ventas concretadas.
                    // Pero si la orden está creada, ya se "vendió" en teoría, aunque no se haya entregado.
                    // Voy a contar todas las que no estén canceladas.
                    if orden.estado != EstadoOrden::Cancelada {
                         let count = ventas_por_producto.entry(orden.id_producto).or_insert(0);
                         *count += orden.cant_producto as u32;
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
        /// Retorna Vec<(Categoria, TotalVentas, CalificacionPromedio)>
        /// CalificacionPromedio: Asumiremos promedio de reputación de vendedores en esa categoría ponderado?
        /// O simplemente promedio de reputación de los vendedores que vendieron en esa categoría.
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
                                     stat.1 += orden.total;
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
                 let mut suma_reputacion = 0;
                 let mut count_vendedores = 0;
                 
                 for vendedor in vendedores {
                     if let Some((cant_calif, puntaje_tot)) = self.marketplace.get_reputacion_vendedor(vendedor) {
                         if cant_calif > 0 {
                             suma_reputacion += puntaje_tot / cant_calif; // Promedio entero
                             count_vendedores += 1;
                         }
                     }
                 }
                 
                 let promedio_cat = if count_vendedores > 0 {
                     (suma_reputacion / count_vendedores) as u8
                 } else {
                     0
                 };
                 
                 resultado.push((cat, total_ventas, promedio_cat));
             }
             
             resultado
        }

        /// Cantidad de órdenes por usuario.
        #[ink(message)]
        pub fn cantidad_ordenes_usuario(&self, usuario: AccountId) -> u32 {
            let cantidad_ordenes = self.marketplace.get_cantidad_ordenes();
            let mut count = 0;
            for i in 0..cantidad_ordenes {
                if let Some(orden) = self.marketplace.get_orden(i) {
                    if orden.comprador == usuario || orden.vendedor == usuario {
                        count += 1;
                    }
                }
            }
            count
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn new_works() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            // En unit tests, la llamada a contratos externos fallará si se intenta ejecutar.
            // Solo probamos la instanciación aquí.
            let _reportes = ReportesView::new(accounts.alice);
        }
    }
}
