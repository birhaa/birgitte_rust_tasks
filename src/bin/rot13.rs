
use std::io::Write;

const NUM_CHARS: usize = 26;
const HALF_NUM_CHARS: usize = NUM_CHARS/2;
const ALPABETH: [char; NUM_CHARS] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z',
];


struct Rot13Writer<T>
where T: Write
{
    writer: T
}

impl<T> Rot13Writer<T>
where T: Write{
    pub fn new(inner: T ) -> Self {
        Rot13Writer { writer: inner }
    }
}

fn switch(i: usize) -> char{
    let idx = if i < HALF_NUM_CHARS {i+HALF_NUM_CHARS} else {i-HALF_NUM_CHARS};
    ALPABETH[idx]
}

impl<T> Write for Rot13Writer<T>
where T: Write{

    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let str = buf
        .iter()
        .map(|x| *x as char)
        .map(|c| {
            let lower_case_char: char = c.to_ascii_lowercase();
            return match ALPABETH.iter().position(|&letter| letter == lower_case_char) {
                Some(i) => if c.is_uppercase() {switch(i).to_ascii_uppercase()} else {switch(i)},
                None => c
            };
        })
        .collect::<String>();
        self.writer.write(str.as_bytes())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

fn main(){
 let mut content = Vec::<u8>::default();
 let mut buff = Rot13Writer::new(&mut content);
 buff.write(b"lbh penpxrq z1 fhcre qvssvphyg pbqvat punyyratr... pbqr vf ddommNst").unwrap();
 println!("result: {:?}", content.iter().map(|x| *x as char).collect::<String>());
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use crate::Rot13Writer;

    #[test]
    fn test_rod13(){
        let mut content = Vec::<u8>::default();
        let mut buff = Rot13Writer::new(&mut content);
        buff.write(b"abcdefgklmnipqrstuvwxyz.123456789 ").unwrap();
        assert_eq!( content.iter().map(|x| *x as char).collect::<String>(), "nopqrstxyzavcdefghijklm.123456789 ")
    }
}