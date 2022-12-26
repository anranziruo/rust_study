#[derive(Clone, Debug)]
struct VecLine {
    line:i32
}

#[test]
fn test_vec_line(){
    let mut datas = Vec::new();
    datas.push(VecLine{line:2});
    datas.push(VecLine{line:2});
    println!("{:?}",datas);
    let v = VecLine{line:3};
    datas.insert(1, v);
    println!("{:?}",datas);
}