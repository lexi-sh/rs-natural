/*
 * This code is taken courtesy of https://github.com/mrordinaire/rust-stem
*/

use std::ascii::AsciiExt;
use std::borrow::ToOwned;
use std::str;

struct Stemmer {
    b: Vec<u8>,
    k: usize,
    j: usize 
}

impl Stemmer {
    fn new(word: &str) -> Stemmer {
        if !word.is_ascii() {
            panic!("Only support English words with ASCII characters")
        } else {
            let b = word.to_ascii_lowercase().into_bytes();
            let k = b.len();
            Stemmer {
                b: b,
                k: k,
                j: 0
            }
        }
    }

    #[inline] 
    fn is_consonant(&self, i: usize) -> bool {
        match self.b[i] {
            b'a' | b'e' | b'i' | b'o' | b'u' => false,
            b'y' => if i == 0 {
                true
            } else {
                !self.is_consonant(i - 1)
            },
            _ => true
        }
    }

    fn measure(&self) -> usize {
        let mut n = 0;
        let mut i = 0;
        let j = self.j;
        loop {
            if i >= j { return n }
            if !self.is_consonant(i) { break }
            i += 1;
        }
        i += 1;
        loop {
            loop {
                if i >= j { return n }
                if self.is_consonant(i) { break }
                i += 1;
            }
            i += 1;
            n += 1;
            loop {
                if i >= j { return n }
                if !self.is_consonant(i) { break }
                i += 1;
            }
            i += 1;
        }
    }

    fn has_vowel(&self) -> bool {
        for i in 0..self.j {
            if !self.is_consonant(i) {
                return true;
            }
        }
        false
    }

    #[inline]
    fn double_consonant(&self, i: usize) -> bool {
        if i < 1 {
            false
        } else if self.b[i] != self.b[i - 1] {
            false
        } else {
            self.is_consonant(i)
        }
    }

    fn cvc(&self, i: usize) -> bool {
        if i < 2
            || !self.is_consonant(i)
            || self.is_consonant(i - 1)
            || !self.is_consonant(i - 2)
        {
            false
        } else {
            match self.b[i] {
                b'w' | b'x' | b'y' => false,
                _ => true
            }
        }
    }

    /// stem.ends(s) is true <=> [0, k) ends with the string s.
    fn ends(&mut self, _s: &str) -> bool {
        let s = _s.as_bytes();
        let len = s.len();
        if len > self.k {
            false
        } else {
            if &self.b[self.k - len..self.k] == s {
                self.j = self.k - len;
                true
            } else {
                false
            }
        }
    }

    /// stem.setto(s) sets [j,k) to the characters in the string s,
    /// readjusting k.
    fn set_to(&mut self, s: &str) {
        let s = s.as_bytes();
        let len = s.len();
        for i in 0..(len) {
            self.b[self.j + i] = s[i];
        }
        self.k = self.j + len;
    }

    #[inline]
    fn r(&mut self, s: &str) {
        if self.measure() > 0 {
            self.set_to(s);
        }
    }

    fn step1ab(&mut self) {
        if self.b[self.k - 1] == b's' {
            if self.ends("sses") {
                self.k -= 2;
            } else if self.ends("ies") {
                self.set_to("i");
            } else if self.b[self.k - 2] != b's' {
                self.k -= 1;
            }
        }
        if self.ends("eed") {
            if self.measure() > 0 { self.k -= 1 }
        } else if (self.ends("ed") || self.ends("ing")) && self.has_vowel() {
            self.k = self.j;
            if self.ends("at") {
                self.set_to("ate");
            } else if self.ends("bl") {
                self.set_to("ble");
            } else if self.ends("iz") {
                self.set_to("ize");
            } else if self.double_consonant(self.k - 1) {
                self.k -= 1;
                match self.b[self.k - 1] {
                    b'l' | b's' | b'z' => self.k += 1,
                    _ => ()
                }
            } else if self.measure() == 1 && self.cvc(self.k - 1) {
                self.set_to("e");
            }
        }
    }

    /// stem.step1c() turns terminal y to i when there is another vowel in the stem.
    fn step1c(&mut self) {
        if self.ends("y") && self.has_vowel() {
           self.b[self.k - 1] = b'i';
        }
    }

