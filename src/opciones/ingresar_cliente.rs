use crate::Clientes;
use crate::opciones::comandos_consola::limpiar_consola;
use crate::opciones::sistema_tabla::tabla_completa;
use std::io;


pub fn ingresar_cliente(lista: &mut [Clientes; 30]) {

    limpiar_consola();

    println!("Espacios disponibles en la lista");
    for i in 0..30 {
        if lista[i].nombre == "xxxx" {
            println!("Espacio {i}");
        }
    }
    println!("Ingrese el número del cliente que desea ingresar");

    let mut opcion = String::new(); // Declara la variable de la opción
    io::stdin() // Toma un valor por consola
        .read_line(&mut opcion)
        .expect("Error al leer la línea");
    let opcion: usize= match opcion.trim().parse() { // Convierte el valor de consola en un integer (al parecer debe ser usize)
        Ok(num) => num,
        Err(_) => {
            println!("Debe ingresar una opción válida");
            return;
        },

    };
    if opcion > 29 { // Valida si se ingresó un número
        println!("Debe ingresar una opción válida");
    }else{
        if opcion <= 29 { // Valida si se ingresó un número en el rango de 30-1 clientes
            // Código de creación de cliente
            let mut cedula = String::new();
            let mut nombre = String::new();
            let mut sexo = String::new();
            let mut sexo_resul = String::new();
            let mut saldo = String::new();

            println!("");
            println!("Ingrese la cédula");
            io::stdin() // Toma un valor por consola
                .read_line(&mut cedula)
                .expect("Error al leer la línea");
            let cedula: u32 = match cedula.trim().parse() { // Convierte el valor de consola en un integer
                Ok(num) => num,
                Err(_) => 1,
            };

            println!("");
            println!("Ingrese el nombre");
            io::stdin() // Toma un valor por consola
                .read_line(&mut nombre)
                .expect("Error al leer la línea");

            println!("");
            println!("1) Masculino");
            println!("2) Femenino");
            println!("");
            println!("Ingrese la opción del sexo");
            io::stdin() // Toma un valor por consola
                .read_line(&mut sexo)
                .expect("Error al leer la línea");
            let sexo: u32 = match sexo.trim().parse() { // Convierte el valor de consola en un integer
                Ok(num) => num,
                Err(_) => 0,
            };
            if sexo == 1 {
                sexo_resul = String::from("Masculino");
            }else if sexo == 2 {
                sexo_resul = String::from("Femenino");
            }else {
                sexo_resul = String::from("Desconocido"); // Si se digita una opción de sexo incorrecta, este será el resultado
            }

            println!("");
            println!("Ingrese el saldo");
            io::stdin() // Toma un valor por consola
                .read_line(&mut saldo)
                .expect("Error al leer la línea");
            let saldo: u32 = match saldo.trim().parse() { // Convierte el valor de consola en un integer
                Ok(num) => num,
                Err(_) => 0,
            };

            lista[opcion].cedula = cedula;  // Ingresa los valores al campo de la lista correspondiente
            lista[opcion].nombre = nombre;
            lista[opcion].sexo = sexo_resul;
            lista[opcion].saldo = saldo;

            // Aquí es donde debe ir la tabla
            limpiar_consola();
            tabla_completa(&lista, opcion);

            println!("Su cliente fue ingresado correctamente");


        } else{
            println!("Debe ingresar un número de cliente válido");
        }
    }

}
