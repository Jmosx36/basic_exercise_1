/*
   Este módulo es para definir si el último cambio ya se guardó
   Y para desplegar un menú en caso de que no
*/

use crate::opciones::comandos_consola::limpiar_consola;
use crate::Clientes;
use crate::opciones::archivo::*;

use std::io;

pub fn ultimo_cambio(lista: &[Clientes; 30]) -> u8  {
    limpiar_consola();




    let archivo_info = leer_archivo();


    let mut comparacion: u8 = 0;
    for i in 0..=29 {
        if archivo_info[i] == lista[i] {
            continue;
        } else {
            comparacion += 1;
        }
    }
    if comparacion == 0 {
        println!("Muchas gracias, vuelva pronto\nPrograma hecho por Jmosx36");
        return 0;
    }
    println!("Hay cambios aún sin guardar\n");
    println!("1) Salir sin guardar\n2) Guardar y salir\n3) Regresar");

    let mut valor = String::new(); // Declara la variable de la opción de consola

    io::stdin()  //  Función para ingresar un dato por consola
        .read_line(&mut valor) // Guarda el dato de consola en la variable
        .expect("Error al leer la línea");

    let valor: u32 = valor.trim().parse().unwrap_or({
        println!("Debe ingresar un número"); 36 // No importa el número que regrese, con tal de que no sea 0 al 2
    });

    match valor {
        1 => { // Salir sin guardar
            limpiar_consola();
            println!("Muchas gracias, vuelva pronto\nPrograma hecho por Jmosx36");
            0
        },

        2 => { // Guardar y salir
            crear_archivo(lista); // Guarda la información actual
            0
        },

        3 => 1,
        _ => 1,
    }

}
