use crate::Clientes;
use std::io;
use crate::opciones::comandos_consola::limpiar_consola;

pub fn eliminar_cliente(lista: &mut [Clientes; 30]) {
    limpiar_consola();
    // Hay que buscar al cliente por su cédula
    let mut cedula_eliminar = String::new();

    println!("\nDigite su cédula");
    io::stdin()
        .read_line(&mut cedula_eliminar)
        .expect("Error al leer la línea");

    let cedula_eliminar: u32 = cedula_eliminar.trim().parse().unwrap_or(0);
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
            println!("\nEl cliente con cédula {cedula_eliminar} ha sido eliminado");
            break;
        }else {
            comprobante_2 += 1;
            if comprobante_2 >= 30 {
                println!("\nNo se encontró la cédula {cedula_eliminar} en la lista de clientes");
                break;
            }
        }
    }

}
