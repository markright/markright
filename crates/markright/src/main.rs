use std::env;
use std::fs;
use std::io::{self, Read};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    #[cfg(feature = "schemars")]
    if args.iter().any(|a| a == "--schema") {
        let schema = markright::json_schema();
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
        return;
    }

    let html_mode = args.iter().any(|a| a == "--html");
    let format_mode = args.iter().any(|a| a == "--format");
    let lint_mode = args.iter().any(|a| a == "--lint");
    let file = args.iter().find(|a| !a.starts_with('-'));

    let input = match file {
        Some(path) if path != "-" => fs::read_to_string(path).unwrap_or_else(|e| {
            eprintln!("Error reading {path}: {e}");
            std::process::exit(1);
        }),
        _ => {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf).unwrap_or_else(|e| {
                eprintln!("Error reading stdin: {e}");
                std::process::exit(1);
            });
            buf
        }
    };

    let bump = markright::Bump::new();
    let doc = markright::parse(&input, &bump);

    if lint_mode {
        let lints = markright::lint(&doc);
        if lints.is_empty() {
            std::process::exit(0);
        }
        for lint in &lints {
            eprintln!("{}", lint.message);
        }
        std::process::exit(1);
    } else if html_mode {
        println!("{}", markright::to_html(&doc));
    } else if format_mode {
        print!("{}", markright::to_string(&doc));
    } else {
        #[cfg(feature = "serde")]
        {
            let json = serde_json::to_string_pretty(&doc).unwrap();
            println!("{json}");
        }

        #[cfg(not(feature = "serde"))]
        {
            println!("{doc:?}");
        }
    }
}
