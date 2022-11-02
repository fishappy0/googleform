use rand::prelude::*;
use reqwest;
use std::{collections::HashMap, iter::zip};
use tokio;

fn option_choice(options: Vec<&str>, weights: Vec<usize>) -> String {
    let mut probabilities = vec![1; options.len()];
    let mut rng: StdRng = SeedableRng::from_entropy();
    for i in 0..weights.len() {
        probabilities[weights[i] - 1] = probabilities[weights[i] - 1] + rng.gen_range(1..2);
    }
    let option_with_prob = options.iter().zip(probabilities.iter()).collect::<Vec<_>>();
    // println!("{option_with_prob:?}");
    option_with_prob
        .choose_weighted(&mut rng, |item| item.1)
        .unwrap()
        .0
        .to_string()
}
#[tokio::main]
async fn tokio_main() {
    let client = reqwest::Client::new();
    // let form_entries = HashMap::from([
    //     ("Name",             "entry.366340186"),
    //     ("Phone",            "entry.723792600"),
    //     ("Gender",           "entry.696791717"),
    //     ("Preference",       "entry.129356322"),
    //     ("Age",              "entry.709804345"),
    //     ("City",             "entry.2059754675"),
    //     ("Employment",       "entry.1915405966"),
    //     ("Salary",           "entry.1450900952"),
    //     ("ReferredBy",       "entry.219204721"),
    //     ("YearUsed",         "entry.1819819218"),
    //     ("Frequent",         "entry.1154860766"),
    //     ("Satisfactory",     "entry.733109057"),
    //     ("Device",           "entry.1184075448"),
    //     ("UseCase",          "entry.134730887"),
    //     ("EaseOfUse",        "entry.353176222"),
    //     ("FavFeature",       "entry.350962920"),
    //     ("MoreInfo",         "entry.1855321311"),
    //     ("ProductDiscovery", "entry.1554274227"),
    //     ("ProductsPerOrder", "entry.743124842"),
    //     ("BuyTogether",      "entry.1738853646"),
    //     ("Food",             "entry.1473252665"),
    //     ("Finding",          "entry.36805873"),
    //     ("MostUsed",         "entry.1752750168"),
    //     ("Tracking",         "entry.1625124840"),
    //     ("Deal",             "entry.543595480"),
    //     ("AppSpeed",         "entry.2013689512"),
    //     ("SupportSpeed",     "entry.1634977044"),
    //     ("UsedMall",         "entry.272649038"),
    //     ("MallOpinion",      "entry.1567615955"),
    //     ("EventRating",      "entry.2096425064"),
    //     ("StarRating",       "entry.1452918861"),
    //     ("Recommend",        "entry.1324790261"),
    // ]);
    let name = vec![
        "Khưu Phong Châu",
        "Bình Thuỵ Du",
        "Tấn Khánh Hải",
        "Phù Xuân Hiếu",
        "Liên Hoàng Linh",
        "Thang Nam Phương",
        "Bùi Lê Duy",
        "Bùi Thanh Hải",
        "Nguyễn An",
        "Lâm Xuân Minh",
        "Đống Đức Quyền",
        "Nguyễn Thị Hồng",
        "Cung Thanh Toàn",
        "Lê Thanh Trung",
        "Nguyễn Thanh Trung",
        "Nguyễn Thị Thanh Trung",
        "Lê Xuân Trung",
        "Lê Văn Nam",
    ];
    let phone_num = vec![
        "0346074199",
        "0384179997",
        "0866133553",
        "0353301768",
        "0336130507",
        "0345243268",
        "0365745186",
        "0345757288",
        "0977297847",
        "0338052566",
        "0399088239",
        "0374465179",
        "0337770866",
        "0972532053",
        "0971562442",
        "0388632112",
        "0389136788",
        "0394171282",
    ];
    for i in 0..name.len() {
        let form = reqwest::multipart::Form::new()
            .text("entry.366340186", name[i])
            .text("entry.723792600", phone_num[i])
            .text("entry.696791717", option_choice(vec!["Nam", "Nữ"], vec![]))
            .text(
                "entry.129356322",
                option_choice(
                    vec![
                        "Thư thoái, thư giản",
                        "Hồi hộp, chóng mua hàng",
                        "Chậm rãi, tham khảo thông tin",
                        "Choáng ngộp, chọn theo số đông",
                    ],
                    vec![],
                ),
            )
            .text(
                "entry.709804345",
                option_choice(vec!["15-25", "26-35"], vec![1]),
            )
            .text(
                "entry.2059754675",
                option_choice(
                    vec![
                        "Hà Nội",
                        "TP.Hồ Chí Minh",
                        "Đà Nẵng",
                        "Tỉnh khác",
                        "Việt Kiều",
                    ],
                    vec![2],
                ),
            )
            .text(
                "entry.1915405966",
                option_choice(vec!["Sinh viên", "Vừa học vừa làm", "Đã đi làm"], vec![1]),
            )
            .text(
                "entry.1450900952",
                option_choice(
                    vec![
                        "Không có thu nhập",
                        "1.000.000 - 5.000.000",
                        "5.000.000 - 10.000.000",
                        "10.000.000 - 15.000.000",
                        "15.000.000 - 20.000.000",
                        "20.000.000 - 30.000.000",
                        "Trên 30.000.000",
                    ],
                    vec![],
                ),
            )
            .text(
                "entry.219204721",
                option_choice(
                    vec![
                        "Bạn bè",
                        "Quảng cáo (Trên mạng, TV, tạp chí...)",
                        "Truyền thông",
                        "Người thân",
                    ],
                    vec![],
                ),
            )
            .text(
                "entry.1819819218",
                option_choice(
                    vec!["Không sử dụng", "Dưới 1 năm", "1 - 3 năm", "Trên 3 năm"],
                    vec![],
                ),
            )
            .text(
                "entry.1154860766",
                option_choice(vec!["Có", "Có nhưng ít", "Không sử dụng"], vec![]),
            )
            .text(
                "entry.733109057",
                option_choice(vec!["1", "2", "3", "4", "5"], vec![]),
            )
            .text(
                "entry.1184075448",
                option_choice(
                    vec![
                        "Sử dụng điện thoại cá nhân",
                        "Sử dụng máy tính cá nhân",
                        "Nhờ người thân đặt hàng giùm",
                    ],
                    vec![1, 2],
                ),
            )
            .text(
                "entry.134730887",
                option_choice(
                    vec!["Mua tiêu dùng", "Tặng quà", "Tham khảo giá", "Đặt đồ ăn"],
                    vec![],
                ),
            )
            .text(
                "entry.353176222",
                option_choice(vec!["1", "2", "3", "4", "5"], vec![]),
            )
            .text(
                "entry.350962920",
                option_choice(
                    vec![
                        "Giảm giá (\"deal\")",
                        "Đặt đồ ăn",
                        "Gợi ý sản phẩm",
                        "Chat với shop",
                        "Chọn hình thức thanh toán",
                        "Tìm kiếm",
                        "Đặt hàng",
                        "Shopee mall",
                        "Shopee Live",
                        "Bảo hiểm",
                        "Kiểm tra tình trạng đơn hàng",
                    ],
                    vec![],
                ),
            )
            .text(
                "entry.1855321311",
                option_choice(
                    vec![
                        "Mục chú thích",
                        "Trò chuyện với cửa hàng",
                        "\"Review chất\" ở trên mạng",
                        "Coi các shop khác",
                    ],
                    vec![],
                ),
            )
            .text(
                "entry.1554274227",
                option_choice(
                    vec![
                        "Qua bạn bè",
                        "Coi reviewer trên mạng xã hội",
                        "Coi stream bán hàng",
                        "Lướt \"Dạo\" các trang bán hàng",
                        "Mục giới thiệu",
                    ],
                    vec![],
                ),
            )
            .text(
                "entry.743124842",
                option_choice(vec!["1-5", "5-10", "trên 10 sản phẩm"], vec![]),
            )
            .text(
                "entry.1738853646",
                option_choice(vec!["Có", "Không"], vec![]),
            )
            .text(
                "entry.1473252665",
                option_choice(
                    vec![
                        "Ra tiệm",
                        "Nấu ăn",
                        "Đặt món qua ứng dụng",
                        "Đặt món qua gọi điện cho shop",
                        "Mua đem về",
                    ],
                    vec![],
                ),
            )
            .text(
                "entry.36805873",
                option_choice(
                    vec![
                        "Thanh tìm kiếm trong ứng dụng",
                        "Dùng công cụ tìm kiếm như google",
                    ],
                    vec![],
                ),
            )
            .text(
                "entry.1752750168",
                option_choice(
                    vec![
                        "Giảm giá (\"deal\")",
                        "Đặt đồ ăn",
                        "Gợi ý sản phẩm",
                        "Chat với shop",
                        "Chọn hình thức thanh toán",
                        "Tìm kiếm",
                        "Đặt hàng",
                        "Shopee mall",
                        "Shopee Live",
                        "Bảo hiểm",
                        "Kiểm tra tình trạng đơn hàng",
                    ],
                    vec![],
                ),
            )
            .text(
                "entry.1625124840",
                option_choice(vec!["Có", "Không"], vec![]),
            )
            .text(
                "entry.543595480",
                option_choice(vec!["Có", "Không"], vec![]),
            )
            .text(
                "entry.2013689512",
                option_choice(vec!["Có", "Không"], vec![]),
            )
            .text(
                "entry.1634977044",
                option_choice(vec!["Chậm", "Bình thường", "Nhanh"], vec![]),
            )
            .text(
                "entry.272649038",
                option_choice(vec!["Có", "Không"], vec![]),
            )
            .text(
                "entry.1567615955",
                option_choice(
                    vec!["Tốt", "Đắt hơn thường", "Quy trình bảo hành rắc rối"],
                    vec![1],
                ),
            )
            .text(
                "entry.2096425064",
                option_choice(vec!["1", "2", "3", "4", "5"], vec![]),
            )
            .text(
                "entry.1452918861",
                option_choice(vec!["1", "2", "3", "4", "5"], vec![]),
            )
            .text(
                "entry.1324790261",
                option_choice(vec!["Có", "Không"], vec![]),
            );
        let res = client.post( "https://docs.google.com/forms/d/e/1FAIpQLScgHglUvvN_5aAkSnXJHzrb664pV7rpwaHcMoQo13_yZONCJA/formResponse").multipart(form).send().await;
        println!("{:?}", res);
    }
}
fn main() {
    tokio_main();
}
