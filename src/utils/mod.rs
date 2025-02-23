use std::fs::File;
use std::io;

// dyn表示动态类型，可以包裹实现某一方法的不同的类型
pub fn get_reader(input: &str) -> anyhow::Result<Box<dyn io::Read>> {
    if input == "-" {
        Ok(Box::new(io::stdin().lock()))
    } else {
        Ok(Box::new(File::open(input)?))
    }
}