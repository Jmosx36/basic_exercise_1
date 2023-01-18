use crate::Clientes;
use crate::opciones::comandos_consola::limpiar_consola;

pub fn saldo(lista: &[Clientes; 30], parametro: i8) {
    limpiar_consola();
    if parametro == 1 {
        let mut total_saldo: u32 = 0;
            for i in 0..30 {
                if lista[i].sexo == "Masculino" {
                    total_saldo += lista[i].saldo;
                }
            }
        println!("El saldo total para los hombres es: {}", total_saldo);
        println!("");



    }else if parametro == 2 {
        let mut total_saldo: u32 = 0;
        for i in 0..30 {
            if lista[i].sexo == "Femenino" {
                total_saldo += lista[i].saldo;
            }
        }
        println!("El saldo total para las mujeres es: {}", total_saldo);
        println!("");


    }
       
}

