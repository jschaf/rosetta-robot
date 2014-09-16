// Implements http://rosettacode.org/wiki/SHA-1
// straight port from golang crypto/sha1
// library implementation

use std::io::IoResult;
use std::slice::bytes::copy_memory;

// The size of a SHA1 checksum in bytes.
static size: uint = 20;

// The blocksize of SHA1 in bytes.
static chunk:uint = 64;
static init:[u32,..5] = [0x67452301,0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0];

#[cfg(not(test))]
fn main() {
	let mut d = Digest::new();
	d.write(b"The quick brown fox jumps over the lazy dog").unwrap();
    let sha1=d.sha1();

    for h in sha1.iter() {
        print!("{:x} ", *h);
    }
 }

// digest represents the partial evaluation of a checksum.
struct Digest {
	h:      [u32, ..5],
	x:      [u8, ..chunk],
	nx:     uint,
	len:    u64
}

impl Digest {
    fn new() -> Digest {
        Digest {
            h:  init,
            x:  [0u8, ..chunk],
            nx: 0u,
            len:0u64
        }
    }

    fn sha1(&mut self) -> [u8,..size] {
        let mut len = self.len;
        // Padding.  Add a 1 bit and 0 bits until 56 bytes mod 64.
        let mut tmp : [u8,..64] = [0u8,..64];
        tmp[0] = 0x80u8;

        let m:uint=(len%64u64) as uint;
        if m < 56 {
            self.write(tmp.slice(0u, 56-m)).unwrap();
        } else {
            self.write(tmp.slice(0u, 64+56-m)).unwrap();
        }

        // Length in bits (=lengh in bytes*8=shift 3 bits to the right).
        len = len << 3;
        for i in range (0u, 8) {
            tmp[i] = (len >> (56u - 8*i)) as u8;
        }
        self.write(tmp.slice(0,8)).unwrap();

        assert!(self.nx == 0);

        let mut digest : [u8,..size]=[0u8,..size];
        for (i, s) in self.h.iter().enumerate() {
            digest[i*4] = (*s >> 24) as u8;
            digest[i*4+1] = (*s >> 16) as u8;
            digest[i*4+2] = (*s >> 8) as u8;
            digest[i*4+3] = *s as u8;
        }
        digest
    }

    fn process_block(&self, data:&[u8]) ->  [u32, ..5]{
        let k:[u32,..4] = [0x5A827999, 0x6ED9EBA1, 0x8F1BBCDC, 0xCA62C1D6];

        #[inline]
        fn part(a: u32, b: u32) -> (u32, u32) {
            (a<<5 | a>>(32-5), b<<30 | b>>(32-30))
        }

        let mut w :[u32, ..16] = [0u32, ..16];

        let (mut h0, mut h1, mut h2, mut h3, mut h4) =
            (self.h[0], self.h[1], self.h[2], self.h[3], self.h[4]);

        let mut p = data;

        while p.len() >= chunk {
            for i in range(0u, 16) {
                let j = i * 4;
                w[i] =  (p[j]   as u32)<<24 |
                        (p[j+1] as u32)<<16 |
                        (p[j+2] as u32) <<8 |
                         p[j+3] as u32;
            }

            let (mut a, mut b, mut c, mut d, mut e) = (h0, h1, h2, h3, h4);

            for i in range(0u, 16) {
                let f = b & c | (!b) & d;
                let (a5, b30) = part(a, b);
                let t = a5 + f + e + w[i&0xf] + k[0];
                 b=a; a=t; e=d; d=c; c=b30;
            }
            for i in range(16u, 20) {
                let tmp = w[(i-3)&0xf] ^ w[(i-8)&0xf] ^ w[(i-14)&0xf] ^ w[(i)&0xf];
                w[i&0xf] = tmp<<1 | tmp>>(32-1);
                let f = b & c | (!b) & d;
                let (a5, b30) = part(a, b);
                let t = a5 + f + e + w[i&0xf] + k[0];
                b=a; a=t; e=d; d=c; c=b30;
            }
            for i in range(20u, 40) {
                let tmp = w[(i-3)&0xf] ^ w[(i-8)&0xf] ^ w[(i-14)&0xf] ^ w[(i)&0xf];
                w[i&0xf] = tmp<<1 | tmp>>(32-1);
                let f = b ^ c ^ d;
                let (a5, b30) = part(a, b);
                let t = a5 + f + e + w[i&0xf] + k[1];
                b=a; a=t; e=d; d=c; c=b30;
            }
            for i in range(40u, 60) {
                let tmp = w[(i-3)&0xf] ^ w[(i-8)&0xf] ^ w[(i-14)&0xf] ^ w[(i)&0xf];
                w[i&0xf] = tmp<<1 | tmp>>(32-1);
                let f = ((b | c) & d) | (b & c);
                let (a5, b30) = part(a, b);
                let t = a5 + f + e + w[i&0xf] + k[2];
                b=a; a=t; e=d; d=c; c=b30;
            }
            for i in range(60u, 80) {
                let tmp = w[(i-3)&0xf] ^ w[(i-8)&0xf] ^ w[(i-14)&0xf] ^ w[(i)&0xf];
                w[i&0xf] = tmp<<1 | tmp>>(32-1);
                let f = b ^ c ^ d;
                let (a5, b30) = part(a, b);
                let t = a5 + f + e + w[i&0xf] + k[3];
                b=a; a=t; e=d; d=c; c=b30;
            }
            h0 += a;
            h1 += b;
            h2 += c;
            h3 += d;
            h4 += e;

            p = p.slice_from(chunk);
        }
        [h0, h1, h2, h3, h4]
    }
}

