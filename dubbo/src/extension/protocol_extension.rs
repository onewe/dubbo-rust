use async_trait::async_trait;
use crate::extension::invoker_extension::Invoker;
use crate::Url;

// url: transport://0.0.0.0?name=dubbo&service-name=hello&protocol=dubbo
// extension_url: extension://0.0.0.0?extension-type=transport&extension-name=dubbo&transport-url=transport://0.0.0.0?name=dubbo&service-name=hello&protocol=dubbo
#[async_trait]
pub trait Transport {

    async fn reference(&mut self, url: Url) -> Box<dyn Invoker + Send + 'static>;

    fn url(&self) -> &Url;

    fn clone(&self) -> Box<dyn Transport + Send + 'static>;

}