mod eeschema;
mod gui;
mod pcbnew;
mod xvfb;

use std::path;
use std::time::Duration;

use log::error;
use pretty_env_logger;
use regex;
use structopt::StructOpt;

const XVFB_PORT: u8 = 99;

pub struct Timeouts {
    window_launch: Duration,
    popup_launch: Duration,
    execution: Duration,
}


#[derive(StructOpt)]
#[structopt(
    name = "run-erc",
    about = "Run Kicad's Electric Rule Checker by spawning the Kicad eeschema and navigating through the UI"
)]
struct ErcOptions {
    #[structopt(parse(from_os_str))]
    path_to_sch: path::PathBuf,
    #[structopt(
        long,
        about = "Run headless by spawning a xvfb process. Xvfb must already by installed on the system."
    )]
    headless: bool,
    #[structopt(
        long,
        about = "How many seconds should we wait for Eeschema to launch before giving up?",
        default_value = "5",
    )]
    eeschema_launch_timeout_in_s: u64,
    #[structopt(
        long,
        about = "How many seconds should we wait before assuming a popup window had time to get launched?",
        default_value = "1",
    )]
    popup_timeout_in_s: u64,
    #[structopt(
        long,
        about = "How many seconds should we wait for the Electrical Rules Checker to produce an output?",
        default_value = "5",
    )]
    erc_timeout_in_s: u64
}

impl ErcOptions {
    fn get_timeouts(&self) -> Timeouts {
        Timeouts {
            window_launch: Duration::from_secs(self.eeschema_launch_timeout_in_s),
            popup_launch: Duration::from_secs(self.popup_timeout_in_s),
            execution: Duration::from_secs(self.erc_timeout_in_s)
        }
    }
}

#[derive(StructOpt)]
#[structopt(
    name = "run-drc",
    about = "Run Kicad's Design Rule Checker by spawning pcbnew and navigating through the UI"
)]
struct DrcOptions {
    #[structopt(parse(from_os_str))]
    path_to_kicad_pcb: path::PathBuf,
    #[structopt(
        long,
        about = "Run headless by spawning a xvfb process. Xvfb must already by installed on the system."
    )]
    headless: bool,
    #[structopt(
        long,
        about = "How many seconds should we wait for Pcbnew to launch before giving up?",
        default_value = "10",
    )]
    pcbnew_launch_timeout_in_s: u64,
    #[structopt(
        long,
        about = "How many seconds should we wait before assuming a popup window had time to get launched?",
        default_value = "1",
    )]
    popup_timeout_in_s: u64,
    #[structopt(
        long,
        about = "How many seconds should we wait for a certain operation (DRC) to execute?",
        default_value = "20",
    )]
    drc_timeout_in_s: u64
}

impl DrcOptions {
    fn get_timeouts(&self) -> Timeouts {
        Timeouts {
            window_launch: Duration::from_secs(self.pcbnew_launch_timeout_in_s),
            popup_launch: Duration::from_secs(self.popup_timeout_in_s),
            execution: Duration::from_secs(self.drc_timeout_in_s)
        }
    }
}

#[derive(StructOpt)]
#[structopt(
    name = "kicad_cli",
    about = "Perform useful tasks with kicad from the command line"
)]
enum Options {
    RunErc(ErcOptions),
    RunDrc(DrcOptions),
}

#[derive(Debug)]
struct ErcOutput {
    num_errors: usize,
    num_warnings: usize,
}

impl ErcOutput {
    fn try_from_eeschema_output(s: &str) -> Result<Self, String> {
        let last_line = s.split_terminator('\n')
                         .find(|line| line.starts_with(" ** ERC messages")).unwrap();
        let re = regex::Regex::new(r" \*\* ERC messages:\s+(\d+)\s+Errors\s+(\d)+\s+Warnings\s+(\d+)").expect("Invalid regex");
        for cap in re.captures_iter(last_line) {
            let total: usize = cap[1].parse().unwrap();
            let num_errors: usize = cap[2].parse().unwrap();
            let num_warnings: usize = cap[3].parse().unwrap();
            if total != num_errors + num_warnings{
                return Err(format!("Expected total ERC messages to be the sum of Errors and Warnings. Not the case in: {}", last_line));
            }
            return Ok(Self {num_errors, num_warnings})
        }
        panic!("Failed to parse eeschema output");
    }
}

fn run_erc(args: ErcOptions) -> Result<(), String> {
    let _xvfb_process = if args.headless {
        Some(xvfb::Xvfb::run(XVFB_PORT)?)
    } else {
        None
    };
    let _eeschema_process = eeschema::Eeschema::run(&args.path_to_sch)?;
    let erc_output = ErcOutput::try_from_eeschema_output(&gui::erc::get_erc_output_from_gui(args.get_timeouts()).map_err(|e| {
        error!("Failed to obtain erc output");
        e
    })?)?;
    
    println!("{:?}", erc_output);
    Ok(())
}

#[derive(Debug)]
struct DrcOutput {
    num_errors: usize,
    num_unconnected_pads: usize,
}

impl DrcOutput {
    fn try_from_pcbnew_output(s: &str) -> Result<Self, String> {
        let num_errors = s.split_terminator('\n')
                          .find_map(|line| {
                              if line.ends_with("DRC errors **") {
                                Some(line.replace("** Found ", "").replace(" DRC errors **", "").parse().unwrap())
                              } else {
                                None
                              }
                          }).unwrap();
        let num_unconnected_pads = s.split_terminator('\n')
                          .find_map(|line| {
                              if line.ends_with("unconnected pads **") {
                                Some(line.replace("** Found ", "").replace(" unconnected pads **", "").parse().unwrap())
                              } else {
                                None
                              }
                          }).unwrap();
        Ok(Self {num_errors, num_unconnected_pads})
    }
}
fn run_drc(args: DrcOptions) -> Result<(), String> {
    let _xvfb_process = if args.headless {
        Some(xvfb::Xvfb::run(XVFB_PORT)?)
    } else {
        None
    };
    let _pcbnew_process = pcbnew::Pcbnew::run(&args.path_to_kicad_pcb)?;
    let drc_output = DrcOutput::try_from_pcbnew_output(&gui::drc::get_drc_output_from_gui(args.get_timeouts()).map_err(move |e| {
        error!("Failed to obtain drc output");
        e
    })?)?;
    println!("{:?}", drc_output);
    Ok(())
}

fn main() -> Result<(), String> {
    pretty_env_logger::init();
    match Options::from_args() {
        Options::RunErc(args) => run_erc(args),
        Options::RunDrc(args) => run_drc(args),
    }
}
