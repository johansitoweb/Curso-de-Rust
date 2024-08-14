fn main() {  
    let nombre = "Juan"; // Este es un string & es inmutable  
    let mut edad = 30; // mut permite que así edad sea mutable  

    println!("Hola, {}! Tienes {} años.", nombre, edad);  
    
    edad += 1; // Incrementamos la edad  
    println!("El próximo año tendrás {} años.", edad);  
}