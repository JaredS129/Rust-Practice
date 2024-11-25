use rand::Rng;

fn print_buffer(bfr: &Vec<f32>) {
    for i in bfr.iter() {
        println!("{}", i);
    }
}

fn rand_fill(bfr: &Vec<f32>) {
    let mut range = rand::thread_rng();
    for i in bfr.iter() {
        // generate a random float between -1 and 1
        let random_float: f32 = range.gen_range(-1.0..1.0);
        bfr[i] = random_float;
    }
}
fn main() {
    const SAMPLE_RATE: usize = 10;
    let initial_buffer: [f32; SAMPLE_RATE] = [0.0; SAMPLE_RATE];
    let buffer: Vec<f32> = Vec::from(initial_buffer);  
    print_buffer(&buffer); 
}