use criterion::{criterion_group, criterion_main, Criterion};
use std::collections::HashMap;

// HashMap for text
struct HsMpFrTxt<'a> {
    text: &'a str,
    map: HashMap<&'a str, usize>
}

// HashMap for Integer List Exercise
struct Mode<'a> {
    numbers: &'a [i32],
    map: HashMap<i32, usize>
}

impl<'a> HsMpFrTxt<'a> {
    fn new(text: &'a str) -> Self {
        Self { 
            text, 
            map: HashMap::new()  
        }
    }

    fn original_code(&mut self) -> &HashMap<&str, usize> {
        self.map.clear();

        for num in self.text.split_whitespace() {
            let count = self.map.entry(num).or_insert(0);
            *count += 1;
        }

        println!("{:?}", self.map);
        &self.map
    }

    fn my_code(&mut self) -> &HashMap<&str, usize> {
        self.map.clear();

        for num in self.text.split_whitespace() {
            *self.map.entry(num).or_insert(0) += 1;
        }

        &self.map
    }
}

impl<'a> Mode<'a> {
    fn new(numbers: &'a [i32]) -> Self {
        Self { 
            numbers, 
            map: HashMap::new() 
        }
    } 
    
    fn original_code_median_mode_exercise(&mut self) -> Option<i32> {
        let mut frequency_map = HashMap::new();
        let mut max_frequency = 0;
        let mut mode = self.numbers[0];

        for &num in self.numbers {
            let count = frequency_map.entry(num).or_insert(0);
            *count += 1;

            if *count > max_frequency {
                max_frequency += *count;
                mode = num
            }
        }
    
        Some(mode)
    }

    fn my_code_median_mode_exercise(&mut self) -> Option<i32> {
        let mut frequency_map = HashMap::new();
        let mut max_frequency = 0;
        let mut mode = self.numbers[0];

        for &num in self.numbers {
            *frequency_map.entry(num).or_insert(0) += 1;
    
            if frequency_map[&num] > max_frequency {
                max_frequency += 1;
                mode = num
            }
        }
    
        Some(mode)
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let text = "hello world wounderful world";
    
    c.bench_function("my code", |b| b.iter(|| {
        let mut mode = HsMpFrTxt::new(text);
        mode.my_code();
    }));
    c.bench_function("original code", |b|b.iter(|| {
        let mut mode = HsMpFrTxt::new(text);
        mode.original_code();
    }));


    let numbers = [1, 2, 3, 4, 1, 5, 6, 7, 8, 12];
    
    c.bench_function("my code int list exercise", |b| b.iter(|| {
        let mut mode = Mode::new(&numbers);
        mode.my_code_median_mode_exercise();
    }));
    c.bench_function("original code int list exercise", |b| b.iter(|| {
        let mut mode = Mode::new(&numbers);
        mode.original_code_median_mode_exercise();
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);