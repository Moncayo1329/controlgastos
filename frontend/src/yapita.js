import React, { useEffect, useState } from "react";
import './pagefirst.css'
import axios from 'axios';

const API_URL = 'http://127.0.0.1:4000';

const Gastos = () => {
  const [gastos, setGastos] = useState([]);
  const [descripcion, setDescripcion] = useState("");
  const [monto, setMonto] = useState("");
  const [categoria, setCategoria] = useState("");
  const [filtroCategoria, setFiltroCategoria] = useState('');

  useEffect(() => {
    fetchGastos();
  }, []);

  const fetchGastos = async (categoria = '') => {
    try {
      const url = categoria ? `${API_URL}/gastos/${categoria}` : `${API_URL}/gastos`;
      const response = await axios.get(url);
      setGastos(response.data);
    } catch (error) {
      console.error('Error al obtener gastos', error);
      alert('No se pudieron cargar los gastos. Asegúrate de que el backend esté corriendo.');
    }
  };
async function agregarGasto (e)  {
    e.preventDefault();
    const montoNumerico = parseFloat(monto.replace(/[^0-9.]/g, ""));
    if (isNaN(montoNumerico)) {
      alert("Monto inválido");
      return;
    }

const categoriasValidas = ["comida", "transporte", "entretenimiento", "alimentos", "otros"];
if (!categoriasValidas.includes(categoria)) {
  alert("Por favor, selecciona una categoría válida");
  return;
}

const gasto = {
  descripcion: descripcion.trim(),
  monto: montoNumerico,
  categoria: categoria,
};
console.log("Enviando gasto:", gasto); 


    try {
      await axios.post(`${API_URL}/gasto`, {
        descripcion,
        monto: montoNumerico,
        categoria,
      });
      alert('Gasto agregado');
      setDescripcion('');
      setMonto('');
      setCategoria('');
      fetchGastos();
    } catch (error) {
      console.error('Error al agregar gasto:', error);
      alert('No se pudo agregar el gasto.');
    }
  };

  const formatMoney = (value) => {
    const num = parseFloat(value);
    if (isNaN(num)) return "€0.00";
    return new Intl.NumberFormat("es-ES", {
      style: "currency",
      currency: "EUR",
    }).format(num);
  };

  const handleMontoChange = (e) => {
    const rawValue = e.target.value.replace(/[^0-9.]/g, "");
    setMonto(rawValue);
  };

  return (
    <form onSubmit={agregarGasto}>
      <div className="form-container" style={{ display: 'flex', flexDirection: 'column', gap: '30px', padding: '20px' }}>
        <div className="form-content d-flex flex-column gap-60"style={{ maxWidth:'400px', width:'100%' }}>
          {/* Descripción */}
          <div className="form-field"style={boxStyle}>
            <p><strong>Descripción:</strong></p>
            <input
              type="text"
              placeholder="Escribe una descripción"
              value={descripcion}
              onChange={(e) => setDescripcion(e.target.value)}
              required
              style={{ padding: "12px", width: "100%" }}
            />
          </div>

          {/* Monto */}
          <div className=" form-field" style={boxStyle}>
            <p><strong>Monto:</strong></p>
            <input
              type="text"
              placeholder="Escribe el monto"
              value={monto}
              onChange={handleMontoChange}
              required
              style={{ padding: "12px", width: "100%" }}
            />
            <small>Formateado: {formatMoney(monto)}</small>
          </div>

          {/* Categoría */}
          <div className="form-field" style={boxStyle}>
            <p><strong>Categoría:</strong></p>
            <select
              placeholder="Escribe la categoría"
              value={categoria}
              onChange={(e) => setCategoria(e.target.value)}
              style={{ padding: "12px", width: "100%" }}
            />
          </div>
        </div>

        <button type="submit" style={{ width: "150px", padding: "10px" }}>Agregar Gasto</button>
      </div>
    </form>
  );
};

const boxStyle = {
  border: '1px solid #ccc',
  borderRadius: '10px',
  padding: '20px',
  width: '250px',
  height:'170px',
  boxShadow: '0 4px 8px rgba(29, 29, 29, 0.1)',
  backgroundColor: '#fff'
};

export default Gastos;
