use clap::Parser;
use ipnetwork::Ipv4Network;

/// CIDRをIPアドレスの一覧に変換します
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// 一覧を表示させたいCIDRを入れる(IPv4のみ) e.g.: 127.0.0.1/30
    #[arg(long)]
    cidr: String,
    /// BigQuery用のクエリを出力する
    #[arg(short, long, default_value_t = false)]
    query: bool,
}

fn main() {
    let args = Args::parse();

    if let Err(e) = validate_cidr(args.cidr.clone()) {
        eprintln!("Error: {}. CIDRは192.168.0.0/16のように表記してください。", e);
        std::process::exit(1);
    }

    let cidr = args.cidr;
    let is_query = args.query;

    scan_network(cidr, is_query);
}

fn validate_cidr(cidr: String) -> Result<String, String> {
    // Validate CIDR
    if cidr.parse::<Ipv4Network>().is_err() {
        return Err("Invalid CIDR".to_string());
    }

    Ok(cidr)
}

fn scan_network(cidr: String, is_query: bool) {
    let network = cidr.parse::<Ipv4Network>().unwrap();
    if is_query {
        println!("(");
        for ip in network.iter() {
            //最後のみカンマをつけない
            if ip == network.iter().last().unwrap() {
                println!("  '{ip}'");
            } else {
                println!("  '{ip}',");
            }
        }
        println!(")");
    } else {
        for ip in network.iter() {
            println!("{}", ip);
        }
    }
}
