struct Persona {  
    nombre: String,  
    edad: u32,  
}  

enum Estado {  
    Activo,  
    Inactivo,  
}  

fn main() {  
    let p = Persona { nombre: String::from("Juan"), edad: 30 };  
    let estado = Estado::Activo;  

    println!("Nombre: {}, Edad: {}", p.nombre, p.edad);  
}