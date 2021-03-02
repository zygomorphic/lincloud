use port_check::free_local_port;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::path::PathBuf;
use structopt::StructOpt;
use local_ipaddress;

#[derive(Clone, Debug)]
pub struct LinConfig {
    // IP address(es) on which linc service will be available
    pub interfaces: Vec<IpAddr>,
    // Port on which linc service will be listening
    pub port: u16,
    // Path to be served by linc service
    pub path: std::path::PathBuf,
}

impl Default for LinConfig {
    fn default() -> Self {
        LinConfig {
            interfaces: vec![
                IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)),
                IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            ],
            port: 8136,
            path: PathBuf::from("./lin_home"),
        }
    }
}

impl LinConfig {
    fn print(&self) {
        println!("\tinterfaces {:?}",self.interfaces);
        println!("\tport {:?}",self.port);
        println!("\tpath {:?}",self.path);
        println!(" Service starting ...");
        println!("{}", local_ipaddress::get().unwrap());
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "linc")]
struct LincCLIArgs {
    #[structopt(long)]
    daemon: bool,
    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    mode: CliMod,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "linc", about = "the stupid content tracker")]
pub enum CliMod {
    #[structopt(name = "cli", about = "cli mode")]
    Client {
        // IP address(es) on which linc service will be available
        #[structopt(
            short = "i",
            long = "interfaces",
            parse(try_from_str = parse_interface),
            number_of_values = 1,
        )]
        interfaces: Vec<IpAddr>,
        // Port on which linc service will be listening
        #[structopt(short = "p", long = "port", default_value = "8080")]
        port: u16,
        // Path to be served by linc service
        #[structopt(name = "PATH", parse(from_os_str))]
        path: Option<PathBuf>,
    },
    #[structopt(name = "yml", about = "yml mode")]
    YmlConf {
        #[structopt(parse(from_os_str))]
        file: Vec<PathBuf>,
    },
    #[structopt(name = "def", about = "default mode")]
    Default,
}

pub fn get_starting_mode() -> CliMod {
    let args = LincCLIArgs::from_args();
    args.mode
}

pub fn get_cli_mode(mode: CliMod) -> LinConfig {
    // println!("starting with {:?}", mode);
    let config: LinConfig;
    if let CliMod::Client {
        interfaces,
        port,
        path,
    } = mode
    {
        let interfaces = if !interfaces.is_empty() {
            interfaces
        } else {
            vec![
                IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)),
                IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            ]
        };
        let port = match port {
            0 => free_local_port().expect("no free ports available"),
            _ => port,
        };
        let linc_home = "./linc_home";

        let path = path.unwrap_or_else(|| PathBuf::from(linc_home));
        config = LinConfig {
            interfaces,
            port,
            path,
        };
    } else {
        config = LinConfig::default();
    };
    // println!("service starting with ->{:?}", config);
    config.print();
    return config;
}

pub fn get_yml_mode(mode: CliMod) -> LinConfig {
    let config: LinConfig;
    if let CliMod::YmlConf { file: config_file } = mode {
        config = LinConfig::default();
        println!("the config file path is {:?}", config_file);
    } else {
        config = LinConfig::default();
    };
    config.print();
    // println!("service starting with ->{:?}", config);
    return config;
}

pub fn get_def_mode() -> LinConfig {
    let config: LinConfig = LinConfig::default();
    config.print();
    // println!("service starting with -> {:?}", config);
    return config;
}

/// Checks wether an interface is valid, i.e. it can be parsed into an IP address
fn parse_interface(src: &str) -> Result<IpAddr, std::net::AddrParseError> {
    src.parse::<IpAddr>()
}
