// Solo es la estructura
// Lo que voy a hacer es guardar la información de los clientes en un archivo de texto cifrado y leerlo cada vez que ejecute el programa

use serde::{Serialize, Deserialize};
use bincode;
use std::fs;

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
/// Struct que contiene los campos de lis clientes
/// con datos tipo String y u32
pub struct Clientes { // cédula, Nombre, Sexo, Saldo
    pub cedula: u32,
    pub nombre: String,
    pub sexo: String,
    pub saldo: u32,
}

/// Crea un objeto cliente con los datos por defecto
///
/// Los clientes con datos por defecto no serán listados
/// por las funciones de filtrado o listado
fn crear_cliente_default() -> Clientes {
    Clientes {
        cedula: 0000000000,
        nombre: String::from("xxxx"),
        sexo: String::from("xxxx"),
        saldo: 00000000 }
}

pub fn crear_lista_clientes() -> [Clientes; 30]{
    // Regresa la lista ya llena con los 30 clientes


    let lista_virgen = [
        crear_cliente_default(),
        crear_cliente_default(),
        crear_cliente_default(),
        crear_cliente_default(),
        crear_cliente_default(),
        crear_cliente_default(),
        crear_cliente_default(),
        crear_cliente_default(),
        crear_cliente_default(),
        crear_cliente_default(),
        crear_cliente_default(),
        crear_cliente_default(),
        crear_cliente_default(),
        crear_cliente_default(),
        crear_cliente_default(),
        crear_cliente_default(),
        crear_cliente_default(),
        crear_cliente_default(),
        crear_cliente_default(),
        crear_cliente_default(),
        crear_cliente_default(),
        crear_cliente_default(),
        crear_cliente_default(),
        crear_cliente_default(),
        crear_cliente_default(),
        crear_cliente_default(),
        crear_cliente_default(),
        crear_cliente_default(),
        crear_cliente_default(),
        crear_cliente_default(),
    ];

    // let read_v = fs::read("informacion_clientes.jmosx").unwrap();
    let read_v: Vec<u8> = match fs::read("informacion_clientes.jmosx") {
        Ok(vector) => vector,
        Err(_) => return lista_virgen,
    };
    let vector_decodificado: [Clientes; 30] = bincode::deserialize(&read_v).unwrap();
    vector_decodificado
}

