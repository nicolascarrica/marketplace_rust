# 🚀 Proyecto Marketplace en ink! (Substrate)

Este proyecto contiene contratos inteligentes escritos en Rust usando [`ink!`](https://use.ink) para un marketplace descentralizado. A continuación te mostramos los pasos para configurar el entorno y compilar los contratos.

---

## 📦 Requisitos Previos

- [Rust](https://www.rust-lang.org/tools/install)
- [`rustup`](https://rustup.rs/)
- [`cargo-contract`](https://use.ink/getting-started)

---

## ⚙️ Pasos para el Deploy

### 0. Clonar el repositorio

```sh
git clone [SSH]
cd tu_repo
```

### 1. Instalar toolchain nightly exacta

```sh
rustup toolchain install nightly
rustup update
rustup component add rust-src --toolchain nightly-2024-05-20
rustup target add wasm32-unknown-unknown --toolchain nightly-2024-05-20
```

Crear un archivo rust-toolchain.toml en la raíz del proyecto con el siguiente contenido:

```toml
[toolchain]
channel = "nightly"
```

### 2. Instalar cargo-contract

```sh
cargo install cargo-contract --version 4.1.3 --locked
```

### 3. Verificar versiones

```sh
cargo contract --version
rustc +nightly-2024-05-20 --version
```

Deberías ver algo así:

```sh
cargo-contract-contract 4.1.3-unknown-x86_64-pc-windows-msvc
rustc 1.80.0-nightly (d84b90375 2024-05-19)
```

### 4. Compilar los contratos

Entrar a cada carpeta de contrato y ejecutar:

```sh
cargo +nightly contract build --release --optimization-passes 0
```

Esto generará los archivos .contract, .wasm y .json necesarios para desplegar el contrato en una red compatible.

#### Pasos para crear Nodo Local (Opcional)

1. **Instalar herramientas base (si no lo hiciste antes):**  
   Sigue la guía oficial de Polkadot para instalar el SDK y verificar tu instalación:  
   [https://docs.polkadot.com/develop/parachains/install-polkadot-sdk/#verifying-installation](https://docs.polkadot.com/develop/parachains/install-polkadot-sdk/#verifying-installation)

2. **Descargar el nodo local para contratos:**  
   Ve al repositorio oficial de [`substrate-contracts-node`](https://github.com/paritytech/substrate-contracts-node)  
   Descarga los binarios de la [versión 0.42 release](https://github.com/paritytech/substrate-contracts-node/releases/tag/v0.42.0) para tu sistema operativo.

3. **Ejecutar el nodo local:**  
   Descomprime el archivo descargado y, desde esa carpeta, ejecuta:
   ```sh
   ./substrate-contracts-node --dev
   ```
   Esto levantará un nodo local en `ws://127.0.0.1:9944` listo para pruebas.

4. **Subir tu contrato usando Polkadot.js Apps:**  
   - Ve a [https://polkadot.js.org/apps/#/explorer](https://polkadot.js.org/apps/#/explorer)
   - Haz switch a "Development Local Node" en la esquina superior izquierda.
   - Ve a la sección **Developer > Contracts**.
   - Haz clic en "Upload & deploy code".
   - Sube el archivo `.contract` generado por:
     ```sh
     cargo contract build --release
     ```
   - Completa el proceso de despliegue y prueba tu contrato.

---

> **Nota:** Si tienes dudas sobre la instalación o ejecución del nodo, revisa la documentación oficial de [substrate-contracts-node](https://github.com/paritytech/substrate-contracts-node).