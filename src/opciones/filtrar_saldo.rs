use crate::Clientes;
use std::io;
use crate::opciones::comandos_consola::limpiar_consola;
use crate::opciones::sistema_tabla::tabla_parcial;

pub fn filtrar_saldo(lista: &[Clientes; 30]) {
    limpiar_consola();
    // Filtrar cédulas por saldo
    let mut saldo = String::new();
    println!("\nDigite el saldo mínimo a buscar\n");
    io::stdin()  //  Función para ingresar un dato por consola
        .read_line(&mut saldo) // Guarda el dato de consola en la variable
        .expect("Error al leer la línea");

    let saldo: u32 = saldo.trim().parse().unwrap_or(0);
    if saldo == 0 {
        limpiar_consola();
        println!("Debe ingresar una opción correcta");
        return;
    }
    println!("Los clientes son un saldo mayor que {saldo} son: ", );
    for i in 0..30 {
        if lista[i].nombre != "xxxx" && lista[i].saldo >= saldo {
            tabla_parcial(lista, i);
        }
    }
}
