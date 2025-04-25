mod gastos;

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use std::fs::OpenOptions;
use std::io::Write;
use gastos::{Gasto, Categoria, filtrar_por_categoria};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::{Arc, Mutex}};
use tokio::time::{sleep, Duration};
use reqwest::Client;
use axum::serve;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use axum::http::{HeaderValue, header::CONTENT_TYPE, Method};
use supabase::SupabaseClient;
use serde_json::json; // Para construir objetos JSON fácilmente


// Estructura para entrada de gastos
#[derive(Deserialize, Serialize)]
struct GastoInput {
    descripcion: String,
    monto: f64,
    categoria: Categoria,
}

// Función para guardar gastos en archivo
fn guardar_en_archivo(gastos: &Vec<Gasto>) -> std::io::Result<()> {
    let mut archivo = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("gastos.txt")?;

    for gasto in gastos {
        writeln!(
            archivo,
            "{} - ${:.2} - {:?}",
            gasto.descripcion, gasto.monto, gasto.categoria
        )?;
    }
    Ok(())
}

// Rutas de la API
async fn obtener_gastos(State(db): State<BD>) -> Json<Vec<Gasto>> {
    let supabase_url = env::var("SUPABASE_URL").expect("SUPABASE_URL no encontrada");
    let supabase_key = env::var("SUPABASE_KEY").expect("SUPABASE_KEY no encontrada");
    let client = SupabaseClient::new(supabase_url,supabase_key);

    let response = client
    .from("gastos")
    .select("*")
    .execute()
    .await
    .unwrap();
   
    println!("Respuesta de Supabase: {:?}", response);

    let gastos: Vec<Gasto> = serde_json::from_str(&response.body()).unwrap();

    Json(gastos)
}

async fn agregar_gasto(State(db): State<BD>, Json(payload): Json<GastoInput>) -> &'static str {
    let supabase_url = env::var("SUPABASE_URL").expect("SUPABASE_URL no encontrada");
    let supabase_key = env::var("SUPABASE_KEY").expect("SUPABASE_KEY no encontrada");
    let client = SupabaseClient::new(supabase_url,supabase_key);
   
    let nuevo_gasto = json!({
        "descripcion": payload.descripcion,
        "monto": payload.monto,
        "categoria":payload.categoria.to_string,
    });

    let response = client
    .from("gastos")
    .insert(nuevo_gasto)
    .execute()
    .await
    .unwrap();

    println!("Respuesta de Supabase:{:?}", response)

    "Gasto agregado (en Supabase)"
}

async fn gastos_por_categoria(Path(cat): Path<String>, State(db): State<BD>) -> Json<Vec<Gasto>> {
    let cat = match cat.to_lowercase().as_str() {


        let supabase_url = env::var("SUPABASE_URL").expect("SUPABASE_URL no encontrada");
    let supabase_key = env::var("SUPABASE_KEY").expect("SUPABASE_KEY no encontrada");
        let client = SupabaseClient::new(supabase_url,supabase_key);



        "comida" => Categoria::Comida,
        "transporte" => Categoria::Transporte,
        "entretenimiento" => Categoria::Entretenimiento,
        "alimentos" => Categoria::Alimentos,
        "otros" => Categoria::Otros,
        _ => Categoria::Otros,
    };

    let response = client
        .from("gastos")
        .select("*")
        .eq("categoria", cat.clone()) // Usa cat (string) directamente
        .execute()
        .await
        .unwrap();

    println!("Respuesta de Supabase: {:?}", response);

    // Deserializar la respuesta a Vec<Gasto>
    let mut gastos: Vec<Gasto> = serde_json::from_str(&response.body()).unwrap();

    // Actualizar la categoría de string a enum en los resultados
    for gasto in &mut gastos {
        gasto.categoria = categoria_enum.clone(); // Usa el enum convertido
    }

    Json(gastos)
}

