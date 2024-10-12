use std::hint::unreachable_unchecked;




fn main() {


    unsafe fn character_to_integer(character: char) -> u8 {

        match character {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            _ => unsafe {
                    unreachable_unchecked()
                },
        }
    }

    unsafe { character_to_integer('1') };


}
