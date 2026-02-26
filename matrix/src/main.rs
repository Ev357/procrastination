pub mod r#macro;
pub mod matrix;

fn main() {
    let a = matrix![
        1, 2;
        3, 4
    ];

    let b = matrix![
        5, 6, 7;
        8, 9, 10
    ];

    let c = a.multiply(&b);

    println!("{:#?}", c);
}