impl Writer for Digest {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> IoResult<()> {
        let mut buf_m = buf;

        self.len += buf_m.len() as u64;

        if self.nx > 0 {
            let mut n = buf_m.len();
            if n > chunk - self.nx {
                n = chunk - self.nx;
            }
            for i in range(0,n) {
                self.x[self.nx + i] = *buf_m.get(i).unwrap();
            }
            self.nx += n;
            if self.nx == chunk {
                let x = self.x.as_slice();
                self.h=self.process_block(x);
                self.nx = 0;
            }
            buf_m = buf_m.slice_from(n);
        }
        if buf_m.len() >= chunk {
            let n = buf_m.len() &!(chunk - 1);
            let x = self.x.slice_from(n);
            self.h=self.process_block(x);
            buf_m = buf_m.slice_from(n);
        }
        let ln=buf_m.len();
        if ln > 0 {
            assert!(self.x.len() >= ln);
            copy_memory(self.x, buf_m);
            self.nx = ln;
        }
        Ok(())
    }
}

#[test]
fn known_sha1s() {
   let input_output = vec![
        (
            "His money is twice tainted: 'taint yours and 'taint mine.",
            vec!(0x59u8, 0x7f, 0x6a, 0x54, 0x0, 0x10, 0xf9, 0x4c,
                0x15, 0xd7, 0x18, 0x6, 0xa9, 0x9a, 0x2c, 0x87, 0x10,
                0xe7, 0x47, 0xbd)
         ),
        (
            "The quick brown fox jumps over the lazy dog",
            vec!(0x2fu8, 0xd4, 0xe1, 0xc6, 0x7a, 0x2d,
            0x28, 0xfc, 0xed, 0x84, 0x9e, 0xe1, 0xbb, 0x76
            , 0xe7, 0x39, 0x1b, 0x93, 0xeb, 0x12)
         ),
        (
            "The quick brown fox jumps over the lazy cog",
            vec!(0xdeu8 ,0x9f ,0x2c ,0x7f ,0xd2 ,0x5e ,0x1b ,0x3a
            ,0xfa ,0xd3 ,0xe8 ,0x5a ,0x0b ,0xd1 ,0x7d ,0x9b
            ,0x10 ,0x0d ,0xb4,0xb3)
         )];

    for &(ref i, ref o) in input_output.iter() {
        let i = i.as_slice();
        let o = o.as_slice();

        let mut d = Digest::new();
        d.write_str(i).unwrap();
        let sha1=d.sha1();

        assert_eq!(sha1.as_slice(), o);
    }
}
