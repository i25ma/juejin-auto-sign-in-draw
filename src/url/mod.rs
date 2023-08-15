// 基础url
pub const BASE_URL: &str = "https://api.juejin.cn";

// 查询今日是否已经签到
pub const ISSIGNINURL: &str = "/growth_api/v1/get_today_status";

// 查询当前拥有矿石数量
pub const GETCURPOINT : &str = "/growth_api/v1/get_cur_point";

// 签到
pub const SIGNINURL : &str = "/growth_api/v1/check_in";

// 查询今日免费抽奖机会
pub const ISDRAW : &str = "/growth_api/v1/lottery_config/get";

// 抽奖
pub const DRAWURL : &str = "/growth_api/v1/lottery/draw";
