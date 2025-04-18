// gastos.rs
use serde::{Deserialize, Serialize};
use std::fmt;


#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Categoria {
Alimentos,
Transporte,
Entretenimiento,
Comida,
Otros,

}



#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Gasto {
    pub descripcion:String,
    pub monto: f64,
    pub categoria: Categoria,

}


impl fmt::Display for Gasto {

fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
    write!(f, "{} -${:.2} ({:?})", self.descripcion, self.monto, self.categoria)
}

}


// filtrar x gastos 

pub  fn filtrar_por_categoria(gastos: &Vec<Gasto>, categoria:Categoria) -> Vec<Gasto> {
    gastos
    .iter()
    .filter(|g| g.categoria == categoria)
    .cloned()
    .collect()
}
