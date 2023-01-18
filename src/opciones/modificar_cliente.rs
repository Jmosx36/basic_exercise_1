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

    println!("");
    println!("Digite su cédula");
    io::stdin()  //  Función para ingresar un dato por consola
        .read_line(&mut cedula) // Guarda el dato de consola en la variable
        .expect("Error al leer la línea");

    let cedula: u32 = match cedula.trim().parse() {  // trim hace que se eliminen espacios y saltos de línea al principio y final del string, parse analiza el contenido de la variable y lo convierte al tipo de dato de la declaración de la variable al principio de la línea
        Ok(num) => num,    // El parse es un enum, y puede regresar Ok o Err, el match se utiliza para crear una acción si se presenta cualquiera de ambos resultados
        Err(_) => 0,   // El _ significa que acepte cualquier argumento que pueda llegar dentro del paréntesis
    };
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
            println!("");
            println!("1) Modificar cédula");
            println!("2) Modificar nombre");
            println!("3) Modificar sexo");
            println!("4) Modificar saldo");
            println!("");
            println!("Escoja una opción");
            println!("");
            io::stdin()
                .read_line(&mut opcion_campo)
                .expect("Error al leer la línea");

            let opcion_campo: u32 = match opcion_campo.trim().parse() {
                Ok(num) => num,
                Err(_) => 0,
            };
            if opcion_campo == 1 { // Cédula
                println!("Digite su nueva cédula");
                println!("");
                io::stdin()
                    .read_line(&mut new_cedula)
                    .expect("Error al leer la línea");

                let new_cedula: u32 = match new_cedula.trim().parse() {
                    Ok(num) => num,
                    Err(_) => 0,
                };
                lista[i].cedula = new_cedula;
                println!("Su nueva cédula es: {new_cedula}", );
                break;

            }else if opcion_campo == 2 { // Nombre
                println!("Digite su nuevo nombre");
                println!("");
                io::stdin()
                    .read_line(&mut new_nombre)
                    .expect("Error al leer la línea");

                lista[i].nombre = new_nombre;
                println!("Su nuevo nombre es: {}", lista[i].nombre.trim());
                break;

            }else if opcion_campo == 3 { // Sexo
                println!("");
                println!("1) Masculino");
                println!("2) Femenino");
                println!("");
                println!("Ingrese la opción del sexo");
                io::stdin() // Toma un valor por consola
                    .read_line(&mut opcion_sexo)
                    .expect("Error al leer la línea");
                let opcion_sexo: u32 = match opcion_sexo.trim().parse() { // Convierte el valor de consola en un integer
                    Ok(num) => num,
                    Err(_) => 0,
                };
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

            }else if opcion_campo == 4 { // Saldo
                println!("Digite su nuevo saldo");
                println!("");
                io::stdin()
                    .read_line(&mut new_saldo)
                    .expect("Error al leer la línea");

                let new_saldo: u32 = match new_saldo.trim().parse() {
                    Ok(num) => num,
                    Err(_) => 0,
                };
                lista[i].saldo = new_saldo;
                println!("Su nuevo saldo es: {new_saldo}", );
                break;

            }else {
                println!("Debe escoger una opción válida");
                break;
            }
        } else {
            comprobante_2 += 1;
            if comprobante_2 >= 30 {
                println!("");
                println!("No se encontró la cédula {cedula} en la lista de clientes");
                break;
            }
        }

    }
}
