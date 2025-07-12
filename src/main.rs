struct Pair <T, U>{
    value1: T,
    value2: U,
}


impl<T, U> Pair<T, U>{
   fn value1(&self) -> &T {
        &self.value1;
   }
   fn value2(&self) -> &U {
        &self.value2;
   }
   fn swap(self)=> Pair<U, T>{
    Pair{
        value1: self.value2,
        value2: self.value1,
    }
   }
}

fn main(){
    let pair = Pair{value1: 10, value2: "World"};


    println!("Value 1: {}", pair.value1());
    println!("Value 2: {}", pair.value2());


    let swapped pair = pair.swap();

    println!("After swap - Value 1: {}, Value 2: {}", swapped_pair.value1, swapped_pair.value2);
}