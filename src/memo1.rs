#[derive(Clone, Debug)]
struct S1;

#[derive(Debug)]
struct S2_1 { s1: Option<S1> }

#[derive(Debug)]
struct S2_2<'a> { s1: Option<&'a S1> }

/*
#[derive(Debug)]
struct S2_3<'a> { s1: Option<&'a mut S1> }
 */

impl S2_1 {
    fn f1(&self) -> S1 {
        // self.s1.clone().unwrap()
        self.s1.clone().unwrap()
    }
}

impl <'a> S2_2<'a> {
    fn f1(&self) -> &S1 {
        self.s1.unwrap()
    }
}

/*
impl <'a> S2_3<'a> {
    fn f1(&self) -> &S1 {
        self.s1.unwrap()
    }
}
*/

#[test]
fn test_1() {
    let s1 = S1 { };
    let s2 = S2_1 { s1: Some(s1) };
    println!("{:?}", s2.f1());
}

#[test]
fn test_2() {
    let s1 = S1 { };
    let s2 = S2_2 { s1: Some(&s1) };
    println!("{:?}", s2.f1());
}

/*
#[test]
fn test_3() {
    let mut s1 = S1 { };
    let s2 = S2_3 { s1: Some(&mut s1) };
    println!("{:?}", s2.f1());
    assert!(false)
}
*/
