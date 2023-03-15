fn main() {
    println!("Hello, world!");
    let mut number_vec = vec![8, 9, -4, 56, 79, -12, -5, 2, 0];

    // number bubble sort
    println!("number_vec排序前: {:?}",number_vec);
    number_bubble_sort(&mut number_vec);
    println!("number_vec排序后：{:?}",number_vec);

    let mut generic_number_vec = vec![8, 9, -4, 56, 79, -12, -5, 2, 0];
    let mut generic_char_vec = vec!["A", "C","*","F"];
    println!("generic_number_vec排序前: {:?}",generic_number_vec);
    bubble_sort(&mut generic_number_vec);
    println!("generic_number_vec排序后：{:?}",generic_number_vec);

    println!("generic_char_vec排序前: {:?}",generic_char_vec);
    bubble_sort(&mut generic_char_vec);
    println!("generic_char_vec排序后：{:?}",generic_char_vec);

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