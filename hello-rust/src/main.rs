use std::fmt;

// #[derive(Debug)]; 如果自定义实现，则这个要注释掉，也就是不自动推导出Debug函数
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.0, self.1)
    }
}

impl fmt::Debug for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "debuged: {}, {}", self.0, self.1)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    image: f64
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}", self.real, self.image)
    }
}

fn main() {
    let min_max = MinMax(3, 5);
    let complex: Complex = Complex {real: 3.3, image: 7.2};
    println!("Display {:}", min_max);
    println!("Debug {:?}", min_max);
    println!("Display: {:}", complex);
    println!("Debug: {:?}", complex);
}