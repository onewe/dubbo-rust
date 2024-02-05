


#[test]
fn tests() {
   
}


#[dubbo_macro::reference(interface_name = "RemoteService", ser = "json")]
pub trait RemoteService {

    fn say_hello<T, R, 'a>(&self, name: String, t1: T, r1: &'a R) -> String 
    where 
        T: Clone, 
        R: Clone + 'a;

    fn test_async(&self, name: String) -> String;
    
}
