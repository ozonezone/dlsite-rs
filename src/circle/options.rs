use crate::search::{macros::*, options::Order};

#[derive(Default)]
pub struct CircleQueryOptions {
    /// Display lang
    pub order: Option<Order>,
    pub options: Option<Vec<String>>,
    /// 30, 50 or 100
    pub per_page: Option<u32>,
    pub page: Option<u32>,
}

impl CircleQueryOptions {
    pub(super) fn to_path(&self, circle_id: &str) -> String {
        let mut path = "/circle/profile/=".to_string();

        push_option_array!(path, self, options);
        push_option!(path, self, per_page);
        push_option!(path, self, per_page);
        // TODO: Investigate this
        path.push_str("/show_type/3/hd/1/without_order/1");
        push_option!(path, self, page);
        path.push_str("/maker_id/");
        path.push_str(circle_id);
        path.push_str(".html");
        push_option!(path, self, order);

        path
    }
}
#[cfg(test)]
mod tests {
    use crate::search::options::Order;

    #[test]
    fn circle_param() {
        assert_eq!(
            "/circle/profile/=/show_type/3/hd/1/without_order/1/maker_id/RG24350.html",
            super::CircleQueryOptions::default().to_path("RG24350")
        );
    }
    #[test]
    fn circle_param_1() {
        assert_eq!(
            "/circle/profile/=/show_type/3/hd/1/without_order/1/page/2/maker_id/RG24350.html",
            super::CircleQueryOptions {
                page: Some(2),
                ..Default::default()
            }
            .to_path("RG24350")
        );
    }
    #[test]
    fn circle_param_2() {
        assert_eq!(
            "/circle/profile/=/show_type/3/hd/1/without_order/1/page/2/maker_id/RG24350.html/order/price",
            super::CircleQueryOptions {
                page: Some(2),
                order: Some(Order::Price),
                ..Default::default()
            }
            .to_path("RG24350")
        );
    }
}
