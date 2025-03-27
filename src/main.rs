use std::io;

struct Gasto {
    descripcion: String,
    monto: f64, // f64 es para numero decimales.
}

fn main() {
    // Crear variables para almacenar los datos ingresados por el usuario.
    let mut descripcion = String::new();
    let mut monto = String::new();

    // Pedir al usuario que ingrese la descripción del gasto.
    println!("¿Cuál es la descripción del gasto?");
    io::stdin()
        .read_line(&mut descripcion)
        .expect("Error al leer la entrada");

    // Pedir al usuario que ingrese el monto del gasto.
    println!("¿Cuál es el monto del gasto?");
    io::stdin()
        .read_line(&mut monto)
        .expect("Error al leer la entrada");

    // Convertir el monto ingresado (que es texto) a un número decimal.
    let monto: f64 = monto.trim().parse().expect("Por favor ingresa un número válido");

    // Crear una instancia del struct Gasto con los datos ingresados.
    let gasto1 = Gasto {
        descripcion: descripcion.trim().to_string(),
        monto,
    };

    // Mostrar los datos del gasto creado.
    println!(
        "Has registrado un gasto de '{}' por ${:.2}",
        gasto1.descripcion, gasto1.monto
    );
}
