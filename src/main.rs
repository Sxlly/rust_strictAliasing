// Shae Sullivan
// 20643731
// Programming Languages - Assignment
// 21/09/24


use std::hint::unreachable_unchecked; // import unchecker from standard library




fn main() { // declare main method


    unsafe fn character_to_integer(character: char) -> u8 { //decalre unsafe function -> takes parameter character -> DT char -> return unsigned 8 bit

        match character { //match case takes -> character 
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            _ => unsafe {
                    unreachable_unchecked() //calls unreachable_unchecked method -> informs compilaer that site function is callable isnt reachable
                },
        }
    }

    unsafe { character_to_integer('1') }; //makes unsafe call to character_to_integer ->  with char parameter '1'


}
