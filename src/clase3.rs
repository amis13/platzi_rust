fn main() {
    println!("Introduce tu nombre: ");
    let mut nombre : String = String::new();
    std::io::stdin().read_line(&mut nombre).unwrap();
    nombre = nombre.trim().to_string();

    println!("Elige tu pastilla: ");
    println!("[ AZUL 💊 ] | [ ROJA 💊 ]");
    let mut pastilla : String = String::new();
    
    std::io::stdin().read_line(&mut pastilla).unwrap();
    pastilla = pastilla.trim().to_lowercase().to_string();

    if pastilla == "roja" {
        println!("Hola {}, enhorabuena por elegir la pastilla {}, vuelta a la realidad...", nombre, pastilla);
    } else if pastilla == "azul"{
        println!("Hola {}, enhorabuena por elegir la pastilla {}, BIENVENIDO A MATRIX...", nombre, pastilla);
    } else {
        println!("SOLO ROJA O AZUL {}, no existe {}...", pastilla, nombre);
    }
}
