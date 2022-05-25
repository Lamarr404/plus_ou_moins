use std::io::stdin; /*On utilise ces librairies pour les entrées utilisateurs*/
use rand::Rng; /* On utilise ces librairies pour la génération du nombre aléatoire */

fn main() {

        let rnumber = rand::thread_rng().gen_range(1..101);  /* Génération du nombre secret aleatoire et stockage dans var rnumber */

        loop { /* Début de la boucle loop */

        println!(" JEUX DU PLUS OU MOINS ");
        println!(" Faites votre choix: ");

        let mut input_text = String::new(); /* Création d'une variable string vide mutable pour stocker l'entrée utilisateur */

        stdin() /* Appelle la fonction stdin pour récupérer l'entrée utilisateur en tant que type str */

            .read_line(&mut input_text) /* Récupération de l'entrée et stockage dans la var input_text vide précédemment créé */
            .expect("failed to read from stdin"); /* Erreur en cas de mauvaise entrée utilisateur */

        let trimmed: i32 = input_text.trim().parse().unwrap(); /* Création d'une nouvelle variable qui convertit la str de l'utilisateur en int 32 bit avec .parse() suppression des espaces avec .trim() et obliger l'entrée utilisateur avec .unwrap()*/

                if trimmed == rnumber { /* Si le choix utilisateur est strictement égale au rnumber ==> WIN + break loop */

                        println!("WIN");
                        break;

                } else if trimmed < rnumber { /* Si le choix utilisateur est inférieur au nombre aleatoire ==> Affiche + */

                        println!("+");

                } else if trimmed > rnumber { /* Si le choix utilisateur est inférieur au nombre aleatoire ==> Affiche - */

                        println!("-");

                }

             }
}
