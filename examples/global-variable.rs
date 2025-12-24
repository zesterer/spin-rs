use std::thread;

use library::*;

fn main() {
    // Imagine having separate idependant processes connecting to the same shared library to get access to hardware ressources.

    let regular_process = thread::spawn(|| {
        while let Some(i) = regular_user() {
            println!("hi {:>4.1} from regular user!", i);
            assert_eq!(true, i >= FROM_NOW_ON_ONLY_IMPORTANT_USERS)
        }
    });

    let important_process = thread::spawn(|| {
        while let Some(i) = important_user() {
            println!("hi {:>4.1} from important user!", i);
        }
    });

    let _ = regular_process.join();
    let _ = important_process.join();

    assert_eq!(true, get_current_value() >= 0.0);
}

/// In a (shared) library, it might make sense to have an internal state that is only controlled by the library itself.
/// An example could be a hardware ressources with a known limit.
///
mod library {
    use spin::Mutex;

    const REGULAR_STEP: f64 = 1.0;
    const IMPORTANT_STEP: f64 = 1.5;
    pub const FROM_NOW_ON_ONLY_IMPORTANT_USERS: f64 = 10.0;
    pub const HARDWARE_LIMIT: f64 = 21.0;

    struct State {
        limited_hardware_ressource: f64,
    }

    static INTERNAL_STATE: Mutex<State> = Mutex::new(State {
        limited_hardware_ressource: HARDWARE_LIMIT,
    });

    pub fn regular_user() -> Option<f64> {
        let mut state = INTERNAL_STATE.lock();
        if state.limited_hardware_ressource <= FROM_NOW_ON_ONLY_IMPORTANT_USERS + REGULAR_STEP {
            None
        } else {
            state.limited_hardware_ressource = state.limited_hardware_ressource - REGULAR_STEP;
            Some(state.limited_hardware_ressource)
        }
    }

    pub fn important_user() -> Option<f64> {
        let mut state = INTERNAL_STATE.lock();
        if state.limited_hardware_ressource <= IMPORTANT_STEP {
            None
        } else {
            state.limited_hardware_ressource = state.limited_hardware_ressource - IMPORTANT_STEP;
            Some(state.limited_hardware_ressource)
        }
    }

    pub fn get_current_value() -> f64 {
        let state = INTERNAL_STATE.lock();
        state.limited_hardware_ressource
    }
}
