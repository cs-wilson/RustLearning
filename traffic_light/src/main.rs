fn main() {
    let red_light = TrafficLight::Red;
    println!("The duration time of red light is {}s",red_light.duration_time());
    let yellow_light = TrafficLight::Yellow;
    println!("The duration time of yellow light is {}s",yellow_light.duration_time());
    let green_light = TrafficLight::Green;
    println!("The duration time of green light is {}s",green_light.duration_time());
}

// 为枚举交通信号灯实现一个 trait，trait里包含一个返回时间的方法，不同的灯持续的时间不同

//定义一个枚举
enum TrafficLight {
    Red,
    Yellow,
    Green
}
//定义一个特征trait
pub trait Duration{
    fn duration_time(&self) -> u8;
}
//实现一个方法，方法中有一个函数来返回不同灯的时间，用模式匹配
impl Duration for TrafficLight {
    fn duration_time(&self) -> u8 {
        match &self {
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 60,
        }
    }
}






