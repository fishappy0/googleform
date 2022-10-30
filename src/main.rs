use thirtyfour::prelude::*;
use tokio;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    driver.goto("https://www.google.com").await?;
    let elem = driver.find(By::Name("q")).await?;
    elem.send_keys("Hello world").await?;

    let elem_button = elem.find(By::Css("input[type='submit']")).await?;
    elem_button.click().await?;

    driver.quit().await?;
    Ok(())
}
