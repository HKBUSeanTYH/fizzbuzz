fn main() {
    let triggers = vec![Trigger::new(String::from("Fizz"), Box::new(|x| {x % 5 == 0})), 
                                      Trigger::new(String::from("Buzz"), Box::new(|x|{ x % 7 == 0}))];
    fizzbuzz(0..=10, triggers);
}

struct Trigger{
    predicate: Box<dyn Fn(i32) -> bool>,
    /*
        > no two closures, even if identical, have the same type
        > consider boxing your closure and/or using it as a trait object
        because of this, use a Box with trait object in order to store the closure inside Trigger
    */
    output: String,
}

impl Trigger {
    fn new(output: String, predicate: Box<dyn Fn(i32)->bool>) -> Self {
        Trigger { output, predicate }
    }
}

//generic funtion that takes any iterator that iterates i32s (this handles Range and RangeInclusive)
fn fizzbuzz<T: Iterator<Item=i32>>(range: T, triggers: Vec<Trigger>) {
    for i in range {
        let mut output = String::new();
        for elem in triggers.iter() {
            if (elem.predicate)(i) {
                output.push_str(&elem.output);
            }
            /*
                FnOnce can only be called once. cannot move out of `elem.predicate` which is behind a shared reference.
                The predicate does not capture and can be called multiple times, change to Fn to fix

                Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter: 
                - borrowing immutably 
                - borrowing mutably
                - taking ownership

                Fn - Borrows arguments and captured variables from environment immutably
                FnMut - Borrows arguments and captured variables from environment mutably
                FnOnce - Takes ownership of arguments and captured variables from environment
            */
        }
        println!("{i}: {output}");
    }
}