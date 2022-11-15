use std::process::Output;

use rand::prelude::*;
use reqwest;
use tokio;

// Takes in a vector of options and list contains the options position to amplify the probability
// and Returns the chosen option
// THis uses the .choose weighted method of rand crate
fn option_choice(options: Vec<&str>, weights: Vec<usize>) -> String {
    let mut probabilities = vec![1; options.len()];
    let mut rng: StdRng = SeedableRng::from_entropy();
    for i in 0..weights.len() {
        probabilities[weights[i] - 1] = probabilities[weights[i] - 1] + rng.gen_range(1..2);
    }
    let option_with_prob = options.iter().zip(probabilities.iter()).collect::<Vec<_>>();
    option_with_prob
        .choose_weighted(&mut rng, |item| item.1)
        .unwrap()
        .0
        .to_string()
}
// #[tokio::main]
// async fn tokio_meenn() {
//     let client = reqwest::Client::new();
//     // let form_entries = HashMap::from([
//     //     ("Name",             "entry.366340186"),
//     //     ("Phone",            "entry.723792600"),
//     //     ("Gender",           "entry.696791717"),
//     //     ("Preference",       "entry.129356322"),
//     //     ("Age",              "entry.709804345"),
//     //     ("City",             "entry.2059754675"),
//     //     ("Employment",       "entry.1915405966"),
//     //     ("Salary",           "entry.1450900952"),
//     //     ("ReferredBy",       "entry.219204721"),
//     //     ("YearUsed",         "entry.1819819218"),
//     //     ("Frequent",         "entry.1154860766"),
//     //     ("Satisfactory",     "entry.733109057"),
//     //     ("Device",           "entry.1184075448"),
//     //     ("UseCase",          "entry.134730887"),
//     //     ("EaseOfUse",        "entry.353176222"),
//     //     ("FavFeature",       "entry.350962920"),
//     //     ("MoreInfo",         "entry.1855321311"),
//     //     ("ProductDiscovery", "entry.1554274227"),
//     //     ("ProductsPerOrder", "entry.743124842"),
//     //     ("BuyTogether",      "entry.1738853646"),
//     //     ("Food",             "entry.1473252665"),
//     //     ("Finding",          "entry.36805873"),
//     //     ("MostUsed",         "entry.1752750168"),
//     //     ("Tracking",         "entry.1625124840"),
//     //     ("Deal",             "entry.543595480"),
//     //     ("AppSpeed",         "entry.2013689512"),
//     //     ("SupportSpeed",     "entry.1634977044"),
//     //     ("UsedMall",         "entry.272649038"),
//     //     ("MallOpinion",      "entry.1567615955"),
//     //     ("EventRating",      "entry.2096425064"),
//     //     ("StarRating",       "entry.1452918861"),
//     //     ("Recommend",        "entry.1324790261"),
//     // ]);
//     let name = vec![
//         "Khưu Phong Châu",
//         "Bình Thuỵ Du",
//         "Tấn Khánh Hải",
//         "Phù Xuân Hiếu",
//         "Liên Hoàng Linh",
//         "Thang Nam Phương",
//         "Bùi Lê Duy",
//         "Bùi Thanh Hải",
//         "Nguyễn An",
//         "Lâm Xuân Minh",
//         "Đống Đức Quyền",
//         "Nguyễn Thị Hồng",
//         "Cung Thanh Toàn",
//         "Lê Thanh Trung",
//         "Nguyễn Thanh Trung",
//         "Nguyễn Thị Thanh Trung",
//         "Lê Xuân Trung",
//         "Lê Văn Nam",
//     ];
//     let phone_num = vec![
//         "0346074199",
//         "0384179997",
//         "0866133553",
//         "0353301768",
//         "0336130507",
//         "0345243268",
//         "0365745186",
//         "0345757288",
//         "0977297847",
//         "0338052566",
//         "0399088239",
//         "0374465179",
//         "0337770866",
//         "0972532053",
//         "0971562442",
//         "0388632112",
//         "0389136788",
//         "0394171282",
//     ];
//     for i in 0..name.len() {
//         let form = reqwest::multipart::Form::new()
//             .text("entry.366340186", name[i])
//             .text("entry.723792600", phone_num[i])
//             .text("entry.696791717", option_choice(vec!["Nam", "Nữ"], vec![]))
//             .text(
//                 "entry.129356322",
//                 option_choice(
//                     vec![
//                         "Thư thoái, thư giản",
//                         "Hồi hộp, chóng mua hàng",
//                         "Chậm rãi, tham khảo thông tin",
//                         "Choáng ngộp, chọn theo số đông",
//                     ],
//                     vec![],
//                 ),
//             )
//             .text(
//                 "entry.709804345",
//                 option_choice(vec!["15-25", "26-35"], vec![1]),
//             )
//             .text(
//                 "entry.2059754675",
//                 option_choice(
//                     vec![
//                         "Hà Nội",
//                         "TP.Hồ Chí Minh",
//                         "Đà Nẵng",
//                         "Tỉnh khác",
//                         "Việt Kiều",
//                     ],
//                     vec![2],
//                 ),
//             )
//             .text(
//                 "entry.1915405966",
//                 option_choice(vec!["Sinh viên", "Vừa học vừa làm", "Đã đi làm"], vec![1]),
//             )
//             .text(
//                 "entry.1450900952",
//                 option_choice(
//                     vec![
//                         "Không có thu nhập",
//                         "1.000.000 - 5.000.000",
//                         "5.000.000 - 10.000.000",
//                         "10.000.000 - 15.000.000",
//                         "15.000.000 - 20.000.000",
//                         "20.000.000 - 30.000.000",
//                         "Trên 30.000.000",
//                     ],
//                     vec![2, 3],
//                 ),
//             )
//             .text(
//                 "entry.219204721",
//                 option_choice(
//                     vec![
//                         "Bạn bè",
//                         "Quảng cáo (Trên mạng, TV, tạp chí...)",
//                         "Truyền thông",
//                         "Người thân",
//                     ],
//                     vec![1],
//                 ),
//             )
//             .text(
//                 "entry.1819819218",
//                 option_choice(
//                     vec!["Không sử dụng", "Dưới 1 năm", "1 - 3 năm", "Trên 3 năm"],
//                     vec![3],
//                 ),
//             )
//             .text(
//                 "entry.1154860766",
//                 option_choice(vec!["Có", "Có nhưng ít", "Không sử dụng"], vec![1]),
//             )
//             .text(
//                 "entry.733109057",
//                 option_choice(vec!["1", "2", "3", "4", "5"], vec![4]),
//             )
//             .text(
//                 "entry.1184075448",
//                 option_choice(
//                     vec![
//                         "Sử dụng điện thoại cá nhân",
//                         "Sử dụng máy tính cá nhân",
//                         "Nhờ người thân đặt hàng giùm",
//                     ],
//                     vec![1, 2],
//                 ),
//             )
//             .text(
//                 "entry.134730887",
//                 option_choice(
//                     vec!["Mua tiêu dùng", "Tặng quà", "Tham khảo giá", "Đặt đồ ăn"],
//                     vec![],
//                 ),
//             )
//             .text(
//                 "entry.353176222",
//                 option_choice(vec!["1", "2", "3", "4", "5"], vec![4]),
//             )
//             .text(
//                 "entry.350962920",
//                 option_choice(
//                     vec![
//                         "Giảm giá (\"deal\")",
//                         "Đặt đồ ăn",
//                         "Gợi ý sản phẩm",
//                         "Chat với shop",
//                         "Chọn hình thức thanh toán",
//                         "Tìm kiếm",
//                         "Đặt hàng",
//                         "Bảo hiểm",
//                         "Kiểm tra tình trạng đơn hàng",
//                     ],
//                     vec![1, 3],
//                 ),
//             )
//             .text(
//                 "entry.1855321311",
//                 option_choice(
//                     vec![
//                         "Mục chú thích",
//                         "Trò chuyện với cửa hàng",
//                         "\"Review chất\" ở trên mạng",
//                         "Coi các shop khác",
//                     ],
//                     vec![],
//                 ),
//             )
//             .text(
//                 "entry.1554274227",
//                 option_choice(
//                     vec![
//                         "Qua bạn bè",
//                         "Coi reviewer trên mạng xã hội",
//                         "Coi stream bán hàng",
//                         "Lướt \"Dạo\" các trang bán hàng",
//                         "Mục giới thiệu",
//                     ],
//                     vec![],
//                 ),
//             )
//             .text(
//                 "entry.743124842",
//                 option_choice(vec!["1-5", "5-10", "trên 10 sản phẩm"], vec![]),
//             )
//             .text(
//                 "entry.1738853646",
//                 option_choice(vec!["Có", "Không"], vec![]),
//             )
//             .text(
//                 "entry.1473252665",
//                 option_choice(
//                     vec![
//                         "Ra tiệm",
//                         "Nấu ăn",
//                         "Đặt món qua ứng dụng",
//                         "Đặt món qua gọi điện cho shop",
//                         "Mua đem về",
//                     ],
//                     vec![3],
//                 ),
//             )
//             .text(
//                 "entry.36805873",
//                 option_choice(
//                     vec![
//                         "Thanh tìm kiếm trong ứng dụng",
//                         "Dùng công cụ tìm kiếm như google",
//                     ],
//                     vec![],
//                 ),
//             )
//             .text(
//                 "entry.1752750168",
//                 option_choice(
//                     vec![
//                         "Giảm giá (\"deal\")",
//                         "Đặt đồ ăn",
//                         "Gợi ý sản phẩm",
//                         "Chat với shop",
//                         "Chọn hình thức thanh toán",
//                         "Tìm kiếm",
//                         "Đặt hàng",
//                         "Bảo hiểm",
//                         "Kiểm tra tình trạng đơn hàng",
//                     ],
//                     vec![3, 1],
//                 ),
//             )
//             .text(
//                 "entry.1625124840",
//                 option_choice(vec!["Có", "Không"], vec![]),
//             )
//             .text(
//                 "entry.543595480",
//                 option_choice(vec!["Có", "Không"], vec![]),
//             )
//             .text(
//                 "entry.2013689512",
//                 option_choice(vec!["Có", "Không"], vec![]),
//             )
//             .text(
//                 "entry.1634977044",
//                 option_choice(vec!["Chậm", "Bình thường", "Nhanh"], vec![]),
//             )
//             .text(
//                 "entry.1567615955",
//                 option_choice(
//                     vec!["Tốt", "Đắt hơn thường", "Quy trình bảo hành rắc rối"],
//                     vec![1],
//                 ),
//             )
//             .text(
//                 "entry.2096425064",
//                 option_choice(vec!["1", "2", "3", "4", "5"], vec![5]),
//             )
//             .text(
//                 "entry.1452918861",
//                 option_choice(vec!["1", "2", "3", "4", "5"], vec![5]),
//             )
//             .text(
//                 "entry.1324790261",
//                 option_choice(vec!["Có", "Không"], vec![1]),
//             );
//         let res = client.post( "https://docs.google.com/forms/d/e/1FAIpQLScgHglUvvN_5aAkSnXJHzrb664pV7rpwaHcMoQo13_yZONCJA/formResponse").multipart(form).send().await;
//         println!("{:?}", res);
//     }
// }
#[tokio::main]
async fn tokio_main() {
    let client = reqwest::Client::new();
    for _ in std::iter::repeat(0).take(100) {
        let form = reqwest::multipart::Form::new()
                .text("entry.219838182",option_choice(vec!["18-30 years","31-40 years","41 years and above"], vec![1,2]))
                .text("entry.782791643",option_choice(vec!["Male","Female"], vec![]))
                .text("entry.2046010680",option_choice(vec!["31 to 44 hours","45 to 64 hours","More than 65 hours"],vec![2]))
                .text("entry.1827942647",option_choice(vec!["Stay the same","increase moderately","increase substantially"],vec![2]))
                .text("entry.1041128894",option_choice(vec!["10 To 20%","More Than 20%"], vec![2]))
                .text("entry.674249568",option_choice(vec!["Just you","Yourself and others"], vec![1]))
                .text("entry.1451687898",option_choice(vec!["Yes","Never thought about it"], vec![1]))
                .text("entry.666966072",option_choice(vec!["Yes I have a plan","I have only partially completed my plan"], vec![]))
                .text("entry.166318111","Good")
                .text("entry.1324874226","Life")
                .text("entry.1324874226","Automotive")
                .text("entry.1324874226","Homeowner's/Renter's")
                .text("entry.1324874226",option_choice(vec!["Long Term Care",""],vec![]))
                .text("entry.1230939168 ",option_choice(vec!["Spend far less than your income", "Spend the same amount as your income"], vec![1]))
                .text("entry.455822227",option_choice(vec!["3 months or less","6 months or more"], vec![2]))
                .text("entry.718204594",option_choice(vec!["Pay most bills and credit card payments on time and in full","Pay all bills and credit card payments on time and in full"], vec![2]))
                .text("entry.873247914",option_choice(vec!["Do not have any debt","appropriate level of debt"], vec![1]))
                .text("entry.1573216417",option_choice(vec!["0% - 25%","25% - 50%"], vec![1]))
                .text("entry.829445361",option_choice(vec!["I considered several products/loans/policies/accounts] from different companies before making my decision", "I didn’t consider any other at all"], vec![2]))
                .text("entry.1884613196",option_choice(vec!["Advice of friends/relatives (working/ not working in the financial services industry)", "Product specific information found on the internet"], vec![1]))
                .text("entry.851684716",option_choice(vec!["Equitable","Good","Excellent"], vec![2, 3]))
                .text("entry.128375930",option_choice(vec!["Yes, I take active steps","No, I don't take active steps","I do not need to improve my score"],vec![3]))

                .text("entry.1698734016",option_choice(vec!["Yes","No"], vec![1]))
                .text("entry.1116245185",option_choice(vec!["yes","No","I don't know enough about them"], vec![2]))
                .text("entry.836356083",option_choice(vec!["Very negative","Negative","Neutral","Positive"], vec![1]))
                .text("entry.1636415603",option_choice(vec!["1-2 years","3-4 years"],vec![]))

                .text("entry.1560377054",option_choice(vec!["Mostly opportunities","Mostly a risk"],vec![2]))
                .text("entry.1198945916",option_choice(vec!["Disagree","Agree","Strongly agree"],vec![1]))
                .text("entry.695582847",option_choice(vec!["Somewhat stable","Stable","Very stable"], vec![2,3]))
                .text("entry.1235423902",option_choice(vec!["Very possible", "Somewhat possible"],vec![1]))
                .text("entry.852004970",option_choice(vec!["Savings","Financial services that are formalized","realized income through sale of assets"],vec![1]))
                .text("entry.379416842",option_choice(vec!["Agree","Strongly agree","Disagree"],vec![2]))
                .text("entry.591289162",option_choice(vec!["Public Provident Fund/Bank Fixed Deposit","Gold/Silver and other precious metals","Stocks/Bonds/Mutual funds/hedge funds and other non-banking financial companies"],vec![3]))

                .text("entry.1703915594",option_choice(vec!["Very confident","confident"],vec![1]))
                .text("entry.156074937",option_choice(vec!["Yes","No"],vec![2]))
                .text("entry.1700848937",option_choice(vec!["Excellent","Good"],vec![1]))
                .text("entry.962290136",option_choice(vec!["Excellent","Good"],vec![1]))

                .text("entry.536319637",option_choice(vec!["Yes","No"],vec![1]))
                .text("entry.1681038144",option_choice(vec!["Returns","Low Risk Factor","Credit Rating"],vec![2]))
                .text("entry.1429100967",option_choice(vec!["Yes","No"],vec![2]))
                .text("entry.1867028321","Yes")

                .text("entry.986069950",option_choice(vec!["Equity fund","Hybrid fund (equity & Debt combination)","Debt fund","ELSS Funds","Money Market Funds"],vec![1]))
                .text("entry.1791089700",option_choice(vec!["prepared to take chances after sufficient research","Cautious","Risk averse"],vec![1]))
                .text("entry.1207777001","No")
                .text("entry.1422367147","less that today")
                .text("entry.1143557710",option_choice(vec!["50,000$","60,000$"],vec![1]))
                .text("entry.810529932",option_choice(vec!["It is critical for me to understand how much money my investment will bring me in the future.","I value a low but secure return on my investments."],vec![1]))
                .text("entry.680009560",option_choice(vec!["Invest it in safe high quality bonds or bond mutual funds","Deposit it in a bank account, money market account or insured cd"],vec![1]))

                .text("entry.2049316969",option_choice(vec!["very comfortable with investing my funds and I know how a lot about the product","somewhat comfortable but I don't know how to place my funds accurately on my own"],vec![1]))
                .text("entry.355875592",option_choice(vec!["Less than a year","1-5 years"], vec![]))
                .text("entry.1603979682",option_choice(vec!["Approach financial service providers/professionals in the field","seek a friends advice."],vec![1]))
                .text("entry.1819554122",option_choice(vec!["strongly agree","Agree"],vec![1]))
                .text("entry.857354501 ",option_choice(vec!["I wouldn't stress over losses throughout that time period.","If my loss was larger than 10%, I would be worried."],vec![2]))
                .text("entry.1724631970",option_choice(vec!["I'm ready to take the hit from a setback in order to increase my profits.","I am more interested in entirely avoiding losses."],vec![2]))
                .text("entry.24738987",option_choice(vec!["Building up a balance of money in your bank account","Giving money to family to save on your behalf","Saving in an informal savings club"],vec![2]))
                .text("entry.492869581",option_choice(vec!["Buying a house or large consumer goods (ex. boat, car, rental property)","Paying for my education/my childrens' educations","Starting or investing in a business"],vec![3]))
                .text("entry.276704930",option_choice(vec!["Agree","Strongly agree"],vec![]));
        let res = client.post( "https://docs.google.com/forms/d/e/1FAIpQLSfsp96X_HETCBC-UePQU9Caap8gbBuuH6o56xjG7PnsV-sSpg/formResponse").multipart(form).send().await;
        println!("{:?}", res);
    }
}
fn main() {
    tokio_main();
}
