use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();


    let message = if args.len() > 1 {
        &args[1]
    } else {
        "Ara chi 7ajra!"
    };

    let len = message.len();

    
    let bt = "_".repeat(len + 4);
    let bb = "-".repeat(len + 4);

    let allal = format!(r#"                 {}
                /{}\
                  {}!      
                \{}/
                      
      ____//____       
     /    ||    \
    |  (~°||°~)  |
     \    ||    /
      \   ||   /
       \__||__/
       |-----|
      /|     |\
     / |     | \
    /  |     |  \
       |     |
       \_____/
    "#
    , bt, " ".repeat(len + 4),message,bb);

    println!("{}",allal);
}