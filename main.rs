use rand::Rng;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::time::Duration;
use std::thread;


fn main(){

    loop{
        let mut number_list = vec![];
        for i in 0..13{
            let mut random = ChaCha8Rng::from_entropy();
            let random1 = random.gen_range(0..=1000000);
            number_list.push(random1);
        }

        println!("Before: ");


        let mut largest = &number_list[0];

        for _i in &number_list{
            println!("{_i}");
        }

        for number in &number_list{
            if number > largest{
                largest = number;

            }
        }
        thread::sleep(Duration::from_secs(3));

        println!("\n\n\nthe highest number within that vector is: {largest}");

        thread::sleep(Duration::from_secs(1));
    }

}
