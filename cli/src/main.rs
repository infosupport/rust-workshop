
use simplelog::ColorChoice;
use simplelog::LevelFilter;
use simplelog::TermLogger;
use simplelog::TerminalMode;

/// This function initializes the logging facade and implementation for the application.
/// 
/// The logs will only show up on the terminal, and if the terminal supports it, the messages
/// will be coloured. Error messages will be printed to standard error, all other logging goes
/// to standard out.
fn prepare_logging(verbose: bool) {
    let level = if verbose { LevelFilter::Debug } else { LevelFilter::Info };
    let config = simplelog::Config::default();

    TermLogger::init(level, config, TerminalMode::Mixed, ColorChoice::Auto)
        .unwrap();
}

/// The main function of the application.
/// 
/// This is where the application starts.
pub fn main() {
    prepare_logging(false);

    log::info!("Hello, world!");
}