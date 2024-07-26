use std::thread;
use std::time::Duration;

enum TrafficLightState {
    Red,
    Green,
    Yellow,
}

impl TrafficLightState {
    fn next(&self) -> TrafficLightState {
        match self {
            TrafficLightState::Red => TrafficLightState::Green,
            TrafficLightState::Green => TrafficLightState::Yellow,
            TrafficLightState::Yellow => TrafficLightState::Red,
        }
    }

    fn duration(&self) -> Duration {
        match self {
            TrafficLightState::Red => Duration::new(2*60, 0),
            TrafficLightState::Green => Duration::new(1*60+30, 0),
            TrafficLightState::Yellow => Duration::new(30, 0),
        }
    }

    fn display(&self) {
        match self {
            TrafficLightState::Red => println!("Red Light"),
            TrafficLightState::Green => println!("Green Light"),
            TrafficLightState::Yellow => println!("Yellow Light"),
        }
    }
}

fn main() {
    let mut current_state = TrafficLightState::Red;

    loop {
        current_state.display();
        thread::sleep(current_state.duration());
        current_state = current_state.next();
    }
}