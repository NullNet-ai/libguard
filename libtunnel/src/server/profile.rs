use std::net::SocketAddr;

pub trait Profile {
    // @TODO: ProfileId: IntoHash
    //type ProfileId;

    fn get_unique_id(&self) -> String;

    fn get_visitor_addr(&self) -> SocketAddr;

    fn get_visitor_token(&self) -> Option<String>;
}
