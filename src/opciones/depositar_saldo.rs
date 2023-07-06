use crate::Clientes;
use std::io;
use crate::opciones::comandos_consola::limpiar_consola;


pub fn depositar_saldo(lista: &mut [Clientes; 30]) {
    // Ingresar cédula como parámetro
    let mut cedula = String::new();
    let mut saldo_nuevo = String::new();

    limpiar_consola();
    println!("\nDigite su cédula");
    io::stdin()  //  Función para ingresar un dato por consola
        .read_line(&mut cedula) // Guarda el dato de consola en la variable
        .expect("Error al leer la línea");

    let cedula: u32 = cedula.trim().parse().unwrap_or(0);

    if cedula == 0 {
        limpiar_consola();
        println!("Debe ingresar una cédula correcta");
        return;
    }

    let mut comprobante: u8 = 0;
    let mut comprobante_2: u8 = 0;
    for i in 0..30 {
        if lista[i].cedula == cedula {
            comprobante = 36;
        }
        if comprobante == 36 {
            println!("\nCliente {} con cédula {} y saldo {}\n", i, lista[i].cedula, lista[i].saldo);

            println!("\nDigite el saldo que desea depositar");
            io::stdin()  //  Función para ingresar un dato por consola
                .read_line(&mut saldo_nuevo) // Guarda el dato de consola en la variable
                .expect("Error al leer la línea");

            let saldo_nuevo: u32 = saldo_nuevo.trim().parse().unwrap_or_else(|_|{
                limpiar_consola();
                println!("Debe ingresar un dato correcto");
                0
            });

            if saldo_nuevo == 0 { return }

            lista[i].saldo += saldo_nuevo;
            println!("\nSu nuevo saldo es: {}", lista[i].saldo);
            break;
        }else {
            comprobante_2 += 1;
            if comprobante_2 >= 30 {
                println!("\nNo se encontró la cédula {cedula} en la lista de clientes");
                break;
            }
        }

    }
}
