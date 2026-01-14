//! This example is focused on the "library" part and not on the "main" function.
//! The main function shall be a single threaded no_std environment. For example a real time bare metal kernel.
//! The library shall be independently developed and the interface between the kernel and the library is a single C function call. Inside the library, a global variable is used to store its internal state.

fn main() {
    let value = library::lib_function();
    assert_eq!(value, 1);
    let value = library::lib_function();
    assert_eq!(value, 2);
    let value = library::lib_function();
    assert_eq!(value, 3);
}

mod library {
    extern crate spin;

    struct VeryComplexStruct {
        first: i32,
        second: i32,
        return_first: bool,
    }

    impl VeryComplexStruct {
        pub const fn new() -> VeryComplexStruct {
            VeryComplexStruct {
                first: 1,
                second: -1,
                return_first: true,
            }
        }

        pub fn value(&mut self) -> i32 {
            let return_value;
            if self.return_first {
                self.return_first = false;
                return_value = self.first;
                self.first += 1;
            } else {
                self.return_first = true;
                return_value = self.second;
                self.second += 1;
            }
            return_value
        }
    }

    // A global variable is used to store the internal state of the library / plugin
    static mut DEPRECATED_INTERNAL_STATE: VeryComplexStruct = VeryComplexStruct::new();       

    // As static mut references shall not be used (https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html), spin::Mutex is used instead.
    static INTERNAL_STATE: spin::Mutex<VeryComplexStruct> =
        spin::Mutex::new(VeryComplexStruct::new());
 
    #[unsafe(no_mangle)]
    pub extern "C" fn lib_function() -> i32 {
        let state = INTERNAL_STATE.try_lock();
        match state {
            Some(mut thing) => thing.value(),
            None => 0,
        }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn deprecated_lib_function() -> i32 {
        #[allow(static_mut_refs)]
        unsafe {
            DEPRECATED_INTERNAL_STATE.value()
        }
    }
}
