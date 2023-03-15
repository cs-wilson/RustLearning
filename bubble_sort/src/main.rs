fn main() {
    println!("Hello, world!");
    let mut number_vec = vec![8, 9, -4, 56, 79, -12, -5, 2, 0];

    // number bubble sort
    println!("number_vec排序前: {:?}",number_vec);
    number_bubble_sort(&mut number_vec);
    println!("number_vec排序后：{:?}",number_vec);

    let mut char_vec = vec!["A", "C","*","F"];

    println!("number_vec排序前: {:?}",number_vec);
    bubble_sort(&mut number_vec);
    println!("number_vec排序后：{:?}",number_vec);

    println!("char_vec排序前: {:?}",char_vec);
    bubble_sort(&mut char_vec);
    println!("char_vec排序后：{:?}",char_vec);

}

fn number_bubble_sort(array: &mut Vec<i8>) -> &Vec<i8> {
    for _i in 0..array.len() {
        for i in 0..array.len() - 1 {
            if array[i] > array[i + 1] {
                array.swap(i, i + 1); 
            }
        }
    }
    array
}

fn bubble_sort<T: PartialOrd>(array: &mut Vec<T>) -> &Vec<T> {
    let length = array.len();
    for _i in 0..length {
        for i in 0..length - 1 {
            if array[i] > array[i + 1] {
                array.swap(i, i + 1); 
            }
        }
    }
    array
}