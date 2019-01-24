mod file_reader;
mod print_utils;
mod graph_generator;
mod genetic_algorithm;

use std::io;

fn main() {
    eprintln!();
    eprintln!("Projektowanie Efektywnych Algorytmów - Zadanie 3");
    eprintln!("Problem komiwojażera (TSP)");
    eprintln!("Karol Kłusek, 235082");

    //Zmienna przechowująca graf w postaci macierzowej
    let mut matrix: Vec<Vec<i32>> = Vec::new();

    loop {
        let mut selected_number = String::new();

        eprintln!();
        eprintln!("Wybierz funkcję:");
        eprintln!("1. Wczytaj plik z grafem");
        //eprintln!("10. Wczytaj plik rbg323.atsp");
        eprintln!("2. Wygeneruj losowy graf");
        eprintln!("3. Algorytm genetyczny");
        //eprintln!("30. Algorytm genetyczny ze specyficznymi parametrami");
        eprintln!("0. Wyjście");
        eprintln!("Wybór: ");

        //Wczytaj odpowiedź użytkownika
        io::stdin().read_line(&mut selected_number).expect(
            "Błąd wejścia/wyjścia",
        );

        //Zmień typ odpowiedzi na integer
        let selected_number: u32 = selected_number.trim().parse().expect("Błędna wartość");

        //Podejmij akcję w zalezności od wyboru użytkownika
        match selected_number {
            0 => std::process::exit(0),
            1 => {
                let mut file_name = String::new();
                eprintln!("Nazwa pliku: ");
                io::stdin().read_line(&mut file_name).expect(
                    "Błąd wejścia/wyjścia",
                );

                matrix = file_reader::read_any_file(String::from(file_name.trim()));

                print_utils::print_matrix(&matrix);
            }
            2 => {
                let mut number_of_nodes: String = String::new();
                eprintln!("Ilość wierzchołków:");
                io::stdin().read_line(&mut number_of_nodes).expect(
                    "Błąd wejścia/wyjścia",
                );

                let number_of_nodes: i32 =
                    number_of_nodes.trim().parse().expect("Błędna wartość");

                matrix = graph_generator::generate_random_graph(number_of_nodes);
                print_utils::print_matrix(&matrix);
            }
            3 => {
                if matrix.is_empty() {
                    eprintln!("Najpierw wczytaj graf z pliku!");
                } else {
                    solve_genetic(&mut matrix)
                }
            }

            10 => {
                let mut file_name = "data/rbg323.atsp";
//                let mut file_name = "data/br17.atsp";

                matrix = file_reader::read_any_file(String::from(file_name.trim()));
                eprintln!("Wczytano plik data/rbg323.atsp ");
            }

            30 => {
                // iteracje, rozm. popul., ilość par dzieci, prawd. mut., prawd. krzyż., czas, typ kros., typ mut.

                matrix = file_reader::read_any_file("data/ftv47.atsp".to_string());

                solve_genetic_now(&mut matrix, 1000, 50, 25, 0.01f32, 0.8f32, 120, 1, 1); eprintln!("1/12");
                solve_genetic_now(&mut matrix, 1000, 50, 25, 0.01f32, 0.8f32, 120, 1, 2); eprintln!("2/12");
                solve_genetic_now(&mut matrix, 1000, 50, 25, 0.01f32, 0.8f32, 120, 2, 1); eprintln!("3/12");
                solve_genetic_now(&mut matrix, 1000, 50, 25, 0.01f32, 0.8f32, 120, 2, 2); eprintln!("4/12");
                solve_genetic_now(&mut matrix, 1000, 100, 50, 0.01f32, 0.8f32, 120, 1, 1); eprintln!("5/12");
                solve_genetic_now(&mut matrix, 1000, 100, 50, 0.01f32, 0.8f32, 120, 1, 2); eprintln!("6/12");
                solve_genetic_now(&mut matrix, 1000, 100, 50, 0.01f32, 0.8f32, 120, 2, 1); eprintln!("7/12");
                solve_genetic_now(&mut matrix, 1000, 100, 50, 0.01f32, 0.8f32, 120, 2, 2); eprintln!("8/12");
                solve_genetic_now(&mut matrix, 1000, 150, 75, 0.01f32, 0.8f32, 120, 1, 1); eprintln!("9/12");
                solve_genetic_now(&mut matrix, 1000, 150, 75, 0.01f32, 0.8f32, 120, 1, 2); eprintln!("10/12");
                solve_genetic_now(&mut matrix, 1000, 150, 75, 0.01f32, 0.8f32, 120, 2, 1); eprintln!("11/12");
                solve_genetic_now(&mut matrix, 1000, 150, 75, 0.01f32, 0.8f32, 120, 2, 2); eprintln!("12/12");
            }

            _ => eprintln!("Niepoprawna wartość!"),
        }
    }

    fn solve_genetic(matrix: &mut Vec<Vec<i32>>) {

        // Iteracje
        eprintln!("Iteracje:");
        let mut iterations: String = String::new();
        io::stdin().read_line(&mut iterations).expect(
            "Błąd wejścia/wyjścia",
        );
        let iterations: i32 = iterations.trim().parse().expect("Błędna wartość");

        // Ilość elementów populacji
        eprintln!("Rozmiar populacji:");
        let mut population_size: String = String::new();
        io::stdin().read_line(&mut population_size).expect(
            "Błąd wejścia/wyjścia",
        );
        let population_size: i32 = population_size.trim().parse().expect("Błędna wartość");

        // Ilość par dzieci
        eprintln!("Ilość par dzieci:");
        let mut children_pairs_size: String = String::new();
        io::stdin().read_line(&mut children_pairs_size).expect(
            "Błąd wejścia/wyjścia",
        );
        let children_pairs_size: i32 = children_pairs_size.trim().parse().expect(
            "Błędna wartość",
        );

        // Prawdopodobieństwo mutacji
        eprintln!("Prawdopodobieństwo mutacji:");
        let mut mutation_probability: String = String::new();
        io::stdin().read_line(&mut mutation_probability).expect(
            "Błąd wejścia/wyjścia",
        );
        let mutation_probability: f32 = mutation_probability.trim().parse().expect(
            "Błędna wartość",
        );

        // Prawdopodobieństwo krzyżowania
        eprintln!("Prawdopodobieństwo krzyżowania:");
        let mut crossing_probability: String = String::new();
        io::stdin().read_line(&mut crossing_probability).expect(
            "Błąd wejścia/wyjścia",
        );
        let crossing_probability: f32 = crossing_probability.trim().parse().expect(
            "Błędna wartość",
        );

        // Maksymalny czas
        eprintln!("Maksymalny czas:");
        let mut max_time: String = String::new();
        io::stdin().read_line(&mut max_time).expect(
            "Błąd wejścia/wyjścia",
        );
        let max_time: i32 = max_time.trim().parse().expect("Błędna wartość");

        // typ krzyzowania
        let mut crossing_type_integer: String = String::new();
        loop {
            eprintln!("Typ krzyzowania:");
            eprintln!("[1] OX");
            eprintln!("[2] EX");
            io::stdin().read_line(&mut crossing_type_integer).expect(
                "Błąd wejścia/wyjścia",
            );
            // jesli uzytkownik wprowadzi prawidlowy parametr
            if crossing_type_integer.trim() == "1"
                || crossing_type_integer.trim() == "2"
            {
                break;
            }
            // ...jesli nie
            print!("Bledny typ mutacji 😡");
        }

        let crossing_type_integer: i32 = crossing_type_integer.trim().parse().expect(
            "Błędna wartość"
        );

        // typ mutowania
        let mut mutation_type_integer: String = String::new();
        loop {
            eprintln!("Typ mutacji:");
            eprintln!("[1] SWAP");
            //eprintln!("[10] GREAT SWAP");
            eprintln!("[2] INSERT");
            //eprintln!("[20] GREAT INSERT");
            io::stdin().read_line(&mut mutation_type_integer).expect(
                "Błąd wejścia/wyjścia",
            );
            if mutation_type_integer.trim() == "1"
               || mutation_type_integer.trim() == "2"
               || mutation_type_integer.trim() == "10"
               || mutation_type_integer.trim() == "20"
            {
                break;
            }
            // ...jesli nie
            print!("Bledny typ mutacji 😡");
        }

        let mutation_type_integer: i32 = mutation_type_integer.trim().parse().expect(
            "Błędna wartość"
        );

        // Rozwiązanie z parametrami
        genetic_algorithm::solve(
            matrix,
            iterations,
            population_size,
            children_pairs_size,
            mutation_probability,
            crossing_probability,
            max_time,
            crossing_type_integer,
            mutation_type_integer
        )
    }

    fn solve_genetic_now(matrix: & Vec<Vec<i32>>,
                         iterations: i32,
                         population_size: i32,
                         children_pairs_size: i32,
                         mutation_probability: f32,
                         crossing_probability: f32,
                         max_time: i32,
                         crossing_type_integer: i32,
                         mutation_type_integer: i32)
    {
        // eprintln!("Wykonywanie algorytmu dla parametrow.
        //  iteracje: {0}
        //  populacja: {1}
        //  pary dzieci: {2}
        //  szansa mutacji: {3}
        //  szansa krzyżowania: {4}
        //  czas: {5}
        //  typ krzyzowania: {6}
        //  typ mutowania: {7}",
        //          iterations,
        //          population_size,
        //          children_pairs_size,
        //          mutation_probability,
        //          crossing_probability,
        //          max_time,
        //          crossing_type_integer,
        //          mutation_type_integer);

        genetic_algorithm::solve(
            matrix,
            iterations,
            population_size,
            children_pairs_size,
            mutation_probability,
            crossing_probability,
            max_time,
            crossing_type_integer,
            mutation_type_integer
        );

        
    }
}
