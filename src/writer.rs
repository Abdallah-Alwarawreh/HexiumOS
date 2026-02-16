#![allow(clippy::needless_return, clippy::identity_op, clippy::many_single_char_names)]
use crate::vga_colors::{Color, color_code};
use core::arch::asm;

const V: usize = 0xb8000;
const W: usize = 80;
const H: usize = 25;
const C: u16 = 0x3D4;
const D: u16 = 0x3D5;

pub struct Writer {
    c: usize,
    r: usize,
    clr: u8,
}

#[inline(always)]
unsafe fn ob(p: u16, v: u8) {
    asm!("out dx, al", in("dx") p, in("al") v, options(nostack));
}

impl Writer {
    pub const fn new(clr: u8) -> Self {
        Writer { c: 0, r: 0, clr }
    }

    #[inline(never)]
    fn uc(&self) {
        let p = self.r.wrapping_mul(W).wrapping_add(self.c);
        unsafe {
            ob(C, 0x0F);
            ob(D, (p & 0xFF) as u8);
            ob(C, 0x0E);
            ob(D, ((p >> 8) & 0xFF) as u8);
        }
    }

    pub fn en_cur(&self) {
        unsafe {
            ob(C, 0x0A);
            ob(D, 14);
            ob(C, 0x0B);
            ob(D, 15);
        }
        self.uc();
    }

    pub fn wb(&mut self, b: u8) {
        match b {
            b'\n' => self.nl(),
            _ => {
                if self.c >= W { self.nl(); }
                let off = (self.r * W + self.c) << 1;
                unsafe {
                    let ptr = V as *mut u8;
                    *ptr.add(off) = b;
                    *ptr.add(off + 1) = self.clr;
                }
                self.c += 1;
            }
        }
        self.uc();
    }

    pub fn ws(&mut self, s: &str) {
        s.bytes().for_each(|b| 
            if (0x20..=0x7e).contains(&b) || b == b'\n' { self.wb(b); } 
            else { self.wb(0xfe) }
        );
    }

    pub fn wbs(&mut self, s: &[u8]) {
        for &b in s {
            if (0x20..=0x7e).contains(&b) || b == b'\n' { self.wb(b); }
            else { self.wb(0xfe); }
        }
    }

    fn nl(&mut self) {
        self.c = 0;
        if self.r < H - 1 {
            self.r += 1;
        } else {
            self.scr();
        }
    }

    fn scr(&mut self) {
        unsafe {
            let ptr = V as *mut u8;
            (1..H).for_each(|row| 
                (0..W).for_each(|col| {
                    let src = (row * W + col) << 1;
                    let dst = ((row - 1) * W + col) << 1;
                    *ptr.add(dst) = *ptr.add(src);
                    *ptr.add(dst + 1) = *ptr.add(src + 1);
                })
            );
            (0..W).for_each(|col| {
                let off = ((H - 1) * W + col) << 1;
                *ptr.add(off) = b' ';
                *ptr.add(off + 1) = self.clr;
            });
        }
    }

    pub fn clr(&mut self) {
        unsafe {
            let ptr = V as *mut u8;
            (0..W*H).for_each(|i| {
                *ptr.add(i<<1) = b' ';
                *ptr.add((i<<1)+1) = self.clr;
            });
        } 
        self.c = 0;
        self.r = 0;
    }

    pub fn scl(&mut self, fg: Color, bg: Color) {
        self.clr = color_code(fg, bg);
    }

    pub fn gc(&self) -> usize { self.c }
    pub fn gr(&self) -> usize { self.r }

    pub fn sp(&mut self, c: usize, r: usize) {
        self.c = c; self.r = r; self.uc();
    }
}
