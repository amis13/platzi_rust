fn main() {
    println!("Escribe tu nombre: ");
    let mut nombre : String = String::new();
    
    std::io::stdin().read_line(&mut nombre).unwrap();
    nombre = nombre.trim().to_string();

    // Obtener edad en consola y convertir en numero
    println!("Escribe edad: ");
    let mut edad : String = String::new();
    std::io::stdin().read_line(&mut edad).unwrap();

    let edad_int : u8 = edad.trim().parse().unwrap();

    println!("Hola {}, está bien que tengas {}, yo 27", nombre, edad_int);
}
