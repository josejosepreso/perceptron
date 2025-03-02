mod perceptron;
mod letter;

use perceptron::perceptron::Perceptron;
use letter::letter::read_csv;
use letter::letter::print;
use letter::letter::read_csv_letter;
use letter::letter::flatten;

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

    println!("Numero de epocas: {}", perceptron.train(training_input.clone(), targets));

    // TRAINING INPUTS
    let mut result: Vec<i16>;
    let mut prediction: (Vec<i16>, Vec<i16>);
    for (a, _) in csv_s {
	println!("Prediccion para");
	print(a.clone());

	result = perceptron.predict(&flatten(a));
	prediction = (
	    (0 .. 7).map(|i| result[i]).collect::<Vec<_>>(),
	    (7 .. 10).map(|i| result[i]).collect::<Vec<_>>()
	);

	println!("{:?}\n", prediction);
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

    for a in letters {
	println!("Prediccion para");
	print(a.clone());

	result = perceptron.predict(&flatten(a));
	prediction = (
	    (0 .. 7).map(|i| result[i]).collect::<Vec<_>>(),
	    (7 .. 10).map(|i| result[i]).collect::<Vec<_>>()
	);

	println!("{:?}\n", prediction);	
    }
}
