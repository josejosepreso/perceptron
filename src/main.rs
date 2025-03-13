mod perceptron;
mod letter;

use perceptron::perceptron::Perceptron;
use letter::letter::read_csv;
use letter::letter::print;
use letter::letter::read_csv_letter;
use letter::letter::flatten;

const LETTERS: &'static [&str] = &["A", "B", "C", "D", "E", "J", "K"];

fn prediction_message(prediction: &Vec<Vec<i16>>) -> String {
    let letter_index = if let Some(index) = prediction[0].iter().position(|&n| n == 1) {
	index as i32
    } else {
	-1
    };
    
    let font_index = if let Some(index) = prediction[1].iter().position(|&n| n == 1) {
	index as i32
    } else {
	-1
    };

    if letter_index == -1 {
	return format!("{:?} -- Fuente {}", prediction, font_index + 1);
    }

    if font_index == -1 {
	return format!("{:?} -- Letra {}", prediction, LETTERS[letter_index as usize]);
    }

    return format!("{:?} -- Letra {}, fuente {}", prediction, LETTERS[letter_index as usize], font_index + 1);   
}

fn print_result(prediction: &Vec<Vec<i16>>, index: usize) -> () {
    println!("{}\n",
	     match index {
		 3 => String::from("No identificado."),
		 _ => prediction_message(prediction)
	     }
    );
}

fn check_result(letter: i16, font: i16) -> usize {
    match letter + font {
	-6 => match letter {
	    -3 => 3,
	    _ => 0
	},
	-8 => match letter {
	    -7 => 2,
	    _ => 1
	},
	_ => 3
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

    let mut result: Vec<i16>;
    let mut prediction: Vec<Vec<i16>>;
    let mut index: usize;
    
    // TRAINING INPUTS
    for (a, _) in csv_s {
	println!("Prediccion para");
	print(&a);

	result = perceptron.predict(&flatten(a));	
	prediction = vec! [
	    (0 .. 7).map(|i| { result[i] }).collect::<Vec<_>>(),
	    (7 .. 10).map(|i| { result[i] }).collect::<Vec<_>>()
	];

	index = check_result(
	    prediction[0].iter().sum::<_>(),
	    prediction[1].iter().sum::<_>()
	);

	print_result(&prediction, index);
    }

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

    let mut stats: Vec<i8> = vec! [0, 0, 0, 0]; // 0: matched, 1: letter_matched, 2: font_matched, 3: not matched

    for a in letters {
	println!("Prediccion para");
	print(&a);

	result = perceptron.predict(&flatten(a));	
	prediction = vec! [
	    (0 .. 7).map(|i| { result[i] }).collect::<Vec<_>>(),
	    (7 .. 10).map(|i| { result[i] }).collect::<Vec<_>>()
	];

	index = check_result(
	    prediction[0].iter().sum::<_>(),
	    prediction[1].iter().sum::<_>()
	);

	stats[index] += 1;

	print_result(&prediction, index);
    }

    print!("Fuente y letra identificada: {}\nSolo letra identificada: {}\nSolo fuente identificada: {}\nNo identificada: {}\n",
	   stats[0], stats[1], stats[2], stats[3]
    );
}
