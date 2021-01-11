mod eeschema;
mod gui;
mod pcbnew;
mod xvfb;

use std::path;
use structopt::StructOpt;

const XVFB_PORT: u8 = 99;

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

fn run_erc(args: ErcOptions) -> Result<(), String> {
    let xvfb_process = if args.headless {
        Some(xvfb::Xvfb::run(XVFB_PORT)?)
    } else {
        None
    };
    let mut eeschema_process = eeschema::Eeschema::run(&args.path_to_sch)?;
    let erc_output = gui::erc::get_erc_output_from_gui().map_err(|e| {
        println!("Erred");
        xvfb_process.map(|mut xvfb_process| {
            println!("Captured stderr from xvfb:\n{}", xvfb_process.dump_stderr());
        });
        println!(
            "Captured stderr from eeschema:\n{}",
            eeschema_process.dump_stderr()
        );
        println!(
            "Captured stdout from eeschema:\n{}",
            eeschema_process.dump_stdout()
        );
        e
    })?;
    println!("{}", erc_output);
    Ok(())
}

fn run_drc(args: DrcOptions) -> Result<(), String> {
    let xvfb_process = if args.headless {
        Some(xvfb::Xvfb::run(XVFB_PORT)?)
    } else {
        None
    };
    let mut pcbnew_process = pcbnew::Pcbnew::run(&args.path_to_kicad_pcb)?;
    let drc_output = gui::drc::get_drc_output_from_gui().map_err(move |e| {
        xvfb_process.map(|mut xvfb_process| {
            println!("Captured stderr from xvfb:\n{}", xvfb_process.dump_stderr());
        });
        println!(
            "Captured stderr from pcbnew:\n{}",
            pcbnew_process.dump_stderr()
        );
        println!(
            "Captured stdout from pcbnew:\n{}",
            pcbnew_process.dump_stdout()
        );

        drop(pcbnew_process);
        e
    })?;
    println!("{}", drc_output);
    Ok(())
}

fn main() -> Result<(), String> {
    match Options::from_args() {
        Options::RunErc(args) => run_erc(args),
        Options::RunDrc(args) => run_drc(args),
    }
}
