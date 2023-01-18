use crate::Clientes;
use std::io;
use crate::opciones::comandos_consola::limpiar_consola;
use crate::opciones::sistema_tabla::tabla_parcial;

pub fn filtrar_saldo(lista: &[Clientes; 30]) {
    limpiar_consola();
    // Filtrar cédulas por saldo
    let mut saldo = String::new();
    println!("");
    println!("Digite el saldo mínimo a buscar");
    println!("");
    io::stdin()  //  Función para ingresar un dato por consola
        .read_line(&mut saldo) // Guarda el dato de consola en la variable
        .expect("Error al leer la línea");

    let saldo: u32 = match saldo.trim().parse() {  // trim hace que se eliminen espacios y saltos de línea al principio y final del string, parse analiza el contenido de la variable y lo convierte al tipo de dato de la declaración de la variable al principio de la línea
        Ok(num) => num,    // El parse es un enum, y puede regresar Ok o Err, el match se utiliza para crear una acción si se presenta cualquiera de ambos resultados
        Err(_) => 0,   // Si se ingresa un valor incorrecto se mostrarán todos los saldos
    };
    if saldo == 0 {
        limpiar_consola();
        println!("Debe ingresar una opción correcta");
        return;
    }
    println!("Los clientes son un saldo mayor que {saldo} son: ", );
    for i in 0..30 {
        if lista[i].nombre == "xxxx" {
        }else {
            if lista[i].saldo >= saldo {
                tabla_parcial(&lista, i);
            }
        }
    }

}
