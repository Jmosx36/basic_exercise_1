use crate::Clientes; // Aquí hay que usar el path absoluto
use std::io;
use crate::opciones::comandos_consola::limpiar_consola;
use crate::opciones::sistema_tabla::tabla_completa;


pub fn consultar_saldo(lista: &[Clientes; 30]) {
    limpiar_consola();

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
    if cedula == 0 {
        limpiar_consola();
        println!("Debe ingresar una cédula correcta");
        return;
    }
    let mut comprobante: u8 = 0;
    let mut comprobante_2: u8 = 0;
    for i in 0..30 {
        if lista[i].cedula == cedula { // Recorre la lista de clientes en búsqueda de una coincidencia
            comprobante = 36;
        }
        if comprobante == 36 { // Si encuentra una coincidencia
            tabla_completa(&lista, i); // Imprime la tabla con la info del cliente respectivo
            break;
        }else { // En el caso de no haber encontrado una coincidencia
            comprobante_2 += 1;
            if comprobante_2 >= 30 {
                println!("");
                println!("No se encontró la cédula {cedula} en la lista de clientes");
                break;
            }
        }
    }
}
