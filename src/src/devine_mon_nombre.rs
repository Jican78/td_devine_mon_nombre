fn main() {
    println!("Devine mon nombre !\n");
    println!("Saisissez votre proposition.");
    let mut input  = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    //let input: u32 = input.trim().parse().unwrap(); //pourquoi on fait ca
    println!("Votre nombre : {}", input.trim());

}

//crates.io
//supp src
//compil: cargo build
//faire rustling