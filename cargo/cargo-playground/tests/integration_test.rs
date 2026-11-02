use cargo_playground::greet;


#[test]
fn test_greet(){
    greet();
    assert_eq!(2+2,4);
    assert_ne!(2+2,5);
}

