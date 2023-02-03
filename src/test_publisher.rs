use std::sync::Arc;
use std_msgs::msg::String as StringMsg;

struct RepublisherNode {
    node: rclrs::Node,
    _subscription: Arc<rclrs::Subscription<StringMsg>>,
    data: Option<StringMsg>,
}