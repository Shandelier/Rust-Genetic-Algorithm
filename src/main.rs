mod file_reader;
mod print_utils;
mod graph_generator;
mod genetic_algorithm;

use std::io;


fn main() {
    println!();
    println!("Projektowanie Efektywnych Algorytmów - Zadanie 3");
    println!("Problem komiwojażera (TSP)");
    println!("Karol Kłusek, 235082");

    //Zmienna przechowująca graf w postaci macierzowej
    let mut matrix: Vec<Vec<i32>> = Vec::new();

    loop {
        let mut selected_number = String::new();

        println!();
        println!("Wybierz funkcję:");
        println!("1. Wczytaj plik z grafem");
        println!("10. Wczytaj plik rbg323.atsp");
        println!("2. Wygeneruj losowy graf");
        println!("3. Algorytm genetyczny");
        println!("30. Algorytm genetyczny z parametrami: i:{0} pop:{1}\
         pary:{2} mut:{3} czas:{4} typX:{5}", 3, 20, 10, 0.01f32, 120, 1);
        println!("0. Wyjście");
        println!("Wybór: ");

        //Wczytaj odpowiedź użytkownika
        io::stdin().read_line(&mut selected_number).expect(
            "Błąd wejścia/wyjścia",
        );

        //Zmień typ odpowiedzi na integer
        let selected_number: u32 = selected_number.trim().parse().expect("Błędna wartość");

        //Podejmij akcję w zalezności od wyboru użytkownika
        match selected_number {
            0 => std::process::exit(0x0),
            1 => {
                let mut file_name = String::new();
                println!("Nazwa pliku: ");
                io::stdin().read_line(&mut file_name).expect(
                    "Błąd wejścia/wyjścia",
                );

                matrix = file_reader::read_any_file(String::from(file_name.trim()));

                print_utils::print_matrix(&matrix);
            }
            2 => {
                let mut number_of_nodes: String = String::new();
                println!("Ilość wierzchołków:");
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
                    println!("Najpierw wczytaj graf z pliku!");
                } else {
                    solve_genetic(&mut matrix)
                }
            }

            10 => {
                let mut file_name = "data/rbg323.atsp";
//                let mut file_name = "data/br17.atsp";

                matrix = file_reader::read_any_file(String::from(file_name.trim()));
                println!("Wczytano plik data/rbg323.atsp ");
            }

            30 => {
                solve_genetic_now(&mut matrix,
                                  10,
                                  50,
                                  25,
                                  0.01f32,
                                  120,
                                  2,
                                  1)
            }

            _ => println!("Niepoprawna wartość!"),
        }
    }

    fn solve_genetic(mut matrix: &mut Vec<Vec<i32>>) {

        // Iteracje
        println!("Iteracje:");
        let mut iterations: String = String::new();
        io::stdin().read_line(&mut iterations).expect(
            "Błąd wejścia/wyjścia",
        );
        let iterations: i32 = iterations.trim().parse().expect("Błędna wartość");

        // Ilość elementów populacji
        println!("Rozmiar populacji:");
        let mut population_size: String = String::new();
        io::stdin().read_line(&mut population_size).expect(
            "Błąd wejścia/wyjścia",
        );
        let population_size: i32 = population_size.trim().parse().expect("Błędna wartość");

        // Ilość par dzieci
        println!("Ilość par dzieci:");
        let mut children_pairs_size: String = String::new();
        io::stdin().read_line(&mut children_pairs_size).expect(
            "Błąd wejścia/wyjścia",
        );
        let children_pairs_size: i32 = children_pairs_size.trim().parse().expect(
            "Błędna wartość",
        );

        // Prawdopodobieństwo mutacji
        println!("Prawdopodobieństwo mutacji:");
        let mut mutation_probability: String = String::new();
        io::stdin().read_line(&mut mutation_probability).expect(
            "Błąd wejścia/wyjścia",
        );
        let mutation_probability: f32 = mutation_probability.trim().parse().expect(
            "Błędna wartość",
        );

        // Maksymalny czas
        println!("Maksymalny czas:");
        let mut max_time: String = String::new();
        io::stdin().read_line(&mut max_time).expect(
            "Błąd wejścia/wyjścia",
        );
        let max_time: i32 = max_time.trim().parse().expect("Błędna wartość");

        // typ krzyzowania
        let mut crossing_type_integer: String = String::new();
        loop {
            println!("Typ mutacji:");
            println!("[1] PMX");
            println!("[2] EX");
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
            println!("Typ mutacji:");
            println!("[1] SWAP");
            println!("[10] GREAT SWAP");
            println!("[2] INSERT");
            println!("[20] GREAT INSERT");
            io::stdin().read_line(&mut mutation_type_integer).expect(
                "Błąd wejścia/wyjścia",
            );
            if (mutation_type_integer.trim() == "1"
                || mutation_type_integer.trim() == "2"
                || mutation_type_integer.trim() == "10"
                || mutation_type_integer.trim() == "20")
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
                         max_time: i32,
                         crossing_type_integer: i32,
                         mutation_type_integer: i32)
    {
        println!("Wykonywanie algorytmu dla parametrow.\
         iteracje: {0}\
         populacja: {1}\
         pary dzieci: {2}\
         szansa mutacji: {3}\
         czas: {4}\
         typ krzyzowania: {5},
         typ mutowania: {6}",
                 iterations,
                 population_size,
                 children_pairs_size,
                 mutation_probability,
                 max_time,
                 crossing_type_integer,
                 mutation_type_integer);

        genetic_algorithm::solve(
            matrix,
            iterations,
            population_size,
            children_pairs_size,
            mutation_probability,
            max_time,
            crossing_type_integer,
            mutation_type_integer
        )
    }
}
