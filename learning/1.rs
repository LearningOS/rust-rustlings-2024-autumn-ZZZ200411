pub struct NewsArticle{
    pub author:String,
    pub headline:String,
    pub content:String,
}

pub struct Tweet{
    pub usename:String,
    pub content:String,
    pub reply:bool,
    pub retweet:bool,
}

pub trait Summary{
    fn summarize(&self) ->String{
        String::from("Readamore...")  //缺省实现
    }
}