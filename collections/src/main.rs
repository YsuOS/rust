fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2);
    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
}
