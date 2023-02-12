/*
lets make 3 dummy functions
then lets put those in a call list
then lets call them all
and collect the results
*/

pub fn f1() -> i32 {
    1
}

pub fn f2() -> i32 {
    2
}

pub fn f3() -> i32 {
    3
}

fn main() {
    let mut call_list: Vec<fn() -> i32> = Vec::new();
    call_list.push(f1);
    call_list.push(f2);
    call_list.push(f3);

    let mut results = Vec::new();
    for f in call_list {
        results.push(f());
    }

    println!("{:?}", results);
}
