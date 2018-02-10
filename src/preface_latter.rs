
#[inline]
fn ptr_wrap_add(ptr: &mut usize) {
    if *ptr == TAPE_SIZE - 1 {
        *ptr = 0;
    } else {
        *ptr += 1;
    }
}

#[inline]
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

// Intentionally no close brace here, there aren't normal source files.
