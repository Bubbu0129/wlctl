use clap::{Command, arg, crate_version, ArgAction};

pub fn cli() -> Command {
    Command::new("wlctl")
        .about("TUI for managing WiFi using NetworkManager")
        .version(crate_version!())
        // Root-level args (--mode) are for launching the TUI; they do not apply
        // when a subcommand like `doctor` is used.
        .args_conflicts_with_subcommands(true)
        .arg(
            arg!(--mode <mode>)
                .short('m')
                .required(false)
                .help("Device mode")
                .value_parser(["station", "ap"]),
        )
        .arg(
            arg!(--ascii)
                .required(false)
                .help("Use plain ASCII labels instead of nerd-font icons")
                .action(ArgAction::SetTrue),
        )
        .subcommand(
            Command::new("doctor")
                .about("Diagnose why your WiFi isn't working (rfkill, driver, DHCP, DNS, ...)"),
        )
}
