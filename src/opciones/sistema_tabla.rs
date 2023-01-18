// La idea es que a este módulo se le envíen la lista de clientes y el número del clietne que se
// desee imprimir
// Solo en este módulo vendrán la forma completa de imprimir clientes y cada forma parcial

use crate::Clientes;
use stanza::renderer::{Renderer, console::Console};
use stanza::table::Table;


pub fn tabla_completa(lista: &[Clientes; 30], opcion: usize) {

    // Se crea la información de la tabla, cada separación por comas es cada columna
    // El ancho de cada columna se define por el contenido mas largo de cualquiera de sus celdas
    let tabla_completa = Table::default()
        .with_row(["Número de cliente", format!("{opcion}").as_str()])
        .with_row(["Cédula", format!("{}", lista[opcion].cedula).as_str()])
        .with_row(["Nombre", lista[opcion].nombre.trim()])
        .with_row(["Sexo", &lista[opcion].sexo.to_string()])
        .with_row(["Saldo", format!("{}", lista[opcion].saldo).as_str()]);

    // Convierte la tabla en un string imprimible en consola
    let renderizador = Console::default();

    println!("\n{}", renderizador.render(&tabla_completa));

}


pub fn tabla_parcial(lista: &[Clientes; 30], posicion: usize) {

    let tabla_parcial = Table::default()
        .with_row(["Número de cliente", format!("{}", posicion).as_str()])
        .with_row(["Cédula", format!("{}", lista[posicion].cedula).as_str()]);

    let renderizador = Console::default();

    println!("\n{}", renderizador.render(&tabla_parcial));


}
