use reqwest;
use rand;
use tokio;

fn option_choice(options:Vec<&str>, weights:Vec<usize>) -> String{
    "".to_string()
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
        let mut req_body = 
       "entry.366340186".to_string() + "=" + &name[i] + "&" +
       "entry.723792600"+"=" + &phone_num[i] + "&" +
       "entry.696791717"+"=" + &option_choice(3,vec![])+ "&" +
       "entry.129356322"+"=" + &option_choice(3,vec![])+ "&" +
       "entry.709804345"+"=" + &option_choice(3,vec![])+ "&" +
       "entry.2059754675" + "=" + &option_choice(3,vec![])+ "&" +
       "entry.1915405966" + "=" + &option_choice(3,vec![])+ "&" +
       "entry.1450900952" + "=" + &option_choice(3,vec![])+ "&" +
       "entry.219204721"+ "=" +   &option_choice(3,vec![])+ "&" +
       "entry.1819819218" + "=" +&option_choice(3,vec![])+ "&" +
       "entry.1154860766" + "=" +&option_choice(3,vec![])+ "&" +
       "entry.733109057"+ "=" +   &option_choice(3,vec![])+ "&" +
       "entry.1184075448" + "=" + &option_choice(3,vec![])+ "&" +
       "entry.134730887"+ "=" + &option_choice(3,vec![])+ "&" +
       "entry.353176222"+ "=" + &option_choice(3,vec![])+ "&" +
       "entry.350962920"+ "=" + &option_choice(3,vec![])+ "&" +
       "entry.1855321311" + "=" + &option_choice(3,vec![])+ "&" +
       "entry.1554274227" + "=" + &option_choice(3,vec![])+ "&" +
       "entry.743124842"+ "=" + &option_choice(3,vec![])+ "&" +
       "entry.1738853646" + "=" + &option_choice(3,vec![])+ "&" +
       "entry.1473252665" + "=" + &option_choice(3,vec![])+ "&" +
       "entry.36805873"+ "=" + &option_choice(3,vec![])+ "&" +
       "entry.1752750168" + "=" + &option_choice(3,vec![])+ "&" +
       "entry.1625124840" + "=" + &option_choice(3,vec![])+ "&" +
       "entry.543595480"+ "=" + &option_choice(3,vec![])+ "&" +
       "entry.2013689512" + "=" + &option_choice(3,vec![])+ "&" +
       "entry.1634977044" + "=" + &option_choice(3,vec![])+ "&" +
       "entry.272649038" + "=" + &option_choice(3,vec![])+ "&" +
       "entry.1567615955" + "=" + &option_choice(3,vec![])+ "&" +
       "entry.2096425064" + "=" + &option_choice(3,vec![])+ "&" +
       "entry.1452918861" + "=" + &option_choice(3,vec![])+ "&" +
       "entry.1324790261" + "=" + &option_choice(3,vec![]);
        let res = client.post( "https://docs.google.com/forms/d/e/1FAIpQLScgHglUvvN_5aAkSnXJHzrb664pV7rpwaHcMoQo13_yZONCJA/formResponse").body(req_body)send().await;
        println!("{:?}", res);
    }
}
fn main(){
    tokio_main();
}