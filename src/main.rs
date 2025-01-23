use std::thread;

pub const MAX_THREADS: usize = 20 * 32;


//Declares the function that will be called from C
/*"allocmem" is the name of the static/dynamic library
 * created from the C program
 */
#[link(name = "allocmem")]
extern "C" {
    //Defined in C
    pub fn alloc_fifty_mib() -> *mut f64;
}

fn main() -> ! {
    //counter
    let mut i = 0usize;

    //Allocates 50 * MAX_THREADS MiB data on heap.
    //Also spwans MAX_THREADS + 1 OS-threads.
    while i <= MAX_THREADS {
        unsafe {
            let t = alloc_fifty_mib();
            //Doing some data writes...
            for i in 0..(128 * 1024 * 50) {
                *t.offset(i) = 0.000001;
            }
            thread::spawn(
                || {
                    let kib = [0.0_f64; 128];
                    for e in kib {
                        let _ = e + 0.000001;
                    }
                    //Keeping the thread busy forever...
                    loop {}
                }
            );
        }
        i += 1;
    }

    //Prevent the main from exiting.
    loop {}
}
