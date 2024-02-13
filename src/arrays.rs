fn main() {

    let mut nombres : Vec<String> = Vec::new();

    for i in 0..3{
        println!("Escribe un nombre: ");
        let mut nombre : String = String::new();
        std::io::stdin().read_line(&mut nombre).unwrap();
        nombre = nombre.trim().to_string();

        nombres.push(nombre);
    }

    for nombre in &nombres{
        println!("El nombre es: {}", nombre);
    }
    println!("{:?}", nombres);
}
