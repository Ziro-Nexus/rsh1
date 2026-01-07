use regex::Regex;
use xiro::{data_types::primitive_types, memory_table::vartable::Variable};

pub fn expand_xiro_variables(raw_value: String, vtm: &mut xiro::memory_table::vartable::VariableTableInMemory) -> String {
    let mut resolved_value = raw_value.clone();
    
    // Obtenemos las variables y las ordenamos por longitud descendente
    // Esto es una regla de oro en compiladores para evitar "shadowing" parcial
    let mut vars = vtm.clone();
    vars.get_table().clone().sort_by(|a, b| b.name.len().cmp(&a.name.len()));

    for var in vars.get_table().iter() {
        let var_name = &var.name;
        
        // El patrón busca el nombre de la variable con Word Boundaries (\b)
        // Opcional: Puedes añadir que busque también si empieza por '$'
        let pattern = format!(r"(\$\b{}\b)", regex::escape(var_name));
        //let pattern = format!(r"\b{}\b", regex::escape(var_name));
        
        let re = Regex::new(&pattern).unwrap();

        let var_value = match &var.value {
            primitive_types::DataTypes::STR(s) => s.clone(), // En la shell no queremos comillas extras
            primitive_types::DataTypes::NUMBER(n) => n.to_string(),
            primitive_types::DataTypes::FLOAT(f) => f.to_string(),
            primitive_types::DataTypes::BOOL(b) => b.to_string(),
            primitive_types::DataTypes::LIST(l) => {
                let elements: Vec<String> = l.iter().map(|dt| dt.to_string()).collect();
                elements.join(" ") // En shell, las listas suelen ser strings separados por espacio
            }
            _ => var.value.to_string(),
        };

        // Reemplazo atómico
        resolved_value = re.replace_all(&resolved_value, var_value.as_str()).to_string();
    }
    
    resolved_value
}