    fn step2(&mut self) {
        match self.b[self.k - 2] {
            b'a' => {
                if self.ends("ational") { self.r("ate"); return }
                if self.ends("tional") { self.r("tion"); return }
            }
            b'c' => {
                if self.ends("enci") { self.r("ence"); return }
                if self.ends("anci") { self.r("ance"); return }
            }
            b'e' => if self.ends("izer") { self.r("ize"); return },
            b'l' => {
                if self.ends("bli") { self.r("ble"); return } /*-DEPARTURE-*/

             /* To match the published algorithm, replace this line with
                'l' => {
                    if self.ends("abli") { self.r("able"); return } */

                if self.ends("alli") { self.r("al"); return }
                if self.ends("entli") { self.r("ent"); return }
                if self.ends("eli") { self.r("e"); return }
                if self.ends("ousli") { self.r("ous"); return }
            }
            b'o' => {
                if self.ends("ization") { self.r("ize"); return }
                if self.ends("ation") { self.r("ate"); return }
                if self.ends("ator") { self.r("ate"); return }
            }
            b's' => {
                if self.ends("alism") { self.r("al"); return }
                if self.ends("iveness") { self.r("ive"); return }
                if self.ends("fulness") { self.r("ful"); return }
                if self.ends("ousness") { self.r("ous"); return }
            }
            b't' => {
                if self.ends("aliti") { self.r("al"); return }
                if self.ends("iviti") { self.r("ive"); return }
                if self.ends("biliti") { self.r("ble"); return }
            }
            b'g' => if self.ends("logi") { self.r("log"); return }, /*-DEPARTURE-*/
             /* To match the published algorithm, delete this line */
            _ => ()
        }
    }

    fn step3(&mut self) {
        match self.b[self.k - 1] {
            b'e' => {
                if self.ends("icate") { self.r("ic"); return }
                if self.ends("ative") { self.r(""); return }
                if self.ends("alize") { self.r("al"); return }
            }
            b'i' => if self.ends("iciti") { self.r("ic"); return },
            b'l' => {
                if self.ends("ical") { self.r("ic"); return }
                if self.ends("ful") { self.r(""); return }
            }
            b's' => if self.ends("ness") { self.r(""); return },
            _ => ()
        }
    }

    fn step4(&mut self) {
        match self.b[self.k - 2] {
            b'a' => {
                if self.ends("al") {}
                else { return }
            }
            b'c' => {
                if self.ends("ance") {}
                else if self.ends("ence") {}
                else { return }
            }
            b'e' => {
                if self.ends("er") {}
                else { return }
            }
            b'i' => {
                if self.ends("ic") {}
                else { return }
            }
            b'l' => {
                if self.ends("able") {}
                else if self.ends("ible") {}
                else { return }
            }
            b'n' => {
                if self.ends("ant") {}
                else if self.ends("ement") {}
                else if self.ends("ment") {}
                else if self.ends("ent") {}
                else { return }
            }
            b'o' => {
                if self.ends("ion") &&
                    (self.b[self.j - 1] == b's' || self.b[self.j - 1] == b't') {}
                else if self.ends("ou") {}
                else { return }
                /* takes care of -ous */
            }
            b's' => {
                if self.ends("ism") {}
                else { return }
            }
            b't' => {
                if self.ends("ate") {}
                else if self.ends("iti") {}
                else { return }
            }
            b'u' => {
                if self.ends("ous") {}
                else { return }
            }
            b'v' => {
                if self.ends("ive") {}
                else { return }
            }
            b'z' => {
                if self.ends("ize") {}
                else { return }
            }
            _ => return
        }
        if self.measure() > 1 { self.k = self.j }
    }

    fn step5(&mut self) {
        self.j = self.k;
        if self.b[self.k - 1] == b'e' {
            let a = self.measure();
            if a > 1 || a == 1 && !self.cvc(self.k - 2) { self.k -= 1 }
        }
        if self.b[self.k - 1] == b'l' &&
            self.double_consonant(self.k - 1) &&
            self.measure() > 1
        {
            self.k -= 1;
        }
    }

    #[inline]
    fn get(&self) -> String {
        unsafe {
            str::from_utf8_unchecked(&self.b[..self.k]).to_owned()
        }
    }
}

pub fn stem(word: &str) -> String {
    if word.len() > 2 {
        let mut mw = Stemmer::new(word);
        mw.step1ab();
        mw.step1c();
        mw.step2();
        mw.step3();
        mw.step4();
        mw.step5();
        mw.get()
    } else {
        word.to_owned()
    }
}