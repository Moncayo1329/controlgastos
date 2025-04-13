import React, { useEffect, useState } from "react";
import axios from 'axios'
const API_URL = 'http://127.0.0.1:4000'; // URL del backend

const Gastos = () => {
  const [gastos, setGastos ] = useState([])
  const [descripcion, setDescripcion] = useState("");
    const [monto,setMonto] = useState("");
    const [categoria, setCategoria] = useState("");
 const [filtroCategoria, setFiltroCategoria] = useState('')


useEffect(() => {
  fetchGastos();
},[]);


// Funcion para Obtener gastos desde el backend

const fetchGastos = async (categoria = '') => {

try {
const url = categoria ? ` ${API_URL}/gastos/${categoria}` : `${API_URL}/gastos`;
const response = await axios.get(url);
setGastos(response.data);
} catch(error) {
console.error('Error al obtener gastos', error);
alert('No se pudieron cargar los gastos. Asegurate de que el backend este corriendo.')

}
};

// Funcion para agregar UN GASTO. 
const agregarGasto = async (e) => {
  e.preventDefault();
  try{
    await axios.post(`${API_URL}/gasto`,{
      descripcion,
      monto:parseFloat(monto),
      categoria,
    });
    alert('Gasto agregado');
    setDescripcion('');
    setMonto('');
    fetchGastos(); // Actualizar la lista de gastos
  } catch (error) {
    console.error('Error al agregar gasto:', error);
    alert('No se pudo agregar el gasto.');
  }
  };

  // Función para filtrar por categoría
  const filtrarGastos = () => {
    fetchGastos(filtroCategoria.toLowerCase());
  };





// Funcion para el campo de monto como dinero
const formatMoney = (value) => {
    // Eliminar cualquier carácter no numérico
    const numericValue = value.replace(/[^0-9.]/g, "");

// Formatear como moneda (puedes ajustar según tu región)
return new Intl.NumberFormat("es-ES", {
    style: "currency",
    currency: "EUR", // Cambia a tu moneda preferida (USD, MXN, etc.)
  }).format(numericValue || 0);
};

// Función para manejar el cambio en el campo de monto
const handleMontoChange = (e) => {
  const inputValue = e.target.value;
  setMonto(formatMoney(inputValue));
};







const handleSubmit = (e) => {
    e.preventDefault();
    console.log("Datos ingresados:");
    console.log("Descripción:", descripcion);
    console.log("Monto:", monto);
    console.log("Categoría:", categoria);
  };


  
  return (
    <form onSubmit={handleSubmit}>
    <div style={{ display: 'flex', flexDirection: 'column', gap: '20px', padding: '20px' }}>
        <div style={{ display: 'flex', gap: '16px' }}>
            {/* Campo de Descripción */}
          <div style={boxStyle}>
            <p><strong>Descripción:</strong></p>
            <form onSubmit={agregarGasto}></form>
          <input 
            type="text"
            placeholder="Escribe una descripcion"
            value={descripcion}
            onChange={(e) => setDescripcion(e.target.value)}
            required            
            style={{ padding: "5px", width: "100%" }}
            />
          </div>
           {/* Campo de monto */}
          <div style={boxStyle}>
            <p><strong>Monto:</strong></p>
            <input
          type="text"
          placeholder="Escribe el monto"
          value={monto}
          onChange={(e) => setMonto(e.target.value)}
         required
          style={{ padding: "5px", width: "100%" }}
        />
          </div>
            {/* Campo de categoria */}
          <div style={boxStyle}>
            <p><strong>Categoría:</strong></p>
            <input 
            type="texto"
            placeholder="Escribe la categoria"
            value={categoria}
            onChange={(e) => setCategoria(e.target.value)}
            style={{padding: "5px", width:"100%"}}
            
            />
          </div>
        </div>
        </form>
    </div>
    
    
  );
};
// Reutilizamos el estilo de cada caja para mantenerlas iguales
const boxStyle = {
  border: '1px solid #ccc',
  borderRadius: '10px',
  padding: '16px',
  width: '250px',
  boxShadow: '0 4px 8px rgba(29, 29, 29, 0.1)',
  backgroundColor: '#fff'
};

export default Gastos;
