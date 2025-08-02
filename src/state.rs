use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Importamos o modelo de dados que definimos
use crate::models::DataEntry;

// AppState é o estado global da aplicação.
// Usamos Arc<Mutex<...>> para permitir acesso seguro entre múltiplas requisições.
pub type AppState = Arc<Mutex<HashMap<u32, DataEntry>>>;

// Cria um novo estado vazio
pub fn new_state() -> AppState {
    Arc::new(Mutex::new(HashMap::new()))
}