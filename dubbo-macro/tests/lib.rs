


#[test]
fn tests() {

    // mod _check_stream_type {
    //     use std::marker::PhantomData;

 
    //     use futures::Stream;

    //     trait NotImplStream {
    //         const IMPLS: bool = false;
    //     }

    //     impl<T: ?Sized> NotImplStream for T {}

    //     struct Wrapper<T: ?Sized>(PhantomData<T>);

    //     impl<T: ?Sized + Stream> Wrapper<T> {
    //         const IMPLS: bool = true;
    //     }


    //     pub(in super) fn check_stream_type<T: ?Sized>() -> bool {
    //         Wrapper::<T>::IMPLS
    //     }
    // }

    // let s = _check_stream_type::check_stream_type::<String>();

    // println!("is stream: {}", s)
    
}



#[dubbo_macro::reference]
pub trait RemoteService {

    
    #[dubbo_macro::rpc_call(tt = "sync", rt = "str")]
    fn test_async(&mut self, name: String) -> Result<String, Box<dyn std::error::Error + Send + Sync>> ;
    
}
