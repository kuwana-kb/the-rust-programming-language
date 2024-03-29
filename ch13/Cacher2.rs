struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        Some(v) => v,
        None => {
            let v = (self.calculation)(args);
            self.value = Some(v);
            v
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::New(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);
    assert_eq!(v2, 2);
}







// struct Cacher<T>
//     where T: Fn(u32) -> u32 
// {
//     calculation: T,
//     value: Option<u32>,
// }

// impl<T> Cacher<T> 
//     where T: Fn(u32) -> u32
// {
//     fn new(calculation: T) -> Cacher<T> {
//         Cacher {
//             calculation,
//             value: None,
//         }
//     }

//     fn value(&mut self, arg: u32) -> u32 {
//         match self.value {
//             Some(v) => v,
//             None => {
//                 let v = (self.calculation)(arg);
//                 self.value = Some(v);
//                 v
//             },
//         }
//     }
// }
