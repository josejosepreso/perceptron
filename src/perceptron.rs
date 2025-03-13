pub mod perceptron {
    pub struct Perceptron {
	bias: Vec<f64>,
	learning_rate: f64,
	threshold: f64,
	pub weights: Vec<Vec<f64>>
    }
    
    impl Perceptron {
	pub fn new() -> Self {
            Self {
		bias: Vec::new(),
		learning_rate: rand::random(),
		threshold: 0.0,
		weights: Vec::new()
            }
	}
	
	fn activate(&self, x: f64) -> i16 {
            match x {
		a if a > 0.0 => 1,
		a if a >= -self.threshold && a <= self.threshold => 0,
		_ => -1,
            }
	}
	
	fn dot(&self, u: &Vec<f64>, v: &Vec<f64>) -> f64 {
	    u.iter()
		.zip(v)
		.map(|(a, b)| a * b)
		.sum::<_>()
	}
	
	pub fn predict(&self, inputs: &Vec<i16>) -> Vec<i16> {
	    (0 .. self.bias.len())
		.map(|i| {
		    self.activate(
			self.bias[i] + self.dot(&inputs.iter().map(|n| f64::from(n.clone())).collect::<Vec<_>>(), &self.weights[i])
		    )
		})
		.collect::<Vec<_>>()
	}
	
	pub fn train(&mut self, training_input: Vec<Vec<i16>>, targets: Vec<Vec<i16>>) -> usize {
	    self.bias =
		targets[0]
		.iter()
		.map(|_| 0.0)
		.collect::<Vec<_>>();
	    
	    self.weights =
		self.bias
		.iter()
		.map(|_| {
		    training_input[0]
			.iter()
			.map(|_| rand::random())
			.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();
	    
	    let mut epochs: usize = 0;
	    let mut w: Vec<f64>;
	    let mut y_in: Vec<i16>;

	    for i in 0 .. self.bias.len() {
		loop {
		    w = self.weights[i].clone();
		    for (current_input, current_target) in training_input.iter().zip(&targets) {
			// println!("Numero de epocas: {}", epochs);
			epochs += 1;			
			y_in = self.predict(&current_input);
			
			if y_in[i] != current_target[i] {
			    for (j, x) in current_input.iter().enumerate() {
				self.weights[i][j] += self.learning_rate * f64::from(current_target[i]) * f64::from(*x);
				self.bias[i] += self.learning_rate * f64::from(current_target[i]);
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
}
