use rand::Rng;

fn print_buffer(bfr: &Vec<f32>) {
    for i in bfr.iter() {
        println!("{}", i);
    }
}

fn rand_fill(bfr: &mut Vec<f32>) {
    let mut range = rand::thread_rng();
    for i in bfr.iter_mut() {
        let random_float: f32 = range.gen_range(-1.0..1.0);
        *i = random_float;
    }
}
fn main() {
    const SAMPLE_RATE: usize = 10;
    let initial_buffer: [f32; SAMPLE_RATE] = [0.0; SAMPLE_RATE];
    let mut buffer: Vec<f32> = Vec::from(initial_buffer);  
    print_buffer(&buffer); 
    rand_fill(&mut buffer);
    print_buffer(&buffer);
}