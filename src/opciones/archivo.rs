// Creo que ahora lo que sigue es un comprobante de salir sin guardar los últimos cambios
// si el archivo no coincide con el array actual
use std::fs;
use bincode;
use crate::Clientes;
use crate::opciones::comandos_consola::limpiar_consola;
use crate::opciones::clientes_lista::crear_lista_clientes;


/// Función utilizada para crear y cifrar el archivo que
/// contiene el array de clientes
pub fn crear_archivo(lista: &[Clientes; 30]) {

    limpiar_consola();



    let encoded_v = bincode::serialize(&lista).expect("No se pudo codificar el vector");
    fs::write("informacion_clientes.jmosx", encoded_v).expect("Hubo un problema al escribir el archivo");

    println!("Su información ha sido guardada con éxito");

    /*
       Esta sección es para leer el archivo y volverlo a dejar útil
       Creo que usaré esta parte para copiar y pegar

       let read_v = fs::read("informacion_clientes.jmosx").expect("No se pudo leer el archivo");
       let decoded_v: Vec<Clientes> = bincode::deserialize(&read_v).expect("No se pudo decodificar el vector");
       // Esta parte de código es por si es necesario en otra parte

    */

}

/// Función para leer y descifrar el archivo que contiene
/// vector de los clientes
///
/// En caso de que el archivo no exista, lo crea
pub fn leer_archivo() -> [Clientes; 30] {
       let read_v = fs::read("informacion_clientes.jmosx").unwrap_or_else( |_| {
            println!("Hubo un error al leer el archivo");
            crear_archivo(&crear_lista_clientes());
            fs::read("informacion_clientes.jmosx").unwrap()
       });
       let decoded_v: [Clientes; 30] = bincode::deserialize(&read_v).unwrap_or_else( |_| {
           println!("No se pudo decodificar el archivo");
           crear_lista_clientes()
       });
       decoded_v
}
