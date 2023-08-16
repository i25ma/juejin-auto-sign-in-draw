
mod param;
mod url;

use param::*;
use reqwest::header::{self, COOKIE};
use reqwest::Client;
use tokio::time;
use url::*;
use dotenv::dotenv;
use std::env;


type RespError = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), RespError> {

     // 在访问环境变量之前检查一下，防止因读取环境变量失败导致程序恐慌。
    // 先把 dotenv 导入，然后在程序开始的地方执行 dotenv() 函数即可，这就会从当前目录或父目录中的 .env 文件中加载环境变量。
    // 如果你想指定其它路径，可以使用 crate 中提供的 from_filename 或 from_path 这两个函数。
    // 好，那么调用 dotenv() 之后为什么还要调用 ok() 方法？
    // 首先，dotenv() 返回的是 Result<PathBuf> 类型，如果返回值不使用的话，就会发出一个警告：
    // 调用 ok() 之后，会把 Result 转化为 Option，而 Option 就不会产生未使用 Result 的警告了。
    // 那么，为什么不使用 unwrap()？
    // 因为在生产环境中，你不会使用 .env 这个文件，你应该使用真实的环境变量，这时 dotenv() 函数就会加载失败，如果使用 unwrap()，那么你的程序就会停止运行。
    // 所以这里使用 ok() 的目的就是当加载 dotenv 环境文件失败的时候可以忽略错误。
    dotenv().ok();

    let aid: String = env::var("AID").expect("AID 没有在 .env文件中设置");
    let uuid: String = env::var("UUID").expect("UUID 没有在 .env 文件中设置");
    let signature: String = env::var("SIGNATURE").expect("SIGNATURE 没有在 .env 文件中设置");
    let _cookie: String = env::var("_COOKIE").expect("_COOKIE  没有在 .env 文件中设置");

    let params = Post {
        aid: aid.as_str(),
        uuid: uuid.as_str(),
        _signature: signature.as_str(),
        cookie: _cookie.as_str(),
    };

    println!("{:#?}",params);
    let client = init(&params).unwrap();

    let sign_resp = is_sign_in(client.clone()).await?;

    if let Some(false) = sign_resp.data {
        sign_in(client.clone(), &params).await?;

        time::sleep(time::Duration::from_secs(5)).await;
    };

    let draw_resp = is_draw(client.clone()).await?;
    if draw_resp.data.free_count != 0 {
        draw(client.clone(), &params).await?;
    }

    let get_cur_point = get_cur_point(client.clone()).await?;
    println!("get_cur_point:{:#?}", get_cur_point);
    if get_cur_point.data >= 1 {
        let num = (get_cur_point.data / 200) as i32;
        println!("{num}");
        if num > 10 {
            let ten_num = (num / 10) as i32;
            for i in 0..ten_num {
                let draw_num = i + 1;
                println!("十连抽 抽了{draw_num} 次奖");
                ten_draw(client.clone(), &params).await?;
            }
        } else {
            for i in 0..num {
                let draw_num = i + 1;
                println!("单抽 抽了{draw_num} 次奖");
                draw(client.clone(), &params).await?;
            }
        }
    }  
    Ok(())
}

// 初始化reqwest 客户端
fn init(params: &Post) -> Result<Client, RespError> {
    let mut headers = header::HeaderMap::new();

    headers.insert(COOKIE, params.cookie.parse().unwrap());

    let client = Client::builder().default_headers(headers).build()?;
    Ok(client)
}

// 是否已签到
async fn is_sign_in(client: Client) -> Result<SignResp, RespError> {
    let resp = client
        .get(BASE_URL.to_string() + ISSIGNINURL)
        .send()
        .await?
        .json::<SignResp>()
        .await?;

    println!("是否已经签到： {:#?}", resp);
    Ok(resp)
}

// 签到

async fn sign_in(client: Client, new_post: &Post<'_>) -> Result<(), RespError> {
    println!("new_post:{:#?}", new_post);
    let resp = client
        .post(BASE_URL.to_string() + SIGNINURL)
        .json(new_post)
        .send()
        .await?
        .text()
        .await?;

    println!("签到：{:?}", resp);
    Ok(())
}

// 查询矿石数量
async fn get_cur_point(client: Client) -> Result<GetCurPoint, RespError> {
    let resp = client
        .get(BASE_URL.to_string() + GETCURPOINT)
        .send()
        .await?
        .json::<GetCurPoint>()
        .await?;

    println!("当前矿石数： {:#?}", resp.data);
    Ok(resp)
}

// 是否抽奖
async fn is_draw(client: Client) -> Result<DrawResp, RespError> {
    let resp = client
        .get(BASE_URL.to_string() + ISDRAW)
        .send()
        .await?
        .json::<DrawResp>()
        .await?;
    println!("未抽奖次数还有：{:#?}", resp.data.free_count);
    Ok(resp)
}

// 抽奖
async fn draw(client: Client, new_post: &Post<'_>)-> Result<String, RespError> {
    let resp = client
        .post(BASE_URL.to_string() + DRAWURL)
        .json(new_post)
        .send()
        .await?
        .text()
        .await?;
    println!("抽奖：{:#?}", resp);
    Ok(resp)
}

// 十连抽
async fn ten_draw(client: Client, new_post: &Post<'_>) ->Result<String, RespError> {
    let resp = client
        .post(BASE_URL.to_string() + TEN_DRAWURL)
        .json(new_post)
        .send()
        .await?
        .text()
        .await?;
    println!("十连抽：{:#?}", resp);
    Ok(resp)
}