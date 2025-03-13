pub mod letter {
    pub fn print(letter: &Vec<Vec<i16>>) -> () {
	letter.iter()
	    .for_each(|v| {
		v.iter()
		    .for_each(|digit| {
			match digit {
			    1 => print!("#"),
			    _ => print!(".")
			}
		    });
		println!();
	    })
    }

    pub fn flatten<T>(nested: Vec<Vec<T>>) -> Vec<T> {
	nested.into_iter()
	    .flatten()
	    .collect()
    }

    pub fn read_csv_letter(path: &str) -> Vec<Vec<i16>> {
	std::fs::read_to_string(path)
	    .unwrap()
	    .lines()
	    .map(|line| {
		line.split(",")
		    .map(|digit| digit.parse::<i16>().unwrap())
		    .collect::<Vec<_>>()
	    })
	    .collect::<Vec<_>>()
    }

    pub fn read_csv(path: &str) -> (Vec<Vec<i16>>, Vec<i16>) {
	let s = if let Ok(content) = std::fs::read_to_string(path) {
	    content
	} else {
	    String::new()
	};

	assert_ne!(s.len(), 0);

	let letter_target = s.split("\n\n").collect::<Vec<_>>();
	
	(
	    letter_target[0]
		.lines()
		.map(|line| {
		    line.split(",")
			.map(|digit| {
			    digit
				.parse::<i16>()
				.unwrap()
			})
			.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>(),	
	    letter_target[1]
		.split(",")
		.map(|digit| {
		    digit
			.parse::<i16>()
			.unwrap()
		})
		.collect::<Vec<_>>()
	)
    }
}
