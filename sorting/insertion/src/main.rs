// TODO: consider removing the hard-coded variables like 5, the array, and check length >=2
fn main() {
    let mut unsorted: [i32; 5] = [1, 4, 5, 10, 2];

    insertion_sort(&mut unsorted);
    println!("{:?}", unsorted);
}

fn insertion_sort(a: &mut [i32; 5]) -> &mut [i32; 5] {
    for i in 2..a.len() {
        let mut value = a[i];
        // Insert a[i] into sorted subarray a[1:i-1]
        let mut j = i - 1;
        while j > 0 && a[j] > value {
            a[j + 1] = a[j];
            j = j - 1;
        }
        a[j + 1] = value;
    }
    a
}
