fn main() {
    let v = vec![45u32,67,23,8,2,1];
    let result = cal_sum(&v);
    println!("The sum of v is {:?}", result);
    let sum_overflow = vec![45u32,67,23,8,2,1, u32::MAX];
    let sum_overflow_result = cal_sum(&sum_overflow);
    println!("The sum of sum_overflow is {:?}", sum_overflow_result);
    assert_eq!(cal_sum(&sum_overflow), None);
}
// 实现一个函数，为u32类型的整数集合求和，参数类型为 &[u32]，返回类型为Option，溢出时返回None
fn cal_sum(vec: &[u32]) -> Option<u32> {
    let mut sum = 0u32;
    for item in vec.iter(){
        let surplus = u32::MAX - sum;
        if  surplus < *item {
            return None;
        }
        sum += *item;
    }
    Some(sum)
}