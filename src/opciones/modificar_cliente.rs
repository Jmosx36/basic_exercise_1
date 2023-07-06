// Tal vez deba poner los ifs en múltiples subfunciones, pero sería mas complicado y no lo veo tan
// necesario
use crate::opciones::clientes_lista::Clientes;
use std::io;
use crate::opciones::comandos_consola::limpiar_consola;
use crate::opciones::sistema_tabla::tabla_completa;

pub fn modificar_cliente(lista: &mut [Clientes; 30]) {
    limpiar_consola();
    // Listar los clientes creados y dejarle elejir al usuario que campo modificar
    for i in 0..30 {
        if lista[i].nombre == "xxxx" {

        } else {
            tabla_completa(&lista, i);
        }

    }

    let mut cedula = String::new();

    println!("\nDigite su cédula");
    io::stdin()  //  Función para ingresar un dato por consola
        .read_line(&mut cedula) // Guarda el dato de consola en la variable
        .expect("Error al leer la línea");

    let cedula: u32 = cedula.trim().parse().unwrap_or(0);

    let mut comprobante: u8 = 0;
    let mut comprobante_2: u8 = 0;
    let mut opcion_campo = String::new();
    let mut new_cedula = String::new();
    let mut new_nombre = String::new();
    let mut new_saldo = String::new();
    let mut opcion_sexo = String::new();
    let mut sexo_resul = String::new();
    for i in 0..30 {
        if lista[i].cedula == cedula {
            comprobante = 36;
        }
        if comprobante == 36 {
            println!("\n1) Modificar cédula");
            println!("2) Modificar nombre");
            println!("3) Modificar sexo");
            println!("4) Modificar saldo");
            println!("\nEscoja una opción\n");
            io::stdin()
                .read_line(&mut opcion_campo)
                .expect("Error al leer la línea");

            let opcion_campo: u32 = opcion_campo.trim().parse().unwrap_or(0);

            match opcion_campo {
                1 => { // Cédula

                    println!("Digite su nueva cédula\n");
                    io::stdin()
                        .read_line(&mut new_cedula)
                        .expect("Error al leer la línea");

                    let new_cedula: u32 = new_cedula.trim().parse().unwrap_or(0);

                    lista[i].cedula = new_cedula;
                    println!("Su nueva cédula es: {new_cedula}", );
                    break;
                },

                2 => { // Nombre

                    println!("Digite su nuevo nombre\n");
                    io::stdin()
                        .read_line(&mut new_nombre)
                        .expect("Error al leer la línea");

                    lista[i].nombre = new_nombre;
                    println!("Su nuevo nombre es: {}", lista[i].nombre.trim());
                    break;
                },

                3 =>  { // Sexo

                    println!("\n1) Masculino");
                    println!("2) Femenino");
                    println!("\nIngrese la opción del sexo");
                    io::stdin() // Toma un valor por consola
                        .read_line(&mut opcion_sexo)
                        .expect("Error al leer la línea");
                    let opcion_sexo: u32 = opcion_sexo.trim().parse().unwrap_or(0);

                    if opcion_sexo == 1 {
                        sexo_resul = String::from("Masculino");
                    }else if opcion_sexo == 2 {
                        sexo_resul = String::from("Femenino");
                    }else {
                        sexo_resul = String::from("Desconocido"); // Si se digita una opción de sexo incorrecta, este será el resultado
                    }
                    lista[i].sexo = sexo_resul;
                    println!("Su nuevo sexo es: {}", lista[i].sexo);
                    break;
                },

                4 => { // Saldo

                    println!("Digite su nuevo saldo\n");
                    io::stdin()
                        .read_line(&mut new_saldo)
                        .expect("Error al leer la línea");

                    let new_saldo: u32 = new_saldo.trim().parse().unwrap_or(0);

                    lista[i].saldo = new_saldo;
                    println!("Su nuevo saldo es: {new_saldo}", );
                    break;

                },

                 _ => {

                     println!("Debe escoger una opción válida");
                     break;

                 },
            };

        } else {
            comprobante_2 += 1;
            if comprobante_2 >= 30 {
                println!("\nNo se encontró la cédula {cedula} en la lista de clientes");
                break;
            }
        }

    }
}
