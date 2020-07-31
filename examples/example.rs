extern crate parallel_getter;

use parallel_getter::ParallelGetter;
use std::fs::File;

fn main() {
    let url = "http://apt.pop-os.org/proprietary/pool/bionic/main/\
        binary-amd64/a/atom/atom_1.31.1_amd64.deb";
    let mut file = File::create("atom_1.31.1_amd64.deb").unwrap();
    let result = ParallelGetter::new(url, &mut file)
        .threads(4)
        .threshold_memory(10 * 1024 * 1024)
        .callback(1000, Box::new(|p, t| {
            println!(
                "{} of {} KiB downloaded",
                p / 1024,
                t / 1024);
            
            false
        }))
        .get();

    if let Err(why) = result {
        eprintln!("errored: {}", why);
    }
}
