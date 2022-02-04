#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    ///头像
    #[prost(string, tag="1")]
    pub avatar: ::prost::alloc::string::String,
    /// 发送消息用户的用户名
    #[prost(string, tag="2")]
    pub from_username: ::prost::alloc::string::String,
    /// 发送消息用户uuid
    #[prost(string, tag="3")]
    pub from: ::prost::alloc::string::String,
    /// 发送给对端用户的uuid
    #[prost(string, tag="4")]
    pub to: ::prost::alloc::string::String,
    /// 文本消息内容
    #[prost(string, tag="5")]
    pub content: ::prost::alloc::string::String,
    /// 消息内容类型：1.文字 2.普通文件 3.图片 4.音频 5.视频 6.语音聊天 7.视频聊天
    #[prost(int32, tag="6")]
    pub content_type: i32,
    /// 消息传输类型：如果是心跳消息，该内容为heatbeat,在线视频或者音频为webrtc
    #[prost(string, tag="7")]
    pub r#type: ::prost::alloc::string::String,
    /// 消息类型，1.单聊 2.群聊
    #[prost(int32, tag="8")]
    pub message_type: i32,
    /// 图片，视频，语音的路径
    #[prost(string, tag="9")]
    pub url: ::prost::alloc::string::String,
    /// 文件后缀，如果通过二进制头不能解析文件后缀，使用该后缀
    #[prost(string, tag="10")]
    pub file_suffix: ::prost::alloc::string::String,
    /// 如果是图片，文件，视频等的二进制
    #[prost(bytes="vec", tag="11")]
    pub file: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MessageType {
    Reserve = 0,
    Text = 1,
    File = 2,
    Image = 3,
    Audio = 4,
    Video = 5,
    AudioOnline = 6,
    VideoOnline = 7,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MessageUserType {
    Reserved = 0,
    User = 1,
    Group = 2,
}
