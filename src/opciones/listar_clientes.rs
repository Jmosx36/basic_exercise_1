use crate::Clientes;
use crate::opciones::comandos_consola::limpiar_consola;
use crate::opciones::sistema_tabla::tabla_completa;

pub fn listar_clientes(lista: &[Clientes; 30]) { // Opción número 1
    limpiar_consola();
    for i in 0..30 {
        if lista[i].nombre == "xxxx" {
            
        } else {
            tabla_completa(&lista, i);
        }
        
    }
}
