fn main() {  
    let s1 = String::from("Hola"); // s1 es el dueño de la cadena  
    let s2 = s1; // s1 ya no es válida, s2 toma la propiedad  
    // println!("{}", s1); // Esto dará error: s1 ya no es válida  
    println!("{}", s2); // Esto funciona  
}

fn imprimir(s: &String) {  
    println!("{}", s);  
}  

fn main() {  
    let s1 = String::from("Hola");  
    imprimir(&s1); // Prestamos una referencia a s1  
}

