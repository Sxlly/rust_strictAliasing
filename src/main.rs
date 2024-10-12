use std::hint::unreachable_unchecked;




fn main() { // declare main method


    unsafe fn character_to_integer(character: char) -> u8 { //decalre unsafe function -> takes parameter character -> DT char -> return unsigned 8 bit

        match character { //match case takes -> character 
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
