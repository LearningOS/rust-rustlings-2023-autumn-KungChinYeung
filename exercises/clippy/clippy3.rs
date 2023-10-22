// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.


#[allow(unused_variables, unused_assignments)]
// fn main() {
//     let my_option: Option<()> = None;
//     if my_option.is_none() {
//         my_option.unwrap();
//     }

//     let my_arr = &[
//         -1, -2, -3
//         -4, -5, -6
//     ];
//     println!("My array! Here it is: {:?}", my_arr);

//     let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
//     println!("This Vec is empty, see? {:?}", my_empty_vec);

//     let mut value_a = 45;
//     let mut value_b = 66;
//     // Let's swap these two!
//     value_a = value_b;
//     value_b = value_a;
//     println!("value a: {}; value b: {}", value_a, value_b);
// }
#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        // 这里不应该使用 unwrap，因为 Option 是 None。
        // 可以使用 expect 方法，提供一个更有意义的错误消息。
        panic!("This Option should not be None!");
    }

    // 在数组中添加逗号，以将其拆分成多行。
    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let mut my_empty_vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    // 使用 clear 方法清空 Vec。
    my_empty_vec.clear();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // 使用元组解构来交换变量的值。
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
