
enum Background {
    Color(u8, u8, u8),
    Image(&'static str),
}

enum UserType {
    Casual,
    Power
}

struct MyApp {
    theme: Background,
    user_type: UserType,
    secret_user_id: usize
}

fn guarded_match(app: MyApp) -> String {  
    match app {
        MyApp { secret_user_id: uid, .. } if uid <= 100 => "You are an early bird!".to_owned(),
        MyApp { .. } => "Thank you for also joining".to_owned()
    }
}

fn destructuring_match(app: MyApp) -> String {
    match app {
        MyApp { user_type: UserType::Power, 
                secret_user_id: uid, 
                theme: Background::Color(b1, b2, b3) } => 
            format!("A power user with id >{}< and color background (#{:02x}{:02x}{:02x})", uid, b1, b2, b3),
        MyApp { user_type: UserType::Power, 
                secret_user_id: uid,      
                theme: Background::Image(path) } => 
            format!("A power user with id >{}< and image background (path: {})", uid, path),
        MyApp { user_type: _, secret_user_id: uid, .. } => format!("A regular user with id >{}<, individual backgrounds not supported", uid), 
    }
}

fn literal_match(choice: usize) -> String {
    match choice {
        0 | 1 => "zero or one".to_owned(),
        2 ... 9 => "two to nine".to_owned(),
        10 => "ten".to_owned(),
        _ => "anything else".to_owned()
    }
}

fn literal_str_match(choice: &str) -> String {
    match choice {
        "🏋️" => "Power lifting".to_owned(),
        "🏈" => "Football".to_owned(),
        "🥋" => "BJJ".to_owned(),
        _ => "Competitive BBQ".to_owned()
    }
}


fn reference_match(m: &Option<&str>) -> String {
    match m {
        Some(ref s) => s.to_string(),
        _ => "Nothing".to_string()
    }
}

fn tuple_match(choices: (i32, i32, i32, i32)) -> String {
    match choices {
        (_, second, _, fourth) => format!("Numbers at positions 1 and 3 are {} and {} respectively", second, fourth)
    }
}


pub fn main() {
    let opt = Some(42);
    // the most common match:
    match opt {
        Some(nr) => println!("Got {}", nr),
        // _ matches everything else as a placeholder for
        // don't care. 
        _ => println!("Found None") 
    }
    println!();
    println!("Literal match for 0: {}", literal_match(0));
    println!("Literal match for 10: {}", literal_match(10));
    println!("Literal match for 100: {}", literal_match(100));

    println!();
    println!("Literal match for 0: {}", tuple_match((0, 10, 0, 100)));
    
    println!();
    let mystr = Some("Hello");
    println!("Mathing on a reference: {}", reference_match(&mystr));
    println!("It's still owned here: {:?}", mystr);

    println!();
    let power = MyApp {
        secret_user_id: 99,
        theme: Background::Color(255, 255, 0),
        user_type: UserType::Power
    };
    println!("Destructuring a power user: {}", destructuring_match(power));
    
    let casual = MyApp {
        secret_user_id: 10,
        theme: Background::Image("my/fav/image.png"),
        user_type: UserType::Casual
    };
    println!("Destructuring a casual user: {}", destructuring_match(casual));

    let power2 = MyApp {
        secret_user_id: 150,
        theme: Background::Image("a/great/landscape.png"),
        user_type: UserType::Power
    };
    println!("Destructuring another power user: {}", destructuring_match(power2));
    
    println!();
    let early = MyApp {
        secret_user_id: 4,
        theme: Background::Color(255, 255, 0),
        user_type: UserType::Power
    };
    println!("Guarded matching (early): {}", guarded_match(early));

     let not_so_early = MyApp {
        secret_user_id: 1003942,
        theme: Background::Color(255, 255, 0),
        user_type: UserType::Power
    };
    println!("Guarded matching (late): {}", guarded_match(not_so_early));
    println!();

    println!("Literal match for 🥋: {}", literal_str_match("🥋"));
    println!("Literal match for 🏈: {}", literal_str_match("🏈"));
    println!("Literal match for 🏋️: {}", literal_str_match("🏋️"));
    println!("Literal match for ⛳: {}", literal_str_match("⛳"));
}