// Consola que interactúa con la API
async fn consola() {
    let client = Client::new();
    let base_url = "http://127.0.0.1:4000";

    loop {
        println!("\nOpciones:");
        println!("1. Agregar gasto");
        println!("2. Ver todos los gastos");
        println!("3. Filtrar gastos por categoría");
        println!("4. Salir");

        let mut opcion = String::new();
        std::io::stdin()
            .read_line(&mut opcion)
            .expect("Error al leer la entrada");
        let opcion = opcion.trim();

        match opcion {
            "1" => {
                let mut descripcion = String::new();
                let mut monto = String::new();
                let mut categoria = String::new();

                println!("Descripción del gasto (o 'salir' para cancelar):");
                std::io::stdin()
                    .read_line(&mut descripcion)
                    .expect("Error al leer la entrada");

                if descripcion.trim().eq_ignore_ascii_case("salir") {
                    continue;
                }

                println!("Monto del gasto:");
                std::io::stdin()
                    .read_line(&mut monto)
                    .expect("Error al leer la entrada");

                let monto: f64 = match monto.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Por favor, ingrese un número válido.");
                        continue;
                    }
                };

                println!("Categoría (comida, transporte, entretenimiento, alimentos, otros):");
                std::io::stdin()
                    .read_line(&mut categoria)
                    .expect("Error al leer la entrada");

                let categoria = match categoria.trim().to_lowercase().as_str() {
                    "comida" => Categoria::Comida,
                    "transporte" => Categoria::Transporte,
                    "entretenimiento" => Categoria::Entretenimiento,
                    "alimentos" => Categoria::Alimentos,
                    "otros" => Categoria::Otros,
                    _ => {
                        println!("Categoría inválida.");
                        continue;
                    }
                };

                let gasto = GastoInput {
                    descripcion: descripcion.trim().to_string(),
                    monto,
                    categoria,
                };

                match client
                    .post(&format!("{}/gasto", base_url))
                    .json(&gasto)
                    .send()
                    .await
                {
                    Ok(resp) => println!("Respuesta: {}", resp.text().await.unwrap_or_default()),
                    Err(e) => println!("Error al agregar gasto: {}. Asegúrate de que el servidor esté corriendo.", e),
                }
            }
            "2" => {
                match client.get(&format!("{}/gastos", base_url)).send().await {
                    Ok(resp) => {
                        let gastos: Vec<Gasto> = resp.json().await.unwrap_or_default();
                        if gastos.is_empty() {
                            println!("No hay gastos registrados.");
                        } else {
                            println!("\nResumen de gastos:");
                            let total: f64 = gastos.iter().map(|g| g.monto).sum();
                            for gasto in &gastos {
                                println!(
                                    "- {}: ${:.2} ({:?})",
                                    gasto.descripcion, gasto.monto, gasto.categoria
                                );
                            }
                            println!("Total gastado: ${:.2}", total);
                        }
                    }
                    Err(e) => println!("Error al obtener gastos: {}. Asegúrate de que el servidor esté corriendo.", e),
                }
            }
            "3" => {
                println!("Ingrese la categoría (comida, transporte, entretenimiento, alimentos, otros):");
                let mut cat = String::new();
                std::io::stdin()
                    .read_line(&mut cat)
                    .expect("Error al leer la entrada");
                let cat = cat.trim().to_lowercase();

                match client
                    .get(&format!("{}/gastos/{}", base_url, cat))
                    .send()
                    .await
                {
                    Ok(resp) => {
                        let gastos: Vec<Gasto> = resp.json().await.unwrap_or_default();
                        if gastos.is_empty() {
                            println!("No hay gastos en la categoría {}.", cat);
                        } else {
                            println!("\nGastos filtrados por {}:", cat);
                            for gasto in gastos {
                                println!(
                                    "- {}: ${:.2} ({:?})",
                                    gasto.descripcion, gasto.monto, gasto.categoria
                                );
                            }
                        }
                    }
                    Err(e) => println!("Error al filtrar gastos: {}. Asegúrate de que el servidor esté corriendo.", e),
                }
            }
            "4" => {
                println!("Saliendo...");
                break;
            }
            _ => println!("Opción inválida, por favor intenta de nuevo."),
        }
    }
}



#[tokio::main]
async fn main() {
    // Base de datos en memoria
  // Supabase 

  let supabase_url = env::var("SUPABASE_URL").expect("SUPABASE_URL no encontrada");
    let supabase_key = env::var("SUPABASE_KEY").expect("SUPABASE_KEY no encontrada");
  let client = SupabaseClient::new(supabase_url,supabase_key);
  let base_datos: BD = Arc::new(Mutex::new(Vec::new()));

    // Configurar CORS
    let cors = CorsLayer::new()
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::OPTIONS,
        ])
        .allow_origin("http://localhost:3000".parse::<axum::http::HeaderValue>().unwrap())
        .allow_headers([CONTENT_TYPE]);

    // Configurar rutas de la API
    let app = Router::new()
        .route("/", get(|| async { "¡Servidor de Control de Gastos funcionando!" }))
        .route("/gastos", get(obtener_gastos))
        .route("/gasto", post(agregar_gasto))
        .route("/gastos/:categoria", get(gastos_por_categoria))
        .layer(cors)
        .with_state(base_datos.clone());

        // Iniciar servidor en una tarea separada
let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
println!("Servidor corriendo en http://{}", addr);


let server = tokio::spawn(async move {
    if let Err(e) = serve(
        tokio::net::TcpListener::bind(&addr).await.unwrap(),
        app.into_make_service(),
    )
    .await
    {
        eprintln!("Error en el servidor: {}", e);
    }
});


    // Esperar un momento para asegurarnos de que el servidor esté listo
    sleep(Duration::from_millis(100)).await;

    // Iniciar consola
    consola().await;

    // Terminar servidor al salir de la consola
    server.abort();
}
