// Modificar para que no se pueda restar mas de lo que se tiene

use crate::Clientes;
use std::io;
use crate::opciones::comandos_consola::limpiar_consola;

pub fn retirar_saldo(lista: &mut [Clientes; 30]) {
    limpiar_consola();
    // Ingresar cédula como parámetro
    let mut cedula = String::new();
    let mut saldo_nuevo = String::new();

    println!("\nDigite su cédula");
    io::stdin()
        .read_line(&mut cedula)
        .expect("Error al leer la línea");

    let cedula: u32 = cedula.trim().parse().unwrap_or(0);

    if cedula == 0{
        limpiar_consola();
        println!("Debe ingresar una cédula correcta");
        return;
    }
    let mut comprobante: u8 = 0;
    let mut comprobante_2: u8 = 0;
    for i in 0..30 {
        if lista[i].cedula == cedula {
            comprobante = 36; // Comprueba si la cédula ingresada está en el array
        }
        if comprobante == 36 { // Si se encontró la cédula en tonces ejecuta este código
            println!("\nCliente {} con cédula {} y saldo {}\n", i, lista[i].cedula, lista[i].saldo);

            println!("\nDigite el saldo que desea retirar");
            io::stdin()  //  Función para ingresar un dato por consola
                .read_line(&mut saldo_nuevo) // Guarda el dato de consola en la variable
                .expect("Error al leer la línea");

            let saldo_nuevo: u32 = match saldo_nuevo.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    limpiar_consola();
                    println!("Debe ingresar un dato correcto");
                    return;
                },
            };

            limpiar_consola();
            if saldo_nuevo > lista[i].saldo {
                println!("El saldo ingresado es mayor al saldo disponible");
                break;
            } else {
                lista[i].saldo -= saldo_nuevo;
                println!("\nSu nuevo saldo es: {}", lista[i].saldo);
                break;
            }

        }else {
            comprobante_2 += 1;
            if comprobante_2 >= 30 { // Si después de recorrer todo el array no se encuentra la cédula se ejecuta este código
                println!("\nNo se encontró la cédula {cedula} en la lista de clientes");
                break;
            }
        }

    }
}
