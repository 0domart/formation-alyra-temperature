#[derive(Debug)]
enum CategorieTemperature {
    Froid,
    Tempere,
    Chaud
}

struct TemperatureMoyenne {
    valeur: f32,
    categorie: CategorieTemperature
}

fn main() {
    let temperatures : [f32; 7] = [22.0, 19.5, 1.0, 23.5, 20.0, 18.0, 25.0];
    let average = calculate_average(&temperatures);
    println!("Moyenne température : {:.2}°C", average);
   
    // appel a une methode pour categoriser
    let categorie_moyenne = categoriser(average);
    println!("categorie de la temp : {:?}", categorie_moyenne);
   
    match categorie_moyenne {
        CategorieTemperature::Chaud => println!("Il fait chaud"),
        CategorieTemperature::Tempere => println!("Il fait tempere"),
        CategorieTemperature::Froid => println!("Il fait froid"),
    }
   
    let temperature_moyenne = TemperatureMoyenne {
        valeur: average,
        categorie: categorie_moyenne
    };
   
    println!("categorie : {:?}", temperature_moyenne.categorie);
    println!("moyenne : {:?}", temperature_moyenne.valeur);

}

fn categoriser(moyenne: f32) -> CategorieTemperature {
    if moyenne < 10_f32 {
        CategorieTemperature::Froid
    } else if moyenne < 20.0 {
        CategorieTemperature::Tempere
    } else {
        CategorieTemperature::Chaud
    }
}

fn calculate_average(temperatures: &[f32]) -> f32 {
    let somme: f32 = temperatures.iter().sum();
    let nombre_element = temperatures.len() as f32;
    somme / nombre_element
}