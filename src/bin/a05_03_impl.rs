struct Temperature {
    degrees_f: f32,
}

impl Temperature {
    fn show_temperature(& self) {
        println!("{:?} degrees F", self.degrees_f );
    }

    //The capital Self just refer to Temperature
    fn freezing() -> Self {
        Self {
            degrees_f: 32.0,
        }
    }

    fn boiling() -> Self {
        Self {degrees_f: 200.0,}
    }
}


fn main() {
    let hot = Temperature {
        degrees_f: 99.9,
    };
    hot.show_temperature();

    let cold = Temperature::freezing();
    cold.show_temperature();
}