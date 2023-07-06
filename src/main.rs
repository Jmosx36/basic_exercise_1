/*
    Pendientes:
    1) Agregar la tolerancia a fallas (Terminado)
    2) Dividir los múltiples if en subfunciones (Terminado)
    3) Hacer posible expandir la lista de clientes solo agregándolos a la función creadora para que no esté sujeto a 30 IMPOSIBLE
    4) Creo que la siguiente fase será imprimir la información en una tabla mas organizada y estética
    4.1) Creo que el camino a tomar debe ser la librería stanza, le voy a dar el intento (SIP, ES EL CAMINO)
    4.2) En el main no va la librería, solamente en los módulos de impresión de clientes
    4.3) Creo que voy a hacer un módulo general para imprimir tablas y le envío los parámetros correspondientes TERMINADO
    5) Guardar el array de los clientes en un archivo y leer de él en la siguiente ejecución para conservar el estado actual del array TERMINADO
*/

mod opciones; // Ingresa al archivo opciones.rs y busca los módulos que ahí estén especificados
// Hay una carpeta opciones donde están todos los archivos con las funciones de las opciones
// por lo que debe haber un archivo igualmente llamado como la carpeta para poder acceder a ella

use std::io;  // Usar entrada de datos en consola

use opciones::{
    clientes_lista::*,
    consultar_saldo::consultar_saldo,
    depositar_saldo::depositar_saldo,
    eliminar_cliente::eliminar_cliente,
    filtrar_saldo::filtrar_saldo,
    imprimir_opciones::imprimir_opciones,
    ingresar_cliente::ingresar_cliente,
    listar_clientes::listar_clientes,
    modificar_cliente::modificar_cliente,
    retirar_saldo::retirar_saldo,
    saldo::saldo,
    comandos_consola::limpiar_consola,
    archivo::crear_archivo,
    ultimo_cambio::ultimo_cambio,
};


fn main() {
    let mut lista: [Clientes; 30] = crear_lista_clientes();

    limpiar_consola();


    loop {

        imprimir_opciones();  // Imprime en consola la lista de opciones


        println!("Por favor, digite un número de la opción\n");

        let mut valor_consola = String::new(); // Declara la variable de la opción de consola

        io::stdin()  //  Función para ingresar un dato por consola
            .read_line(&mut valor_consola) // Guarda el dato de consola en la variable
            .expect("Error al leer la línea");

        let valor_consola: u32 = match valor_consola.trim().parse() {

            Ok(num) => num,
            Err(_) => {
                limpiar_consola();
                println!("Debe ingresar un número");
                continue;
            },
        };

        match valor_consola { // En vez de hacer una larga comparación con if, se hace un match, pero no
                      // estoy seguri si esto reduce, asi sea un poco, el uso de cpu, no se si es
                      // una comparación mas rápida, pero creo que si

            // Opción 1
            1 => listar_clientes(&lista),

            // Opción 2
            2 => ingresar_cliente(&mut lista),

            // Opción 3
            3 => modificar_cliente(&mut lista),

            // Opción 4
            4 => eliminar_cliente(&mut lista),

            // Opción 5
            5 => {
                println!("\nEscoja una opción");
                println!("1) Saldo de los hombres");
                println!("2) Saldo de las mujeres\n");
                let mut opcion_saldo = String::new(); // Declara la variable de la opción de consola

                io::stdin()  //  Función para ingresar un dato por consola
                    .read_line(&mut opcion_saldo) // Guarda el dato de consola en la variable
                    .expect("Error al leer la línea");

                let opcion_saldo: u32 = opcion_saldo.trim().parse().unwrap_or(0);   // trim hace que se eliminen espacios y saltos de línea al principio y final del string, parse analiza y el unwrap or hace que se devuelva el contenido de Ok o se devuelva un 0

                match opcion_saldo {
                    1 => saldo(&lista, 1),
                    2 => saldo(&lista, 2),
                    _ => {
                        limpiar_consola();
                        println!("Debe escoger una opción correcta");
                    }
                };
            },

            // Opción 6
            6 => filtrar_saldo(&lista),

            // Opción 7
            7 => depositar_saldo(&mut lista),

            // Opción 8
            8 => retirar_saldo(&mut lista),

            // Opción 9
            9 => consultar_saldo(&lista),

            // Opción 10
            10 => crear_archivo(&lista),

            // Opción 11
            11 => match ultimo_cambio(&lista) {
                0 => break,
                _ => continue,
            },

            // Opción ninguna de las anteriores
            _ => {
                println!("\nDebe digitar una opción correcta\n");
            },
        };
    }
}
