fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    println!("{:?}", v);

    // ###############
    let v2 = vec![1, 2, 3];
    // let v2 = vec![1, "Str", 3]; error
    println!("{:?}", v2);

    // ###############
    let v = vec![1, 2, 3];
    let value1 = v.get(0); //v[0]
    let value2 = v.get(100); // v[100] error
    println!("value1 {:?}", value1);
    println!("value2 {:?}", value2);

    // ###############
    let mut v = vec![1, 2, 3, 4];
    for i in &mut v {
        *i *= 2;
        println!("i {}", i);
    }
    println!("value2 {:?}", v); // ถ้า loop ไม่ใส่ & ตรงนี้จะ print ไม่ได้ เพราะ owner ย้ายไปใน func
}
