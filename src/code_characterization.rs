use colored::Colorize;

use crate::entropy::calculate_entropy;
use crate::parse_user_input_vec;

/// Calculates the main characterization of a code, including
/// its average length, Kraft's inequality and efficiency.
///
/// # Arguments
/// * `probabilities` - f32 Vec that holds the probabilities of the font.
/// * `lengths` - f32 Vec that holds the lengths of the code words.
///
/// # Returns
/// A f32 array with length 3 that has the average length at index 0, Kraft's
/// inequality at index 1 and efficiency at index 2.
pub fn characterization(probabilities: Vec<f32>, lengths: Vec<f32>) -> [f32; 3] {
    let entropy: f32 = calculate_entropy(probabilities);
    let mut characteristics: [f32; 3] = [0.0, 0.0, 0.0];

    // Calculating average length and Kraft's inequality
    for i in lengths.iter() {
        let value: f32 = *i;
        characteristics[0] += value;
        characteristics[1] += 1.0/2_i32.pow(value as u32) as f32;
    }
    characteristics[0] /= lengths.len() as f32;

    // Calculating efficiency
    characteristics[2] = entropy / characteristics[0];

    characteristics
}

/// Takes the user input and and calls to characterization().
/// It prints the characterization of the code or the error if
/// there was any.
pub fn input_characterization() {
    // Get the probabilities from the user input. Prints an error if it occurs.
    let probabilities = match parse_user_input_vec("Introduce las probabilidades de los símbolos(separados por comas y sin espacios):") {
        Ok(vector) => vector,
        Err(e) => {
            eprintln!("\n{}{}", "ERROR: ".red(), e.red());
            return;
        }
    };

    // Get the lengths from the user input. Prints an error if it occurs.
    let lengths = match parse_user_input_vec("Introduce las longitudes de las palabras código(separados por comas y sin espacios):") {
        Ok(vector) => vector,
        Err(e) => {
            eprintln!("\n{}{}", "ERROR: ".red(), e.red());
            return;
        }
    };

    // If probabilities is empty at this point is because there
    // was an error during the parsing. It also checks if the lengths
    // of both vectors are different. If so, we end the function.
    if lengths.is_empty() || probabilities.len() != lengths.len(){
        return;
    }

    let characteristics = characterization(probabilities, lengths);
    println!("\n{}{}", "Longitud media de palabra código(L) = ".green(), characteristics[0].to_string().green());
    println!("{}{}", "Desigualdad de Kraft(K) = ".green(), characteristics[1].to_string().green());
    println!("{}{}", "Eficiencia(n) = ".green(), characteristics[2].to_string().green());
}