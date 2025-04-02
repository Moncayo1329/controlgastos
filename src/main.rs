    use std::fs::OpenOptions;
    use std::io::Write;
    use std::io;

fn guardar_en_archivo(gastos: &Vec<Gasto>){
    let mut archivo = OpenOptions::new()
    .write(true)
    .append(true)
    .create(true)
    .open("gastos.txt")
    .expect("No se pudo abrir el archivo");

    for gasto in gastos{
        
        writeln!(archivo,"{} - ${:.2}", gasto.descripcion, gasto.monto)
            .expect("Error al escribir en el archivo");
    }
}


    #[derive(Debug)]
    enum Categoria {
        Alimentos,
        Transporte,
        Entretenimiento,
        Comida,
        Otros, 
    }

    struct Gasto {
        descripcion: String,
        monto: f64, // f64 es para numero decimales.
        categoria:Categoria,
    }

    fn main() {

        let mut gastos: Vec<Gasto> = Vec::new();


    loop{

        // Crear variables para almacenar los datos ingresados por el usuario.
        let mut descripcion = String::new();
        let mut monto = String::new();
        let mut  categoria = String::new();

    


        // Pedir al usuario que ingrese la descripción del gasto.
        println!("¿Cuál es la descripción del gasto? o escriba salir para terminar");
        io::stdin()
            .read_line(&mut descripcion)
            .expect("Error al leer la entrada");

    if descripcion.trim().eq_ignore_ascii_case("salir"){
        break;
    }


        // Pedir al usuario que ingrese el gasto 
        println!("¿Cuál es el monto del gasto?");
        io::stdin()
            .read_line(&mut monto)
            .expect("Error al leer la entrada");

// Pedir al usuario que ingrese el gasto 

            println!("¿Cuál es la categoria del gasto");
        io::stdin()
            .read_line(&mut categoria)
            .expect("Error al leer la categoria");


      // Categoria 
      let categoria = match categoria.trim().to_lowercase().as_str(){
        "comida" => Categoria::Comida,
        "transporte" => Categoria::Transporte,
        "entretenimiento" => Categoria::Entretenimiento,
      "alimentos" => Categoria::Alimentos,
        "otros" => Categoria::Otros,
        _ => {
            println!("Categoría inválida. Intente nuevamente.");
            continue;
        }
    };

 



        // Convertir el monto ingresado (que es texto) a un número decimal.
        let monto: f64 = match monto.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Porfavor ingrese un numero valido");
            continue;
        }

        };

        gastos.push(Gasto {
            descripcion: descripcion.trim().to_string(),
            monto,
            categoria,
        });
        println!("Gasto agregado correctamente!\n");
    }

    guardar_en_archivo(&gastos);  

    println!("\nResumen de gastos:");
    for gasto in &gastos{
        println!("- {}: ${:.2} ({:?})", gasto.descripcion, gasto.monto, gasto.categoria);
    }
    let total: f64 = gastos.iter().map(|g| g.monto).sum();
    println!("Total gastado: ${:.2}", total);

    println!("Los gastos han sido guardados en 'gastos.txt'.");

    }