use clap::Parser;
use config::Config;
use config::File;
use config::FileFormat;
use simplelog::ColorChoice;
use simplelog::LevelFilter;
use simplelog::TermLogger;
use simplelog::TerminalMode;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short = 'v', long = "verbose", help = "Enables verbose mode")]
    verbose: bool,
}

/// This function initializes the logging facade and implementation for the application.
///
/// The logs will only show up on the terminal, and if the terminal supports it, the messages
/// will be coloured. Error messages will be printed to standard error, all other logging goes
/// to standard out.
fn prepare_logging(verbose: bool) {
    let level = if verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    let config = simplelog::Config::default();

    TermLogger::init(level, config, TerminalMode::Mixed, ColorChoice::Auto).unwrap();
}

/// The main function of the application.
///
/// This is where the application starts.
pub fn main() {
    let cli = Cli::parse();

    let verbose = cli.verbose;

    prepare_logging(verbose);

    let source = File::with_name("task.ini").format(FileFormat::Ini);
    let config = Config::builder()
        .add_source(source)
        .build()
        .unwrap();

    let api_key = match config.get_string("apikey") {
        Ok(api_key) => api_key,
        Err(error) => {
            log::error!("Can't find API key: {}", error.to_string());
            std::process::exit(1);
        }
    };

    log::info!("Found API key: {}", api_key);
    log::debug!("Test");
    log::info!("Hello, world!");
}
