use std::cmp::Ordering;

fn main() {
    let mut arr = vec![1,2];
    let mut i = 0;
    let stop: i32 = 520; // STOP AT THE NEXT GREATER ITERATION
    
    // LOOP

    loop {
        arr.push(arr[i] + arr[i +1]);
        i+=1;
        println!("{:?}", arr);

        match arr[i].cmp(&stop) {
            Ordering::Less => continue,
            Ordering::Greater => break,
            Ordering::Equal => break,
        }

    }

}