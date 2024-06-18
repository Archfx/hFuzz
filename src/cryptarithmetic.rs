use z3::*;
use z3::ast::Ast;
use std::collections::HashMap;
use std::io;

fn extract_unique_letters(words: Vec<&str>) -> Vec<char> {
    let mut letters = std::collections::HashSet::new();
    for word in words.iter() {
        for c in word.chars() {
            letters.insert(c);
        }
    }
    letters.into_iter().collect()
}

fn generate_variables(ctx: &Context, letters: Vec<char>) -> HashMap<char, ast::Int> {
    let mut variables = HashMap::new();
    for letter in letters {
        variables.insert(letter, ast::Int::new_const(&ctx, letter.to_string()));
    }
    variables
}

fn add_constraints(solver: &Solver, ctx: &Context, variables: &HashMap<char, ast::Int>, lhs_words: Vec<&str>, rhs_words: Vec<&str>) {
    let zero = ast::Int::from_i64(&ctx, 0);
    let one = ast::Int::from_i64(&ctx, 1);
    let nine = ast::Int::from_i64(&ctx, 9);

    for var in variables.values() {
        solver.assert(&var.ge(&zero));
        solver.assert(&var.le(&nine));
    }

    // Ensure the first character of each word is non-zero
    for word in lhs_words.iter().chain(rhs_words.iter()) {
        let first_char = word.chars().next().unwrap();
        solver.assert(&variables[&first_char].ge(&one));
    }

    // Assert distinctness of variables
    let distinct_vars: Vec<_> = variables.values().collect();
    solver.assert(&ast::Ast::distinct(ctx, &distinct_vars));

    // Convert words to Z3 expressions
    let ten = ast::Int::from_i64(&ctx, 10);
    let mut lhs_sum = ast::Int::from_i64(&ctx, 0);

    for word in lhs_words {
        let mut term = ast::Int::from_i64(&ctx, 0);
        let mut multiplier = ast::Int::from_i64(&ctx, 1);
        
        for c in word.chars().rev() {
            term = &term + &variables[&c] * &multiplier;
            multiplier = &multiplier * &ten;
        }
        lhs_sum = &lhs_sum + &term;
    }

    let mut rhs_sum = ast::Int::from_i64(&ctx, 0);

    for word in rhs_words {
        let mut term = ast::Int::from_i64(&ctx, 0);
        let mut multiplier = ast::Int::from_i64(&ctx, 1);
        
        for c in word.chars().rev() {
            term = &term + &variables[&c] * &multiplier;
            multiplier = &multiplier * &ten;
        }
        rhs_sum = &rhs_sum + &term;
    }

    solver.assert(&lhs_sum._eq(&rhs_sum));
}

fn solve_cryptarithmetic(lhs_words: Vec<&str>, rhs_words: Vec<&str>) {
    let config = Config::new();
    let ctx = Context::new(&config);
    let solver = Solver::new(&ctx);
    
    let all_words: Vec<&str> = lhs_words.iter().chain(rhs_words.iter()).cloned().collect();
    let unique_letters = extract_unique_letters(all_words);
    let variables = generate_variables(&ctx, unique_letters);
    
    add_constraints(&solver, &ctx, &variables, lhs_words.clone(), rhs_words.clone());
    
    match solver.check() {
        SatResult::Sat => {
            let model = solver.get_model().unwrap();
            let mut assignments: Vec<(char, i64)> = variables
                .iter()
                .map(|(&letter, var)| (letter, model.eval(var, true).unwrap().as_i64().unwrap()))
                .collect();
            assignments.sort_by_key(|&(letter, _)| letter);
    
            // Create a hashmap for quick lookup of the assignments
            let assignment_map: HashMap<char, i64> = assignments.into_iter().collect();
    
            // Helper function to replace letters with digits
            fn replace_letters(word: &str, map: &HashMap<char, i64>) -> String {
                word.chars()
                    .map(|c| map.get(&c).unwrap_or(&-1).to_string()) // Default to -1 if the key is not found
                    .collect::<Vec<String>>()
                    .join("")
            }
    
            // Replace letters with digits
            let lhs_digits: Vec<String> = lhs_words.iter()
                .map(|&word| replace_letters(word, &assignment_map))
                .collect();
            let rhs_digits: Vec<String> = rhs_words.iter()
                .map(|&word| replace_letters(word, &assignment_map))
                .collect();
    
            // Find the width of the largest word
            let max_len = lhs_digits.iter().chain((&rhs_digits).iter()).map(|word| word.len()).max().unwrap();
    
            // Print the words and their digits in table form
            println!("");
            for word in lhs_words.iter().chain((&rhs_words).iter()) {
                println!("{:>width$}", word, width = max_len);
            }
            println!("{:-<width$}", "", width = max_len);
            for digits in lhs_digits.iter().chain((&rhs_digits).iter()) {
                println!("{:>width$}", digits, width = max_len);
            }
        },
        _ => println!("No solution found."),
    }
    
}

pub fn maincryptarithmetic() {
    let mut lhs_input = String::new();
    let mut rhs_input = String::new();

    println!("Enter the left-hand side words (space-separated):");
    io::stdin().read_line(&mut lhs_input).expect("Failed to read line");
    let lhs_words: Vec<&str> = lhs_input.trim().split_whitespace().collect();

    println!("Enter the right-hand side words (space-separated):");
    io::stdin().read_line(&mut rhs_input).expect("Failed to read line");
    let rhs_words: Vec<&str> = rhs_input.trim().split_whitespace().collect();

    solve_cryptarithmetic(lhs_words, rhs_words);
}