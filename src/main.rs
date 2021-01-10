mod eeschema;
mod gui;
mod xvfb;

use std::path;
use structopt::StructOpt;

const XVFB_PORT: u8 = 99;

#[derive(StructOpt)]
#[structopt(
    name = "run-erc",
    about = "Run Kicad's Electric Rule Checker by spawning the Kicad gui"
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
    name = "kicad_gui_automation",
    about = "Perform useful tasks with kicad from the command line"
)]
enum Options {
    RunErc(ErcOptions),
    RunDrc,
}

fn run_erc(args: ErcOptions) -> Result<(), String> {
    let xvfb_process = if args.headless {
        Some(xvfb::Xvfb::run(XVFB_PORT)?)
    } else {
        None
    };
    let mut eeschema_process = eeschema::Eeschema::run(&args.path_to_sch)?;
    let erc_output = gui::erc::get_erc_output_from_gui().map_err(|e| {
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

fn main() -> Result<(), String> {
    match Options::from_args() {
        Options::RunErc(args) => run_erc(args),
        Options::RunDrc => unimplemented!(),
    }
}
