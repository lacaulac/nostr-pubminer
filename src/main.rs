use std::{env, sync::mpsc::channel, sync::mpsc::Sender, fs::File, fs::OpenOptions, io::Write, thread::sleep, time::Duration};
use secp256k1::{KeyPair, XOnlyPublicKey};

fn run_thread(sender: Sender<KeyPair>) {
    let secp = secp256k1::Secp256k1::new();
    let mut rng = rand::rngs::OsRng::default();
    loop {
        let secret_key = secp256k1::SecretKey::new(&mut rng);
        sender.send(KeyPair::from_secret_key(&secp, &secret_key)).unwrap();
    }
}

fn filter_pubkeys(pubkey: String, filter: &str) -> bool {
    pubkey.starts_with(filter)
}

fn main() {
    if env::args().len() != 3 {
        println!("Usage: {} <filter> <threadAmount>", env::args().nth(0).unwrap());
        println!("\t Benchmark with \"benchmark\" as filter and threadAmount as the amount of iterations");
        return;
    }
    let filter_string = env::args().nth(1).unwrap();
    let thread_amount = env::args().nth(2).unwrap().parse::<u32>().unwrap();

    if filter_string == "benchmark" {
        run_benchmark(thread_amount as u128);
        return;
    }

    println!("Thread amount: {}", thread_amount);
    let (sender, receiver) = channel();
    let mut senders: Vec<Sender<KeyPair>> = Vec::new();
    for _ in 1..thread_amount {
        senders.push(sender.clone());
    }
    senders.insert(0, sender);
    
    //Create the threads and run them
    let mut threads = Vec::new();
    for i in 0..thread_amount {
        println!("Starting thread {}", i);
        let new_sender = senders.pop().unwrap();
        threads.push(std::thread::spawn(move || {
            run_thread(new_sender);
        }));
    }

    let mut output_file: File;
    match File::open("output.csv") {
        Ok(_) => {
            output_file = OpenOptions::new().write(true).append(true).open("output.csv").unwrap();
        },
        Err(_) => {
            output_file = File::create("output.csv").unwrap();
        }
    }
    //Get the results and write them
    let mut result_amount = 0;
    loop {
        let new_result = receiver.recv();
        match new_result {
            Ok(result) => {
                let (pubkey_readable, _) = XOnlyPublicKey::from_keypair(&result);
                if !filter_pubkeys(pubkey_readable.to_string(), filter_string.as_str()) {
                    continue;
                }
                let tmp_output = format!("{};{}\n", result.display_secret(), pubkey_readable);
                result_amount += 1;
                println!("{} calculated keys...", result_amount);
                output_file.write_all(tmp_output.as_bytes()).unwrap();
            },
            Err(_) => {
                sleep(Duration::from_secs(1));
            }
        }
        
    }
}

fn run_benchmark(amount_of_tries: u128) {
    let (sender, receiver) = channel();
        use std::time::Instant;
        let filter_string = String::from("impossible");

        let mut start: Instant;
        let mut total_generation_time: u128 = 0;
        let mut total_filtering_time: u128 = 0;

        let secp = secp256k1::Secp256k1::new();
        let mut rng = rand::rngs::OsRng::default();
        for _ in 0..amount_of_tries {
            start = Instant::now();
            let secret_key = secp256k1::SecretKey::new(&mut rng);
            sender.send(KeyPair::from_secret_key(&secp, &secret_key)).unwrap();
            total_generation_time += start.elapsed().as_micros();
        }

        let time_for_generation = total_generation_time / amount_of_tries;
        

        for _ in 0..amount_of_tries {
            start = Instant::now();
            let new_result = receiver.recv();
            match new_result {
                Ok(result) => {
                    let (pubkey_readable, _) = XOnlyPublicKey::from_keypair(&result);
                    if !filter_pubkeys(pubkey_readable.to_string(), filter_string.as_str()) {
                        
                    }
                    else {
                        let tmp_output = format!("{};{}\n", result.display_secret(), pubkey_readable);
                    }
                },
                Err(_) => {
                    sleep(Duration::from_secs(1));
                }
            }
            total_filtering_time += start.elapsed().as_micros();
        }

        let time_for_filtering = total_filtering_time / amount_of_tries;

        
        println!("Time for generation: {}µs ({}/{})", time_for_generation, total_generation_time, amount_of_tries);
        println!("\t{} h/s", 1000000 / time_for_generation);
        println!("\t\t{} h/s on 12 cores", (1000000 / time_for_generation) * 12);
        println!("Time for filtering: {}µs ({}/{})", time_for_filtering, total_filtering_time, amount_of_tries);
        return;
}