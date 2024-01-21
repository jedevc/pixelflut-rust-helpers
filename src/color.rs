#![allow(unused)]

pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    pub fn hex(&self) -> String {
        format!("{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RGBA {
    pub fn hex(&self) -> String {
        format!("{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, self.a)
    }
}

pub struct HSV {
    pub h: u8,
    pub s: u8,
    pub v: u8,
}

impl HSV {
    pub fn hex(&self) -> String {
        self.to_rgb().hex()
    }

    fn to_rgb(&self) -> RGB {
        let (h, s, v) = (
            self.h as f64 / 255.0,
            self.s as f64 / 255.0,
            self.v as f64 / 255.0,
        );
        let (r, g, b) = hsl_to_rgb(h, s, v);
        RGB {
            r: (r * 255.0) as u8,
            g: (g * 255.0) as u8,
            b: (b * 255.0) as u8,
        }
    }
}

pub struct HSVA {
    pub h: u8,
    pub s: u8,
    pub v: u8,
    pub a: u8,
}

impl HSVA {
    pub fn hex(&self) -> String {
        self.to_rgba().hex()
    }

    fn to_rgba(&self) -> RGBA {
        let rgb = HSV {
            h: self.h,
            s: self.s,
            v: self.v,
        }
        .to_rgb();
        RGBA {
            r: rgb.r,
            g: rgb.g,
            b: rgb.b,
            a: self.a,
        }
    }
}

fn hue_to_rgb(p: f64, q: f64, t: f64) -> f64 {
    let mut t = t;
    if t < 0.0 {
        t += 1.0
    }
    if t > 1.0 {
        t -= 1.0
    }
    if t < 1.0 / 6.0 {
        p + (q - p) * 6.0 * t
    } else if t < 1.0 / 2.0 {
        q
    } else if t < 2.0 / 3.0 {
        p + (q - p) * (2.0 / 3.0 - t) * 6.0
    } else {
        p
    }
}

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (f64, f64, f64) {
    if s == 0.0 {
        (l, l, l)
    } else {
        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };
        let p = 2.0 * l - q;
        (
            hue_to_rgb(p, q, h + 1.0 / 3.0),
            hue_to_rgb(p, q, h),
            hue_to_rgb(p, q, h - 1.0 / 3.0),
        )
    }
}
