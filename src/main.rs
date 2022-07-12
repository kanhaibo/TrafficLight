// 交通灯enum
pub enum TrafficLight {
    Red,
    Green,
    Yellow,
}
// 交通信号灯需要实现的trait
pub trait Delay {
    fn light_time(&self) -> u32;
}

// 实现成员返回整数不同
impl Delay for TrafficLight {
    fn light_time(&self) -> u32 {
        match &self {
            TrafficLight::Red => 10,
            TrafficLight::Green => 20,
            TrafficLight::Yellow => 5,
        }
    }
}

fn main() {
    let mm = TrafficLight::Red;
    println!("the light is red delay time is  {:?}", mm.light_time());
    let mm = TrafficLight::Green;
    println!("the light is Green delay time is  {:?}", mm.light_time());
}
