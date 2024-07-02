use std::io::{self, Write};

fn main() {
    let enigme = "Je suis le début de la fin, et la fin du temps et de l'espace. Je suis essentiel à la création, et je suis partout. Qui suis-je ?";
    let reponse_correcte = "La lettre e";
    let mut essais = 0;

    loop {
        println!("{}", enigme);

        let mut reponse = String::new();
        print!("Votre réponse : ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut reponse).expect("Échec de la lecture de la ligne");
        let reponse = reponse.trim();

        essais += 1;

        if reponse == reponse_correcte {
            println!("Nombre d'essais : {}", essais);
            break;
        } else {
            println!("Incorrect, essayez encore.");
        }
    }
}
