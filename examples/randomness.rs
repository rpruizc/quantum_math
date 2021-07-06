use rand::{thread_rng, Rng};
use rand::distributions::{Alphanumeric, Distribution, Uniform};
use rand_distr::{Normal, NormalError};

// Random numbers
fn main() -> Result<(), NormalError> {
    let mut rng = thread_rng();

    // Generating random numbers
    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("========= Random Numbers =========");
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());

    // Generating random numbers within a range
    println!("========= Random Numbers within a Range =========");
    println!("Integer: {}", rng.gen_range(0..10));
    println!("Float: {}", rng.gen_range(0.0..10.0));

    // Uniform obtains values with a uniform distribution
    // It's faster generation when generating numbers in the same range
    println!("========= Random Numbers using a Uniform Distribution =========");
    let die = Uniform::from(1..7);

    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {}", throw);
        if throw == 1 {
            break;
        }
    }

    // Using other distributions using rand_distr
    println!("========= Random Numbers using a Normal Distribution =========");
    let normal = Normal::new(2.0, 3.0)?;
    let v = normal.sample(&mut rng);
    println!("{} is from a N(2, 9) distribution", v);

    // Randomly generating strings of given length
    println!("========= Random password from a set of alphanumeric characters =========");
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    println!("Generated password: {}", rand_string);

    // Random password from a set of defined characters
    println!("========= Password from a defined set =========");
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            01234567890\
                            !@#$%^&*()";
    const PASS_LENGTH: usize = 30;

    let password: String = (0..PASS_LENGTH)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    println!("Generated custom charset password: {:?}", password);

    Ok(())
}
