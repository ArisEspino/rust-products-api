# Rust Products API

API REST para la gestión de productos, desarrollada con **Rust**, **Actix-Web**, **PostgreSQL** y **SeaORM**.

## Descripción

Este proyecto consiste en una API REST que permite administrar productos mediante operaciones CRUD.  
Con esta aplicación se pueden crear, listar, obtener, actualizar y eliminar productos.

El proyecto fue desarrollado utilizando Rust como lenguaje principal, Actix-Web para la parte del servidor, PostgreSQL como base de datos y SeaORM para trabajar la persistencia de datos y las migraciones.

## Tecnologías utilizadas

- Rust
- Actix-Web
- PostgreSQL
- SeaORM
- sea-orm-cli

## Arquitectura

El proyecto sigue una estructura simple y ordenada (estructura por capa).

## Estructura del proyecto

```bash
src/
├── config.rs
├── state.rs
├── main.rs
├── entities/
├── errors/
├── handlers/
├── models/
└── routes/

migration/
```
## Clonar repocitorio
```bash
git clone https://github.com/ArisEspino/rust-products-api.git
```
## Acceder a la carpeta
```bash
cd rust-products-api
```
## Ejecución del proyecto
```bash
cargo run
```
