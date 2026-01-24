//! This example is focused on the "library" part and not on the "main" function.
//! The main function shall be a single threaded no_std environment. For example a real time bare metal kernel.
//! The library shall be independently developed and the interface between the kernel and the library are 2 predefined C function calls. Inside the library, global variables are used to store its internal state.

fn main() {
    // the library is not initialized. It returns 0.
    let value = library::lib_function();
    assert_eq!(value, 0);

    library::init_the_library(&(function_to_provide as extern "C" fn() -> bool));

    let value = library::lib_function();
    assert_eq!(value, 1);

    // will be ignored in the library, as the spin::Once is already initialized.
    library::init_the_library(&(function_to_provide2 as extern "C" fn() -> bool));

    let value = library::lib_function();
    assert_eq!(value, -1);
    let value = library::lib_function();
    assert_eq!(value, 2);
}

extern "C" fn function_to_provide() -> bool {
    true
}

extern "C" fn function_to_provide2() -> bool {
    false
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

        pub fn value(&mut self, dynamic_kernel_decision: bool) -> i32 {
            if dynamic_kernel_decision == false {
                return 0;
            }
            let return_value;
            if self.return_first {
                self.return_first = false;
                return_value = self.first;
                self.first += 1;
            } else {
                self.return_first = true;
                return_value = self.second;
                self.second -= 1;
            }
            return_value
        }
    }

    // A global variable is used to store the internal state of the library / plugin.
    static mut DEPRECATED_INTERNAL_STATE: VeryComplexStruct = VeryComplexStruct::new();

    // As static mut references shall not be used (https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html), spin::Mutex is used instead.
    static INTERNAL_STATE: spin::Mutex<VeryComplexStruct> =
        spin::Mutex::new(VeryComplexStruct::new());

    // In this use case, the spin::Once is used to carry the information if the kernel initialized the library properly.
    static FN_CALL_TO_KERNEL: spin::Once<extern "C" fn() -> bool> = spin::Once::new();

    #[unsafe(no_mangle)]
    pub extern "C" fn init_the_library(ref_to_a_function: &extern "C" fn() -> bool) {
        FN_CALL_TO_KERNEL.init_from_ref(ref_to_a_function);
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn lib_function() -> i32 {
        let initialized_from_the_caller = FN_CALL_TO_KERNEL.get();
        let kernel_decision = match initialized_from_the_caller {
            Some(provided_function) => provided_function(),
            None => false,
        };

        let state = INTERNAL_STATE.try_lock();
        match state {
            Some(mut thing) => thing.value(kernel_decision),
            None => 0,
        }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn deprecated_lib_function() -> i32 {
        let initialized_from_the_caller = FN_CALL_TO_KERNEL.get();
        let kernel_decision = match initialized_from_the_caller {
            Some(provided_function) => provided_function(),
            None => false,
        };

        #[allow(static_mut_refs)]
        unsafe {
            DEPRECATED_INTERNAL_STATE.value(kernel_decision)
        }
    }
}
