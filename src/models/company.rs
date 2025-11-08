use url::Url;

#[derive(Debug)]
pub struct Company {
    pub id: Option<u32>,
    pub name: Option<String>,
    pub address: Option<String>,
    pub website: Option<Url>,
    pub phone: Option<String>,
}
