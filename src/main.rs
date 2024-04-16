fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args.len() < 3 {
        print_usage_and_exit();
    }
    let infile = args.remove(0);
    let outfile = args.remove(0);
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    while args.len() > 0 {
        let key = args.remove(0);
        println!("{} matched and args {}", key, args.join(","));
        match key.as_str() {
            "blur" => {
                if args.len() <= 0 {
                    println!("Errored blur");
                    print_usage_and_exit();
                }
                let blur_amount_str = args.remove(0);
                let blur_amount: f32 = blur_amount_str.parse().expect("Failed to parse a number");
                img = img.blur(blur_amount);
            }
            "brighten" => {
                if args.len() <= 0 {
                    println!("Errored brighten");
                    print_usage_and_exit();
                }
                let brighten_amount_str = args.remove(0);
                let brighten_amount: i32 = brighten_amount_str
                    .parse()
                    .expect("Failed to parse a number");
                img = img.brighten(brighten_amount);
            }
            "crop" => {
                if args.len() <= 3 {
                    print_usage_and_exit();
                }
                let x_str = args.remove(0);
                let y_str = args.remove(0);
                let width_str = args.remove(0);
                let height_str = args.remove(0);
                let x: u32 = x_str.parse().expect("Failed to parse x cords");
                let y: u32 = y_str.parse().expect("Failed to parse y cords");
                let width: u32 = width_str.parse().expect("Failed to parse width cords");
                let height: u32 = height_str.parse().expect("Failed to parse height cords");
                img = img.crop(x, y, width, height);
            }
            "rotate" => {
                if args.len() <= 0 {
                    print_usage_and_exit();
                }
                let degree = args.remove(0);

                match degree.as_str() {
                    "90" => {
                        img = img.rotate90();
                    }
                    "180" => {
                        img = img.rotate180();
                    }
                    "270" => {
                        img = img.rotate270();
                    }
                    _ => {
                        println!("90, 180 and 270 are the supported degree")
                    }
                }
            }
            "invert" => {
                img.invert();
            }
            "grayscale" => {
                img = img.grayscale();
            }
            _ => {
                print_usage_and_exit();
            }
        }
    }
    img.save(outfile.clone()).expect("Failed writing OUTFILE.");
}

fn print_usage_and_exit() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    println!("blur INFILE OUTFILE");
    println!("fractal OUTFILE");
    // **OPTION**
    // Print useful information about what subcommands and arguments you can use
    // println!("...");
    std::process::exit(-1);
}
