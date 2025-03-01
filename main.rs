use std::fs;

fn print(letter: Vec<Vec<i8>>) {
    for i in letter {
	for j in i {
	    if j == 1 {
		print!("#");
		continue;
	    }	    
	    print!(".");
	}
	println!("");
    }
}

fn read_csv(path: &str) -> Vec<Vec<i8>> {
    fs::read_to_string(path)
	.unwrap()
	.lines()
	.map(|line| line
	     .split(",")
	     .map(|digit| digit
		  .parse::<i8>()
		  .unwrap()
	     )
	     .collect::<Vec<_>>()
	)
	.collect::<Vec<_>>()
}

struct Perceptron {
    bias: Vec<i8>,
    learning_rate: i8,
    threshold: i8,
    weights: Vec<Vec<i8>>
}

impl Perceptron {
    fn new() -> Self {
        return Self {
            bias: Vec::new(),
            learning_rate: 1,
            threshold: 0,
	    weights: Vec::new()
        };
    }

    fn activate(&self, x: i8) -> i8 {
        match x {
            a if a > 0 => 1,
	    a if a >= -self.threshold && a <= self.threshold => 0,
	    _ => -1,
        }
    }

    fn dot(&self, u: &Vec<i8>, v: &Vec<i8>) -> i8 {
	u.into_iter()
            .zip(v.iter())
            .map(|(a, b)| a * b)
            .sum::<_>()
    }

    fn predict(&self, inputs: &Vec<i8>) -> Vec<i8> {
	(0 .. self.bias.len())
	    .map(|i| {
		self.activate(
		    self.bias[i] + self.dot(inputs, &self.weights[i])
		)
	    })
	    .collect::<Vec<_>>()
    }

    fn train(&mut self, training_input: Vec<Vec<i8>>, targets: Vec<Vec<i8>>) -> usize {
	self.bias = targets[0]
	    .iter()
	    .map(|_| 0)
	    .collect::<Vec<_>>();
	
	self.weights = self.bias
	    .iter()
	    .map(|_| {
		training_input[0]
		    .iter()
		    .map(|_| 0)
		    .collect::<Vec<_>>()
	    })
	    .collect::<Vec<_>>();

	let mut epochs: usize = 0;
	let mut w: Vec<i8>;
	let mut y_in: Vec<i8>;

	for i in 0 .. self.bias.len() {
	    loop {
		w = self.weights[i].clone();
		for (current_input, current_target) in training_input.iter().zip(targets.clone()) {
		    epochs += 1;
		    y_in = self.predict(&current_input);

		    if y_in[i] != current_target[i] {
			for (j, x) in current_input.iter().enumerate() {
			    self.weights[i][j] += self.learning_rate * current_target[i] * x;
			    self.bias[i] += self.learning_rate * current_target[i];
			}
		    }
		}

		if w == self.weights[i] {
		    break;
		}
	    }
	}
	
	return epochs;
    }
}

fn main() {
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
	println!("Prediccion para {:?}: {:?}", a, perceptron.predict(&a));
    }

    /*
    let mut csv_s: Vec<Vec<Vec<i8>>> = Vec::new();
    for path in fs::read_dir("learning").unwrap() {
	csv_s.push(
	    read_csv(&path
		     .unwrap()
		     .path()
		     .display()
		     .to_string()
	    )
	);
	println!("");
    }
    */
}
