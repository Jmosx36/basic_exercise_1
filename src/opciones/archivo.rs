// Creo que ahora lo que sigue es un comprobante de salir sin guardar los últimos cambios
// si el archivo no coincide con el array actual
use std::fs;
use bincode;
use crate::Clientes;
use crate::opciones::comandos_consola::limpiar_consola;


pub fn archivo(lista: &[Clientes; 30]) {

    limpiar_consola();



    let encoded_v = bincode::serialize(&lista).expect("No se pudo codificar el vector");
    fs::write("informacion_clientes.jmosx", encoded_v).expect("No se pudo escribir el archivo");

    println!("Su información ha sido guardada con éxito");

    /*
       Esta sección es para leer el archivo y volverlo a dejar útil
       Creo que usaré esta parte para copiar y pegar

       let read_v = fs::read("informacion_clientes.jmosx").expect("No se pudo leer el archivo");
       let decoded_v: Vec<Clientes> = bincode::deserialize(&read_v).expect("No se pudo decodificar el vector");
       // Esta parte de código es por si es necesario en otra parte

    */

}

pub fn leer_archivo() -> [Clientes; 30] {
       let read_v = fs::read("informacion_clientes.jmosx").expect("No se pudo leer el archivo");
       let decoded_v: [Clientes; 30] = bincode::deserialize(&read_v).expect("No se pudo decodificar el vector");
       decoded_v
}
