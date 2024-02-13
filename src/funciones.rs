fn sumar_uno(numero_a_sumar: i32) -> i32 {
    let mut numero_final = numero_a_sumar + 1;
    println!("{}", numero_final);

    return numero_final;
}

fn main() {
    let once_mas_uno = sumar_uno(11);
    sumar_uno(once_mas_uno);
    sumar_uno(112);
}
