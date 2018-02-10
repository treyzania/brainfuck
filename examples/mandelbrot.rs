#![allow(unused_imports)]
#![allow(dead_code)]

use std::io::{stdout, stdin, Read, Write};

const TAPE_SIZE: usize = 20000;

fn ptr_wrap_add(ptr: &mut usize) {
    if *ptr == TAPE_SIZE - 1 {
        *ptr = 0;
    } else {
        *ptr += 1;
    }
}

fn ptr_wrap_sub(ptr: &mut usize) {
    if *ptr == 0 {
        *ptr = TAPE_SIZE - 1;
    } else {
        *ptr -= 1;
    }
}

fn main() {
    let mut tape = [0u8; TAPE_SIZE];
    let mut ptr = 0usize;

    let stdout = stdout();
    let mut handle = stdout.lock();

    tape[ptr] = tape[ptr].wrapping_add(1);
    tape[ptr] = tape[ptr].wrapping_add(1);
    tape[ptr] = tape[ptr].wrapping_add(1);
    tape[ptr] = tape[ptr].wrapping_add(1);
    tape[ptr] = tape[ptr].wrapping_add(1);
    tape[ptr] = tape[ptr].wrapping_add(1);
    tape[ptr] = tape[ptr].wrapping_add(1);
    tape[ptr] = tape[ptr].wrapping_add(1);
    tape[ptr] = tape[ptr].wrapping_add(1);
    tape[ptr] = tape[ptr].wrapping_add(1);
    tape[ptr] = tape[ptr].wrapping_add(1);
    tape[ptr] = tape[ptr].wrapping_add(1);
    tape[ptr] = tape[ptr].wrapping_add(1);
    while tape[ptr] != 0 {
        tape[ptr] = tape[ptr].wrapping_sub(1);
        ptr_wrap_add(&mut ptr);
        tape[ptr] = tape[ptr].wrapping_add(1);
        tape[ptr] = tape[ptr].wrapping_add(1);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        tape[ptr] = tape[ptr].wrapping_add(1);
        tape[ptr] = tape[ptr].wrapping_add(1);
        tape[ptr] = tape[ptr].wrapping_add(1);
        tape[ptr] = tape[ptr].wrapping_add(1);
        tape[ptr] = tape[ptr].wrapping_add(1);
        ptr_wrap_add(&mut ptr);
        tape[ptr] = tape[ptr].wrapping_add(1);
        tape[ptr] = tape[ptr].wrapping_add(1);
        ptr_wrap_add(&mut ptr);
        tape[ptr] = tape[ptr].wrapping_add(1);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        }
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    tape[ptr] = tape[ptr].wrapping_add(1);
    tape[ptr] = tape[ptr].wrapping_add(1);
    tape[ptr] = tape[ptr].wrapping_add(1);
    tape[ptr] = tape[ptr].wrapping_add(1);
    tape[ptr] = tape[ptr].wrapping_add(1);
    tape[ptr] = tape[ptr].wrapping_add(1);
    ptr_wrap_add(&mut ptr);
    tape[ptr] = tape[ptr].wrapping_sub(1);
    tape[ptr] = tape[ptr].wrapping_sub(1);
    tape[ptr] = tape[ptr].wrapping_sub(1);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    tape[ptr] = tape[ptr].wrapping_add(1);
    tape[ptr] = tape[ptr].wrapping_add(1);
    tape[ptr] = tape[ptr].wrapping_add(1);
    tape[ptr] = tape[ptr].wrapping_add(1);
    tape[ptr] = tape[ptr].wrapping_add(1);
    tape[ptr] = tape[ptr].wrapping_add(1);
    tape[ptr] = tape[ptr].wrapping_add(1);
    tape[ptr] = tape[ptr].wrapping_add(1);
    tape[ptr] = tape[ptr].wrapping_add(1);
    tape[ptr] = tape[ptr].wrapping_add(1);
    tape[ptr] = tape[ptr].wrapping_add(1);
    tape[ptr] = tape[ptr].wrapping_add(1);
    tape[ptr] = tape[ptr].wrapping_add(1);
    tape[ptr] = tape[ptr].wrapping_add(1);
    tape[ptr] = tape[ptr].wrapping_add(1);
    while tape[ptr] != 0 {
        while tape[ptr] != 0 {
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            }
        tape[ptr] = tape[ptr].wrapping_add(1);
        while tape[ptr] != 0 {
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            }
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        tape[ptr] = tape[ptr].wrapping_sub(1);
        }
    tape[ptr] = tape[ptr].wrapping_add(1);
    while tape[ptr] != 0 {
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        while tape[ptr] != 0 {
            tape[ptr] = tape[ptr].wrapping_sub(1);
            }
        ptr_wrap_add(&mut ptr);
        }
    ptr_wrap_sub(&mut ptr);
    ptr_wrap_sub(&mut ptr);
    ptr_wrap_sub(&mut ptr);
    ptr_wrap_sub(&mut ptr);
    ptr_wrap_sub(&mut ptr);
    ptr_wrap_sub(&mut ptr);
    ptr_wrap_sub(&mut ptr);
    ptr_wrap_sub(&mut ptr);
    ptr_wrap_sub(&mut ptr);
    while tape[ptr] != 0 {
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        }
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    while tape[ptr] != 0 {
        tape[ptr] = tape[ptr].wrapping_sub(1);
        }
    tape[ptr] = tape[ptr].wrapping_add(1);
    ptr_wrap_sub(&mut ptr);
    ptr_wrap_sub(&mut ptr);
    ptr_wrap_sub(&mut ptr);
    ptr_wrap_sub(&mut ptr);
    ptr_wrap_sub(&mut ptr);
    ptr_wrap_sub(&mut ptr);
    ptr_wrap_sub(&mut ptr);
    tape[ptr] = tape[ptr].wrapping_add(1);
    tape[ptr] = tape[ptr].wrapping_add(1);
    tape[ptr] = tape[ptr].wrapping_add(1);
    tape[ptr] = tape[ptr].wrapping_add(1);
    tape[ptr] = tape[ptr].wrapping_add(1);
    while tape[ptr] != 0 {
        tape[ptr] = tape[ptr].wrapping_sub(1);
        while tape[ptr] != 0 {
            tape[ptr] = tape[ptr].wrapping_sub(1);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            tape[ptr] = tape[ptr].wrapping_add(1);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            }
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        }
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    tape[ptr] = tape[ptr].wrapping_add(1);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    tape[ptr] = tape[ptr].wrapping_add(1);
    ptr_wrap_sub(&mut ptr);
    ptr_wrap_sub(&mut ptr);
    ptr_wrap_sub(&mut ptr);
    ptr_wrap_sub(&mut ptr);
    ptr_wrap_sub(&mut ptr);
    ptr_wrap_sub(&mut ptr);
    ptr_wrap_sub(&mut ptr);
    ptr_wrap_sub(&mut ptr);
    ptr_wrap_sub(&mut ptr);
    ptr_wrap_sub(&mut ptr);
    ptr_wrap_sub(&mut ptr);
    ptr_wrap_sub(&mut ptr);
    ptr_wrap_sub(&mut ptr);
    ptr_wrap_sub(&mut ptr);
    ptr_wrap_sub(&mut ptr);
    ptr_wrap_sub(&mut ptr);
    ptr_wrap_sub(&mut ptr);
    while tape[ptr] != 0 {
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        }
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    ptr_wrap_add(&mut ptr);
    while tape[ptr] != 0 {
        tape[ptr] = tape[ptr].wrapping_sub(1);
        }
    tape[ptr] = tape[ptr].wrapping_add(1);
    while tape[ptr] != 0 {
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        while tape[ptr] != 0 {
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            while tape[ptr] != 0 {
                tape[ptr] = tape[ptr].wrapping_sub(1);
                }
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            }
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        while tape[ptr] != 0 {
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            }
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        while tape[ptr] != 0 {
            tape[ptr] = tape[ptr].wrapping_sub(1);
            }
        tape[ptr] = tape[ptr].wrapping_add(1);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        tape[ptr] = tape[ptr].wrapping_add(1);
        tape[ptr] = tape[ptr].wrapping_add(1);
        tape[ptr] = tape[ptr].wrapping_add(1);
        tape[ptr] = tape[ptr].wrapping_add(1);
        while tape[ptr] != 0 {
            tape[ptr] = tape[ptr].wrapping_sub(1);
            while tape[ptr] != 0 {
                tape[ptr] = tape[ptr].wrapping_sub(1);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                }
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            }
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        tape[ptr] = tape[ptr].wrapping_add(1);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        tape[ptr] = tape[ptr].wrapping_add(1);
        tape[ptr] = tape[ptr].wrapping_add(1);
        tape[ptr] = tape[ptr].wrapping_add(1);
        tape[ptr] = tape[ptr].wrapping_add(1);
        tape[ptr] = tape[ptr].wrapping_add(1);
        tape[ptr] = tape[ptr].wrapping_add(1);
        tape[ptr] = tape[ptr].wrapping_add(1);
        while tape[ptr] != 0 {
            tape[ptr] = tape[ptr].wrapping_sub(1);
            while tape[ptr] != 0 {
                tape[ptr] = tape[ptr].wrapping_sub(1);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                }
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            }
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        tape[ptr] = tape[ptr].wrapping_add(1);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        while tape[ptr] != 0 {
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            }
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        while tape[ptr] != 0 {
            while tape[ptr] != 0 {
                tape[ptr] = tape[ptr].wrapping_sub(1);
                }
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            while tape[ptr] != 0 {
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                }
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            while tape[ptr] != 0 {
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                }
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            while tape[ptr] != 0 {
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                }
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            while tape[ptr] != 0 {
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                }
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            while tape[ptr] != 0 {
                tape[ptr] = tape[ptr].wrapping_sub(1);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                }
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            while tape[ptr] != 0 {
                tape[ptr] = tape[ptr].wrapping_sub(1);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                }
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            while tape[ptr] != 0 {
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_sub(1);
                }
            tape[ptr] = tape[ptr].wrapping_add(1);
            while tape[ptr] != 0 {
                ptr_wrap_add(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                }
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            while tape[ptr] != 0 {
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                }
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            while tape[ptr] != 0 {
                ptr_wrap_add(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_sub(1);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            }
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                }
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            while tape[ptr] != 0 {
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                ptr_wrap_sub(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_sub(1);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                }
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            while tape[ptr] != 0 {
                ptr_wrap_add(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                }
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            while tape[ptr] != 0 {
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                }
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            while tape[ptr] != 0 {
                ptr_wrap_add(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_sub(1);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            }
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                }
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            while tape[ptr] != 0 {
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                ptr_wrap_sub(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_sub(1);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                }
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            while tape[ptr] != 0 {
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                }
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            while tape[ptr] != 0 {
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                }
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            while tape[ptr] != 0 {
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_sub(1);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_sub(1);
                }
            tape[ptr] = tape[ptr].wrapping_add(1);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            tape[ptr] = tape[ptr].wrapping_add(1);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            while tape[ptr] != 0 {
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                }
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            while tape[ptr] != 0 {
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            }
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            }
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        }
                    }
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            }
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            }
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            }
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        }
                    }
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                }
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            while tape[ptr] != 0 {
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                }
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            while tape[ptr] != 0 {
                tape[ptr] = tape[ptr].wrapping_sub(1);
                ptr_wrap_add(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_sub(1);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                }
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            while tape[ptr] != 0 {
                tape[ptr] = tape[ptr].wrapping_sub(1);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                }
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            while tape[ptr] != 0 {
                tape[ptr] = tape[ptr].wrapping_sub(1);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                }
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            while tape[ptr] != 0 {
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    }
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                while tape[ptr] != 0 {
                    while tape[ptr] != 0 {
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                tape[ptr] = tape[ptr].wrapping_add(1);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                }
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                }
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            }
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            ptr_wrap_sub(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_sub(&mut ptr);
                            }
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                }
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                }
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            }
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            ptr_wrap_sub(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_sub(&mut ptr);
                            }
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                while tape[ptr] != 0 {
                    while tape[ptr] != 0 {
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                tape[ptr] = tape[ptr].wrapping_add(1);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                }
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                while tape[ptr] != 0 {
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    }
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                while tape[ptr] != 0 {
                                    tape[ptr] = tape[ptr].wrapping_sub(1);
                                    }
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                while tape[ptr] != 0 {
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    }
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_sub(&mut ptr);
                                }
                            }
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_sub(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                }
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                while tape[ptr] != 0 {
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    }
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                while tape[ptr] != 0 {
                                    tape[ptr] = tape[ptr].wrapping_sub(1);
                                    }
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                while tape[ptr] != 0 {
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    }
                                ptr_wrap_add(&mut ptr);
                                while tape[ptr] != 0 {
                                    tape[ptr] = tape[ptr].wrapping_sub(1);
                                    }
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_sub(&mut ptr);
                                }
                            }
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_sub(&mut ptr);
                            while tape[ptr] != 0 {
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                }
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            }
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                }
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                }
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                while tape[ptr] != 0 {
                                    tape[ptr] = tape[ptr].wrapping_sub(1);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    tape[ptr] = tape[ptr].wrapping_sub(1);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    tape[ptr] = tape[ptr].wrapping_add(1);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    while tape[ptr] != 0 {
                                        tape[ptr] = tape[ptr].wrapping_sub(1);
                                        ptr_wrap_add(&mut ptr);
                                        ptr_wrap_add(&mut ptr);
                                        ptr_wrap_add(&mut ptr);
                                        tape[ptr] = tape[ptr].wrapping_add(1);
                                        ptr_wrap_sub(&mut ptr);
                                        ptr_wrap_sub(&mut ptr);
                                        ptr_wrap_sub(&mut ptr);
                                        }
                                    ptr_wrap_sub(&mut ptr);
                                    }
                                ptr_wrap_add(&mut ptr);
                                while tape[ptr] != 0 {
                                    tape[ptr] = tape[ptr].wrapping_sub(1);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    tape[ptr] = tape[ptr].wrapping_sub(1);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    tape[ptr] = tape[ptr].wrapping_add(1);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    }
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                }
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                while tape[ptr] != 0 {
                                    tape[ptr] = tape[ptr].wrapping_sub(1);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    tape[ptr] = tape[ptr].wrapping_sub(1);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    tape[ptr] = tape[ptr].wrapping_add(1);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    }
                                ptr_wrap_sub(&mut ptr);
                                }
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                }
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            }
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            }
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_sub(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_add(&mut ptr);
                                }
                            ptr_wrap_sub(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_sub(&mut ptr);
                                }
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                while tape[ptr] != 0 {
                                    tape[ptr] = tape[ptr].wrapping_sub(1);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    tape[ptr] = tape[ptr].wrapping_sub(1);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    tape[ptr] = tape[ptr].wrapping_add(1);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    while tape[ptr] != 0 {
                                        tape[ptr] = tape[ptr].wrapping_sub(1);
                                        ptr_wrap_add(&mut ptr);
                                        ptr_wrap_add(&mut ptr);
                                        ptr_wrap_add(&mut ptr);
                                        ptr_wrap_add(&mut ptr);
                                        tape[ptr] = tape[ptr].wrapping_add(1);
                                        ptr_wrap_sub(&mut ptr);
                                        ptr_wrap_sub(&mut ptr);
                                        ptr_wrap_sub(&mut ptr);
                                        ptr_wrap_sub(&mut ptr);
                                        }
                                    ptr_wrap_add(&mut ptr);
                                    }
                                ptr_wrap_sub(&mut ptr);
                                while tape[ptr] != 0 {
                                    tape[ptr] = tape[ptr].wrapping_sub(1);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    tape[ptr] = tape[ptr].wrapping_sub(1);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    tape[ptr] = tape[ptr].wrapping_add(1);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    }
                                ptr_wrap_sub(&mut ptr);
                                }
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                while tape[ptr] != 0 {
                                    tape[ptr] = tape[ptr].wrapping_sub(1);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    tape[ptr] = tape[ptr].wrapping_sub(1);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    tape[ptr] = tape[ptr].wrapping_add(1);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    }
                                ptr_wrap_add(&mut ptr);
                                }
                            ptr_wrap_sub(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                }
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            }
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                while tape[ptr] != 0 {
                                    tape[ptr] = tape[ptr].wrapping_sub(1);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    tape[ptr] = tape[ptr].wrapping_add(1);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    }
                                ptr_wrap_sub(&mut ptr);
                                }
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                }
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            }
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                }
                            ptr_wrap_sub(&mut ptr);
                            }
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    }
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                while tape[ptr] != 0 {
                    while tape[ptr] != 0 {
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                tape[ptr] = tape[ptr].wrapping_add(1);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                }
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                }
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            }
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            ptr_wrap_sub(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_sub(&mut ptr);
                            }
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                while tape[ptr] != 0 {
                    while tape[ptr] != 0 {
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                tape[ptr] = tape[ptr].wrapping_add(1);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            while tape[ptr] != 0 {
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                }
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                }
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                }
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_sub(&mut ptr);
                            }
                        }
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            while tape[ptr] != 0 {
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                }
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                }
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                }
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                }
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_sub(&mut ptr);
                            }
                        }
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            }
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_sub(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                while tape[ptr] != 0 {
                                    tape[ptr] = tape[ptr].wrapping_sub(1);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    tape[ptr] = tape[ptr].wrapping_add(1);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    }
                                ptr_wrap_sub(&mut ptr);
                                }
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                }
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            }
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_sub(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                }
                            ptr_wrap_sub(&mut ptr);
                            }
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            }
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                while tape[ptr] != 0 {
                                    tape[ptr] = tape[ptr].wrapping_sub(1);
                                    ptr_wrap_sub(&mut ptr);
                                    tape[ptr] = tape[ptr].wrapping_add(1);
                                    ptr_wrap_add(&mut ptr);
                                    }
                                ptr_wrap_add(&mut ptr);
                                }
                            ptr_wrap_sub(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_sub(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                }
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            }
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_sub(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                }
                            ptr_wrap_add(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_sub(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_sub(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                }
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                while tape[ptr] != 0 {
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    }
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                while tape[ptr] != 0 {
                                    tape[ptr] = tape[ptr].wrapping_sub(1);
                                    }
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                while tape[ptr] != 0 {
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    }
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_sub(&mut ptr);
                                }
                            }
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                }
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                while tape[ptr] != 0 {
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    }
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                while tape[ptr] != 0 {
                                    tape[ptr] = tape[ptr].wrapping_sub(1);
                                    }
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                while tape[ptr] != 0 {
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    }
                                ptr_wrap_add(&mut ptr);
                                while tape[ptr] != 0 {
                                    tape[ptr] = tape[ptr].wrapping_sub(1);
                                    }
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_sub(&mut ptr);
                                }
                            }
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_sub(&mut ptr);
                            while tape[ptr] != 0 {
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                }
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            }
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_sub(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_add(&mut ptr);
                                }
                            ptr_wrap_sub(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_sub(&mut ptr);
                                }
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                while tape[ptr] != 0 {
                                    tape[ptr] = tape[ptr].wrapping_sub(1);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    tape[ptr] = tape[ptr].wrapping_sub(1);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    tape[ptr] = tape[ptr].wrapping_add(1);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    while tape[ptr] != 0 {
                                        tape[ptr] = tape[ptr].wrapping_sub(1);
                                        ptr_wrap_add(&mut ptr);
                                        ptr_wrap_add(&mut ptr);
                                        ptr_wrap_add(&mut ptr);
                                        tape[ptr] = tape[ptr].wrapping_add(1);
                                        ptr_wrap_sub(&mut ptr);
                                        ptr_wrap_sub(&mut ptr);
                                        ptr_wrap_sub(&mut ptr);
                                        }
                                    ptr_wrap_add(&mut ptr);
                                    }
                                ptr_wrap_sub(&mut ptr);
                                while tape[ptr] != 0 {
                                    tape[ptr] = tape[ptr].wrapping_sub(1);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    tape[ptr] = tape[ptr].wrapping_sub(1);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    tape[ptr] = tape[ptr].wrapping_add(1);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    }
                                ptr_wrap_sub(&mut ptr);
                                }
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                while tape[ptr] != 0 {
                                    tape[ptr] = tape[ptr].wrapping_sub(1);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    tape[ptr] = tape[ptr].wrapping_sub(1);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    tape[ptr] = tape[ptr].wrapping_add(1);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    }
                                ptr_wrap_add(&mut ptr);
                                }
                            ptr_wrap_sub(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                }
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            }
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            }
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            }
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                }
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                }
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                while tape[ptr] != 0 {
                                    tape[ptr] = tape[ptr].wrapping_sub(1);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    tape[ptr] = tape[ptr].wrapping_sub(1);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    tape[ptr] = tape[ptr].wrapping_add(1);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    while tape[ptr] != 0 {
                                        tape[ptr] = tape[ptr].wrapping_sub(1);
                                        ptr_wrap_add(&mut ptr);
                                        ptr_wrap_add(&mut ptr);
                                        tape[ptr] = tape[ptr].wrapping_add(1);
                                        ptr_wrap_sub(&mut ptr);
                                        ptr_wrap_sub(&mut ptr);
                                        }
                                    ptr_wrap_sub(&mut ptr);
                                    }
                                ptr_wrap_add(&mut ptr);
                                while tape[ptr] != 0 {
                                    tape[ptr] = tape[ptr].wrapping_sub(1);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    tape[ptr] = tape[ptr].wrapping_sub(1);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    tape[ptr] = tape[ptr].wrapping_add(1);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    }
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                }
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                while tape[ptr] != 0 {
                                    tape[ptr] = tape[ptr].wrapping_sub(1);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    tape[ptr] = tape[ptr].wrapping_sub(1);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    tape[ptr] = tape[ptr].wrapping_add(1);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    }
                                ptr_wrap_sub(&mut ptr);
                                }
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                }
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            }
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                while tape[ptr] != 0 {
                                    tape[ptr] = tape[ptr].wrapping_sub(1);
                                    ptr_wrap_add(&mut ptr);
                                    ptr_wrap_add(&mut ptr);
                                    tape[ptr] = tape[ptr].wrapping_add(1);
                                    ptr_wrap_sub(&mut ptr);
                                    ptr_wrap_sub(&mut ptr);
                                    }
                                ptr_wrap_sub(&mut ptr);
                                }
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                }
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            }
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                }
                            ptr_wrap_sub(&mut ptr);
                            }
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                while tape[ptr] != 0 {
                    while tape[ptr] != 0 {
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                tape[ptr] = tape[ptr].wrapping_add(1);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                }
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                }
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            }
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            ptr_wrap_sub(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_sub(&mut ptr);
                            }
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                }
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_add(&mut ptr);
                                tape[ptr] = tape[ptr].wrapping_add(1);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                }
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            }
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            ptr_wrap_sub(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_sub(&mut ptr);
                            }
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                while tape[ptr] != 0 {
                    while tape[ptr] != 0 {
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            while tape[ptr] != 0 {
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                }
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                }
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                }
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_sub(&mut ptr);
                            }
                        }
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            while tape[ptr] != 0 {
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                }
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                }
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                }
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                }
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_sub(&mut ptr);
                            }
                        }
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_sub(1);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                }
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            tape[ptr] = tape[ptr].wrapping_add(1);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            while tape[ptr] != 0 {
                tape[ptr] = tape[ptr].wrapping_sub(1);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_sub(1);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                }
            tape[ptr] = tape[ptr].wrapping_add(1);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            while tape[ptr] != 0 {
                tape[ptr] = tape[ptr].wrapping_sub(1);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_sub(1);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                handle.write(&[tape[ptr]]).unwrap();
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                }
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            while tape[ptr] != 0 {
                tape[ptr] = tape[ptr].wrapping_sub(1);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                handle.write(&[tape[ptr]]).unwrap();
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                }
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            while tape[ptr] != 0 {
                tape[ptr] = tape[ptr].wrapping_sub(1);
                }
            ptr_wrap_add(&mut ptr);
            while tape[ptr] != 0 {
                tape[ptr] = tape[ptr].wrapping_sub(1);
                }
            ptr_wrap_add(&mut ptr);
            while tape[ptr] != 0 {
                tape[ptr] = tape[ptr].wrapping_sub(1);
                }
            ptr_wrap_add(&mut ptr);
            while tape[ptr] != 0 {
                tape[ptr] = tape[ptr].wrapping_sub(1);
                }
            ptr_wrap_add(&mut ptr);
            while tape[ptr] != 0 {
                tape[ptr] = tape[ptr].wrapping_sub(1);
                }
            ptr_wrap_add(&mut ptr);
            while tape[ptr] != 0 {
                tape[ptr] = tape[ptr].wrapping_sub(1);
                }
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            while tape[ptr] != 0 {
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                }
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            while tape[ptr] != 0 {
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                }
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            while tape[ptr] != 0 {
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                }
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            while tape[ptr] != 0 {
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                }
            ptr_wrap_add(&mut ptr);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            while tape[ptr] != 0 {
                tape[ptr] = tape[ptr].wrapping_sub(1);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                }
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            tape[ptr] = tape[ptr].wrapping_add(1);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            tape[ptr] = tape[ptr].wrapping_add(1);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            while tape[ptr] != 0 {
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                }
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            while tape[ptr] != 0 {
                tape[ptr] = tape[ptr].wrapping_sub(1);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                }
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            while tape[ptr] != 0 {
                tape[ptr] = tape[ptr].wrapping_sub(1);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_add(1);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            }
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            }
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                }
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            while tape[ptr] != 0 {
                tape[ptr] = tape[ptr].wrapping_sub(1);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                }
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            while tape[ptr] != 0 {
                tape[ptr] = tape[ptr].wrapping_sub(1);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            ptr_wrap_sub(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_sub(&mut ptr);
                            }
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_sub(1);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                }
            tape[ptr] = tape[ptr].wrapping_add(1);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            while tape[ptr] != 0 {
                tape[ptr] = tape[ptr].wrapping_sub(1);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_sub(1);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                }
            tape[ptr] = tape[ptr].wrapping_add(1);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            while tape[ptr] != 0 {
                tape[ptr] = tape[ptr].wrapping_sub(1);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_sub(1);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            ptr_wrap_sub(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_sub(&mut ptr);
                            }
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            while tape[ptr] != 0 {
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                }
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                }
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                }
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_sub(&mut ptr);
                            }
                        }
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            while tape[ptr] != 0 {
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                ptr_wrap_sub(&mut ptr);
                                }
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                }
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                ptr_wrap_add(&mut ptr);
                                }
                            ptr_wrap_add(&mut ptr);
                            while tape[ptr] != 0 {
                                tape[ptr] = tape[ptr].wrapping_sub(1);
                                }
                            tape[ptr] = tape[ptr].wrapping_add(1);
                            ptr_wrap_sub(&mut ptr);
                            }
                        }
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                tape[ptr] = tape[ptr].wrapping_add(1);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_sub(1);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                }
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            }
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        handle.write(&[tape[ptr]]).unwrap();
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        while tape[ptr] != 0 {
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            while tape[ptr] != 0 {
                tape[ptr] = tape[ptr].wrapping_sub(1);
                }
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            }
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        while tape[ptr] != 0 {
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            }
        ptr_wrap_add(&mut ptr);
        tape[ptr] = tape[ptr].wrapping_add(1);
        tape[ptr] = tape[ptr].wrapping_add(1);
        tape[ptr] = tape[ptr].wrapping_add(1);
        tape[ptr] = tape[ptr].wrapping_add(1);
        tape[ptr] = tape[ptr].wrapping_add(1);
        tape[ptr] = tape[ptr].wrapping_add(1);
        tape[ptr] = tape[ptr].wrapping_add(1);
        tape[ptr] = tape[ptr].wrapping_add(1);
        tape[ptr] = tape[ptr].wrapping_add(1);
        tape[ptr] = tape[ptr].wrapping_add(1);
        while tape[ptr] != 0 {
            tape[ptr] = tape[ptr].wrapping_sub(1);
            while tape[ptr] != 0 {
                tape[ptr] = tape[ptr].wrapping_sub(1);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                }
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            }
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        tape[ptr] = tape[ptr].wrapping_add(1);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        tape[ptr] = tape[ptr].wrapping_add(1);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        while tape[ptr] != 0 {
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            }
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        while tape[ptr] != 0 {
            tape[ptr] = tape[ptr].wrapping_sub(1);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            tape[ptr] = tape[ptr].wrapping_add(1);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            }
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        while tape[ptr] != 0 {
            tape[ptr] = tape[ptr].wrapping_sub(1);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            tape[ptr] = tape[ptr].wrapping_add(1);
            while tape[ptr] != 0 {
                tape[ptr] = tape[ptr].wrapping_sub(1);
                }
            ptr_wrap_add(&mut ptr);
            while tape[ptr] != 0 {
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                }
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            while tape[ptr] != 0 {
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        }
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                }
            }
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        while tape[ptr] != 0 {
            tape[ptr] = tape[ptr].wrapping_sub(1);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            tape[ptr] = tape[ptr].wrapping_add(1);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            }
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        while tape[ptr] != 0 {
            tape[ptr] = tape[ptr].wrapping_sub(1);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            tape[ptr] = tape[ptr].wrapping_add(1);
            ptr_wrap_add(&mut ptr);
            while tape[ptr] != 0 {
                ptr_wrap_add(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                }
            ptr_wrap_sub(&mut ptr);
            tape[ptr] = tape[ptr].wrapping_add(1);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            while tape[ptr] != 0 {
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                }
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            while tape[ptr] != 0 {
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                }
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            while tape[ptr] != 0 {
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                ptr_wrap_sub(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_sub(1);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                }
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            tape[ptr] = tape[ptr].wrapping_sub(1);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            while tape[ptr] != 0 {
                tape[ptr] = tape[ptr].wrapping_sub(1);
                }
            tape[ptr] = tape[ptr].wrapping_add(1);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            }
        tape[ptr] = tape[ptr].wrapping_add(1);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        while tape[ptr] != 0 {
            tape[ptr] = tape[ptr].wrapping_sub(1);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            tape[ptr] = tape[ptr].wrapping_sub(1);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            }
        tape[ptr] = tape[ptr].wrapping_add(1);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        ptr_wrap_sub(&mut ptr);
        while tape[ptr] != 0 {
            tape[ptr] = tape[ptr].wrapping_sub(1);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            tape[ptr] = tape[ptr].wrapping_sub(1);
            ptr_wrap_add(&mut ptr);
            while tape[ptr] != 0 {
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                }
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            while tape[ptr] != 0 {
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    }
                ptr_wrap_sub(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_sub(1);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        }
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_sub(&mut ptr);
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                }
            ptr_wrap_add(&mut ptr);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            while tape[ptr] != 0 {
                tape[ptr] = tape[ptr].wrapping_sub(1);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                }
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            tape[ptr] = tape[ptr].wrapping_add(1);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            tape[ptr] = tape[ptr].wrapping_add(1);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            while tape[ptr] != 0 {
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                }
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            while tape[ptr] != 0 {
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            }
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            }
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        }
                    }
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    }
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        tape[ptr] = tape[ptr].wrapping_sub(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        ptr_wrap_sub(&mut ptr);
                        while tape[ptr] != 0 {
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            ptr_wrap_sub(&mut ptr);
                            }
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            }
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            ptr_wrap_add(&mut ptr);
                            }
                        ptr_wrap_add(&mut ptr);
                        while tape[ptr] != 0 {
                            tape[ptr] = tape[ptr].wrapping_sub(1);
                            }
                        tape[ptr] = tape[ptr].wrapping_add(1);
                        ptr_wrap_sub(&mut ptr);
                        }
                    }
                tape[ptr] = tape[ptr].wrapping_add(1);
                ptr_wrap_add(&mut ptr);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_sub(&mut ptr);
                    while tape[ptr] != 0 {
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        ptr_wrap_add(&mut ptr);
                        }
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                }
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            while tape[ptr] != 0 {
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                }
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            while tape[ptr] != 0 {
                tape[ptr] = tape[ptr].wrapping_sub(1);
                }
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            tape[ptr] = tape[ptr].wrapping_add(1);
            while tape[ptr] != 0 {
                tape[ptr] = tape[ptr].wrapping_sub(1);
                while tape[ptr] != 0 {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    ptr_wrap_add(&mut ptr);
                    tape[ptr] = tape[ptr].wrapping_add(1);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    ptr_wrap_sub(&mut ptr);
                    }
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                ptr_wrap_add(&mut ptr);
                }
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            tape[ptr] = tape[ptr].wrapping_sub(1);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            ptr_wrap_add(&mut ptr);
            tape[ptr] = tape[ptr].wrapping_sub(1);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            ptr_wrap_sub(&mut ptr);
            while tape[ptr] != 0 {
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                ptr_wrap_sub(&mut ptr);
                }
            }
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        ptr_wrap_add(&mut ptr);
        }
}
