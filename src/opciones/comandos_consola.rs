use std::process::Command;


pub fn limpiar_consola() {
    let mut koso = Command::new("clear"); // El comando que está entre comillas será el que se ejecutará en consola
    match koso.output() {
        Ok(o) => {
            unsafe {
                println!("{}", String::from_utf8_unchecked(o.stdout));
            }
        }
        Err(e) => {
            println!("Hubo un error al limpiar la consola{}", e);
        }
    }

}
// En teoría, esta función debería funcionar en cualquier sistema operativo, excepto FreeBSD, aunque eso no lo confirmo

// Creo que debería agregar mas comandos en este formulario, pero ya veremos si me acuerdo o se me ocurren mas ideas
// Creo que al final no se me ocurrieron mas ideas
