pub mod r#macro;
pub mod matrix;
pub mod super_string;

fn main() {
    let a = matrix![
        ss!("a"), ss!("0"), ss!("0");
        ss!("0"), ss!("1"), ss!("0");
        ss!("0"), ss!("0"), ss!("1")
    ];

    let b = matrix![
        ss!("b"), ss!("c");
        ss!("d"), ss!("e");
        ss!("f"), ss!("g")
    ];

    let c = a.multiply(&b);

    println!("{}", c);
}
