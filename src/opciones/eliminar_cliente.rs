use crate::Clientes;
use std::io;
use crate::opciones::comandos_consola::limpiar_consola;

pub fn eliminar_cliente(lista: &mut [Clientes; 30]) {
    limpiar_consola();
    // Hay que buscar al cliente por su cédula
    let mut cedula_eliminar = String::new();

    println!("");
    println!("Digite su cédula");
    io::stdin()  //  Función para ingresar un dato por consola
        .read_line(&mut cedula_eliminar) // Guarda el dato de consola en la variable
        .expect("Error al leer la línea");

    let cedula_eliminar: u32 = match cedula_eliminar.trim().parse() {  // trim hace que se eliminen espacios y saltos de línea al principio y final del string, parse analiza el contenido de la variable y lo convierte al tipo de dato de la declaración de la variable al principio de la línea
        Ok(num) => num,    // El parse es un enum, y puede regresar Ok o Err, el match se utiliza para crear una acción si se presenta cualquiera de ambos resultados
        Err(_) => 0,   // El _ significa que acepte cualquier argumento que pueda llegar dentro del paréntesis
    };
    if cedula_eliminar == 0 {
        println!("Debe digitar una cédula válida");
        return;
    }
    let mut comprobante: u8 = 0;
    let mut comprobante_2: u8 = 0;
    for i in 0..30 {
        if lista[i].cedula == cedula_eliminar {
            comprobante = 36;
        }
        if comprobante == 36 {
            // Eliminar cliente
            lista[i].cedula = 0000000000;
            lista[i].nombre = String::from("xxxx");
            lista[i].sexo = String::from("xxxx");
            lista[i].saldo = 00000000;
            println!("");
            println!("El cliente con cédula {cedula_eliminar} ha sido eliminado");
            break;
        }else {
            comprobante_2 += 1;
            if comprobante_2 >= 30 {
                println!("");
                println!("No se encontró la cédula {cedula_eliminar} en la lista de clientes");
                break;
            }
        }
    }

}
