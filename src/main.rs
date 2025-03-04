mod perceptron;
mod letter;

use getch_rs::{Getch, Key};
use perceptron::perceptron::Perceptron;
use letter::letter::read_csv;
use letter::letter::print;
use letter::letter::read_csv_letter;
use letter::letter::flatten;

fn next(g: &Getch) {
    println!("Presionar Enter para continuar.");
    loop {
        match g.getch() {
            Ok(Key::Char('\r')) => break,
            Ok(_) => (),
            Err(e) => println!("{}", e),
        }
    }
}

fn main() {
    /*
   let training_input = vec! [
	vec! [-1, 1, 1, -1, -1, -1, -1], // 1
	vec! [1, 1, -1, 1, 1, -1, 1],    // 2
	vec! [1, 1, 1, 1, -1, -1, 1],    // 3
	vec! [-1, 1, 1, -1, -1, 1, 1],   // 4
	vec! [1, -1, 1, 1, -1, 1, 1],    // 5
	vec! [1, -1, 1, 1, 1, 1, 1],     // 6
	vec! [1, 1, 1, -1, -1, -1, -1],  // 7
	vec! [1, 1, 1, 1, 1, 1, 1],      // 8
	vec! [1, 1, 1, -1, -1, 1, 1],    // 9
	vec! [1, 1, 1, 1, 1, 1, -1]      // 0
    ];

    let targets = vec! [
	vec! [-1, -1, -1], // 1
	vec! [1, -1, -1],  // 2
	vec! [-1, 1, -1],  // 3
	vec! [1, -1, -1],  // 4
	vec! [-1, 1, -1],  // 5
	vec! [1, -1, 1],   // 6
	vec! [-1, 1, 1],   // 7
	vec! [1, -1, 1],   // 8
	vec! [-1, -1, 1],  // 9
	vec! [1, -1, -1]   // 0
    ];

    let mut perceptron = Perceptron::new();

    println!("Numero de epocas: {}", perceptron.train(training_input.clone(), targets));

    for a in training_input {
	println!("Prediccion para {:?}: {:?}", a.clone(), perceptron.predict(&a));
}*/
    
    let mut csv_s: Vec<(Vec<Vec<i16>>, Vec<i16>)> = Vec::new();
    for path in std::fs::read_dir("src/learning").unwrap() {
	csv_s.push(	    
	    read_csv(&path
		     .unwrap()
		     .path()
		     .display()
		     .to_string()
	    )
	);
    }

    let mut training_input: Vec<Vec<i16>> = Vec::new();
    let mut targets: Vec<Vec<i16>> = Vec::new();

    csv_s.iter().for_each(|(input, target)| {
	training_input.push(input.clone().into_iter().flatten().collect());
	targets.push(target.clone());
    });
    
    let mut perceptron = Perceptron::new();
    println!("Modelo entrenado.\nNumero de epocas: {}", perceptron.train(training_input.clone(), targets));

    let g = Getch::new();
    
    next(&g);

    let mut result: Vec<i16>;
    let mut prediction: Vec<Vec<i16>>;
    
    // TRAINING INPUTS
    for (a, _) in csv_s {
	println!("Prediccion para");
	print(&a);

	result = perceptron.predict(&flatten(a));
	prediction = vec! [
	    (0 .. 7).map(|i| { result[i] }).collect::<Vec<_>>(),
	    (7 .. 10).map(|i| { result[i] }).collect::<Vec<_>>()
	];

	println!("{:?}\n", prediction);
    }

    next(&g);

    // TEST INPUTS
    let mut letters: Vec<Vec<Vec<i16>>> = Vec::new();

    for path in std::fs::read_dir("src/test").unwrap() {
	letters.push(	    
	    read_csv_letter(&path
			    .unwrap()
			    .path()
			    .display()
			    .to_string()
	    )
	);
    }

    let mut stats: Vec<i8> = vec! [0, 0, 0, 0, 0]; // 0: matched, 1: letter_matched, 2: font_matched, 3: not matched, 4: errors
    let mut f: i16;
    let mut l: i16;

    for a in letters {
	println!("Prediccion para");
	print(&a);

	result = perceptron.predict(&flatten(a));
	
	prediction = vec! [
	    (0 .. 7).map(|i| { result[i] }).collect::<Vec<_>>(),
	    (7 .. 10).map(|i| { result[i] }).collect::<Vec<_>>()
	];

	l = prediction[0].iter().sum::<_>();
	f = prediction[1].iter().sum::<_>();
	match l + f {
	    -8 => stats[0] += 1,
	    -10 => stats[3] += 1,
	    -9 => match f {
		-7 => stats[2] += 1,
		_ => stats[1] += 1
	    },
	    _ => stats[4] += 1
	}

	println!("{:?}\n", prediction);	
    }

    print!("Fuente y letra identificada: {}\nSolo letra identificada: {}\nSolo fuente identificada: {}\nNo identificada: {}\nErrores: {}", stats[0], stats[1], stats[2], stats[3], stats[4]);
}
