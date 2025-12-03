#[cfg(test)]
use core::panic;
use std::fmt::Display;
use std::fmt::Debug;
use std::usize;
use std::ops::{Add,Sub,Mul};
#[derive(Debug)]
pub struct Matrix<T> {
    data   : Vec<T>,
    rows   : usize,
    cols   : usize,
} 

 pub trait BasicOps {
    type Item:
         Add<Output = Self::Item>
        +Sub<Output = Self::Item>
        +Mul<i32,Output = Self::Item>
        +Copy; 
    fn add  (&mut self , other : &Matrix<Self::Item>);
    fn sub  (&mut self , other : &Matrix<Self::Item>);
    fn kmul (&mut self , k: i32);
}


impl<T> BasicOps for Matrix<T>
where 
    T: Add<Output=T>
      + Sub<Output=T>
      + Mul<i32, Output=T>
      + Copy,
    {
    type Item = T;
    fn add (& mut self , other : &Matrix<Self::Item>) {
        if self.rows!= other.rows || self.cols != other.cols{
            panic!("Fatal : Dimensions mismatch");
        }
        for i in 0..self.data.len(){
            self.data[i] = self.data[i] + other.data[i];
        }
    }
    fn sub  (& mut self , other : &Matrix<Self::Item>) {
        if self.rows!= other.rows || self.cols != other.cols{
            panic!("Fatal : Dimensions mismatch");
        }
        for i in 0..self.data.len(){
            self.data[i] = self.data[i] - other.data[i];
        }
    }
    fn kmul (&mut self , k: i32) {
        for i in 0..self.data.len(){
            self.data[i] =  self.data[i] * k;
        }
    }

}


impl<T> Matrix<T>
where
    T: Copy + Debug + Display,
{
    /* Constructor Member Function */
    pub fn new(r : usize , c : usize, init : T) -> Self {
        Self {
                data: vec![init; r * c],
                rows: r,
                cols: c,
         }
    }
    fn get_idx(&self, r : usize , c : usize ) -> usize {
            let idx = r + self.rows * c;
            idx
    } 
    fn print_matrix (&self) {
        for i in 0..self.data.len(){
            if (i+1) % 4 == 0{
                println!(" ");
            }
            println!("{:?}",self.data[i]);
        }
    }
}

#[cfg(test)]

impl<T> Matrix<T>
where
    T: Copy + Debug + Display,
{
    pub fn test_new(r :usize, c : usize, init : T) -> Self {
        Self {
                data: vec![init; r * c],
                rows: r,
                cols: c,
         }
    }
    pub fn test_get_idx(&self, r : usize , c : usize ) -> usize {
            let idx = r * self.cols + c;
            idx
    } 

    pub fn test_print_matrix (&self) {
        for r in 0..self.rows{
            for c in 0..self.cols{
                let idx = self.test_get_idx(r, c);
                print!("{:?} ",self.data[idx]);
            }
            println!("");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen() {
        let m = Matrix::test_new(4,34,0);  // <-- this only exists in tests
        m.test_print_matrix();
    }
    #[test]
    fn test_matrix(){
        let mut m1 = Matrix::test_new(3,1,0);
        m1.test_print_matrix();
        println!("---------------------------");
        let m2 = Matrix::test_new(3, 2, 5);
        m2.test_print_matrix();
        println!("----------------------------");
        m1.add(&m2);
        m1.test_print_matrix();
        println!("----------------------------");
    }
}

pub fn 