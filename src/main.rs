use anyhow::Result;
use clap::{Parser, Subcommand};
use restful_booker_automation::api::client::BookerClient;
use restful_booker_automation::builders::BookingBuilder;
use restful_booker_automation::reporting::{html_reporter::HtmlReporter, json_reporter::JsonReporter, metrics::TestSuite};
use restful_booker_automation::utils::config::Config;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Parser)]
#[command(name = "booker-cli", about = "Restful Booker automation CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Health check
    Ping,
    /// Create a sample booking
    CreateBooking,
    /// Generate sample report
    Report,
}

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let cli = Cli::parse();
    let config = Config::from_env()?;
    let client = BookerClient::new(config);

    match cli.command {
        Commands::Ping => {
            let ok = client.ping().await?;
            info!("Ping result: {}", ok);
            println!("Ping: {}", if ok { "OK" } else { "FAIL" });
        }
        Commands::CreateBooking => {
            let booking = BookingBuilder::new()
                .firstname("CLI")
                .lastname("User")
                .totalprice(150)
                .depositpaid(true)
                .checkin("2026-01-01")
                .checkout("2026-01-05")
                .additionalneeds("Breakfast")
                .build();
            let created = client.create_booking(&booking).await?;
            info!("Created booking ID: {}", created.bookingid);
            println!("Created booking: {:?}", created);
        }
        Commands::Report => {
            let mut suite = TestSuite::new("CLI Sample");
            suite.add_result("sample", true, "ok");
            JsonReporter::write(&suite, "target/test_report.json")?;
            HtmlReporter::write(&suite, "target/test_report.html")?;
            println!("Reports written to target/");
        }
    }

    Ok(())
}
