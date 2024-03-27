use std::net::Ipv4Addr;

#[derive(Debug, clap::Args, Clone)]
#[group(required = true, multiple = false)]
pub struct NetGroup {
    /// Prefix in XX format
    #[clap(short = 'p', long, value_name = "PREFIX")]
    prefix: Option<u8>,
    /// Mask in xxx.xxx.xxx.xxx format
    #[clap(short = 'm', long, value_name = "MASK")]
    mask: Option<String>,
}
#[derive(Debug, clap::Args, Clone)]
#[group(required = true, multiple = false)]
pub struct ScaffoldGroup {
    /// Hosts in XX,X,XX format
    #[clap(long, value_name = "HOSTS")]
    hosts: Option<String>,
    /// Prefixes in XX,XX,XX format
    #[clap(long, value_name = "PREFIXES")]
    prefixes: Option<String>,
}

#[derive(Clone, Copy)]
pub struct Address {
    first_byte: u8,
    second_byte: u8,
    third_byte: u8,
    fourth_byte: u8,
}
impl Address {
    pub fn new(first_byte: u8, second_byte: u8, third_byte: u8, fourth_byte: u8) -> Self {
        Address {
            first_byte,
            second_byte,
            third_byte,
            fourth_byte,
        }
    }
    pub fn get(&self) -> Address {
        *self
    }
    pub fn as_vec(&self) -> Vec<u8> {
        vec![
            self.first_byte,
            self.second_byte,
            self.third_byte,
            self.fourth_byte,
        ]
    }
}

pub struct Net {
    network_address: Address,
    broadcast: Address,

    mask: Address,
    wildcard: Address,
    prefix: u8,
}
impl Net {
    pub fn new(network_address: Address, mask: Address) -> Self {
        Net {
            network_address,
            broadcast: broadcast(&network_address, &mask),

            mask,
            prefix: prefix_from_mask(&mask),
            wildcard: wildcard(&mask),
        }
    }
    pub fn get_network_address(&self) -> Address {
        self.network_address.get()
    }
    pub fn get_broadcast(&self) -> Address {
        self.broadcast.get()
    }
    pub fn __repr__(&self) -> String {
        format!(
            r#"Network address: {}.{}.{}.{}
Broadcast: {}.{}.{}.{}

Mask: {}.{}.{}.{}
Wildcard: {}.{}.{}.{}
Prefix: {}"#,
            self.network_address.as_vec()[0],
            self.network_address.as_vec()[1],
            self.network_address.as_vec()[2],
            self.network_address.as_vec()[3],
            self.broadcast.as_vec()[0],
            self.broadcast.as_vec()[1],
            self.broadcast.as_vec()[2],
            self.broadcast.as_vec()[3],
            self.mask.as_vec()[0],
            self.mask.as_vec()[1],
            self.mask.as_vec()[2],
            self.mask.as_vec()[3],
            self.wildcard.as_vec()[0],
            self.wildcard.as_vec()[1],
            self.wildcard.as_vec()[2],
            self.wildcard.as_vec()[3],
            self.prefix
        )
    }
}
pub fn prefix_from_mask(mask: &Address) -> u8 {
    let first_binary = format!("{:b}", mask.first_byte);
    let second_binary = format!("{:b}", mask.second_byte);
    let third_binary = format!("{:b}", mask.third_byte);
    let fourth_binary = format!("{:b}", mask.fourth_byte);
    let prefix = u8::try_from(
        first_binary.matches("1").count()
            + second_binary.matches("1").count()
            + third_binary.matches("1").count()
            + fourth_binary.matches("1").count(),
    );
    match prefix {
        Ok(num) => num,
        Err(error) => {
            println!("{error}");
            0
        }
    }
}
pub fn mask_from_prefix(prefix: &mut u8) -> Address {
    // needs prefix.clone() in parameters
    let mut address_vec: Vec<u8> = vec![];
    for _ in 0..4 {
        if *prefix >= 8 {
            address_vec.push(255);
            *prefix -= 8;
        } else {
            let last_byte: String = format!(
                "{}{}",
                "1".repeat(match usize::try_from(*prefix) {
                    Ok(num) => num,
                    Err(err) => {
                        println!("{err}");
                        0
                    }
                }),
                "0".repeat(match usize::try_from(8 - *prefix) {
                    Ok(num) => num,
                    Err(err) => {
                        println!("{err}");
                        0
                    }
                })
            );
            let decimal_byte = match u8::from_str_radix(last_byte.as_str(), 2) {
                Ok(num) => num,
                Err(err) => {
                    println!("{err}");
                    0
                }
            };
            address_vec.push(decimal_byte);
            break;
        }
    }
    while address_vec.len() < 4 {
        address_vec.push(0)
    }
    Address {
        first_byte: address_vec[0],
        second_byte: address_vec[1],
        third_byte: address_vec[2],
        fourth_byte: address_vec[3],
    }
}

pub fn broadcast(network_address: &Address, mask: &Address) -> Address {
    let first_byte = (network_address.first_byte) | (!mask.first_byte);
    let second_byte = (network_address.second_byte) | (!mask.second_byte);
    let third_byte = (network_address.third_byte) | (!mask.third_byte);
    let fourth_byte = (network_address.fourth_byte) | (!mask.fourth_byte);
    Address {
        first_byte,
        second_byte,
        third_byte,
        fourth_byte,
    }
}
pub fn wildcard(mask: &Address) -> Address {
    let first_byte = !mask.first_byte;
    let second_byte = !mask.second_byte;
    let third_byte = !mask.third_byte;
    let fourth_byte = !mask.fourth_byte;
    Address {
        first_byte,
        second_byte,
        third_byte,
        fourth_byte,
    }
}
pub fn normalize_number(input: &mut u32) {
    *input += 1; //1, not 2, because that will change only the last bit -> if i added 2, network
                 // with 2 host would turn into 8
                 // if i add 1, the network will stay in prefix 30 as wanted
    *input = u32::pow(
        2,
        match u32::try_from(format!("{:b}", *input).len()) {
            Ok(num) => num,
            Err(error) => {
                println!("{error}");
                0
            }
        },
    )
}
pub fn hosts_to_mask(hosts: u32) -> Address {
    let mut prefix: u8 = 32 - u8::try_from((format!("{:b}", hosts - 1)).len()).unwrap();
    mask_from_prefix(&mut prefix)
}
fn next_address(addr: &Address) -> Address {
    let first_byte: (u8, u8);
    let second_byte: (u8, u8);
    let third_byte: (u8, u8);
    let fourth_byte: (u8, u8) = match addr.fourth_byte.checked_add(1) {
        Some(num) => (num, 0),
        _ => (0, 1),
    };
    third_byte = match addr.third_byte.checked_add(fourth_byte.1) {
        Some(num) => (num, 0),
        _ => (0, 1),
    };
    second_byte = match addr.second_byte.checked_add(third_byte.1) {
        Some(num) => (num, 0),
        _ => (0, 1),
    };
    first_byte = match addr.first_byte.checked_add(second_byte.1) {
        Some(num) => (num, 0),
        _ => (0, 1),
    };
    Address {
        first_byte: first_byte.0,
        second_byte: second_byte.0,
        third_byte: third_byte.0,
        fourth_byte: fourth_byte.0,
    }
}
pub fn scaffold_hosts(base_network: &Net, mut hosts_vec: Vec<u32>) -> Vec<Net> {
    let mut networks: Vec<Net> = vec![];
    hosts_vec.sort();
    hosts_vec.reverse();
    networks.push(Net::new(
        base_network.get_network_address(),
        hosts_to_mask(hosts_vec[0]),
    ));
    for index in 1..hosts_vec.len() {
        let mask = hosts_to_mask(hosts_vec[index]);
        let net_address = next_address(&networks[index - 1].get_broadcast());
        let net: Net = Net::new(net_address, mask);
        networks.push(net);
    }
    networks
}
pub fn scaffold_prefixes(base_network: &Net, mut prefixes_vec: Vec<u8>) -> Vec<Net> {
    let mut networks: Vec<Net> = vec![];
    prefixes_vec.sort();
    networks.push(Net::new(
        base_network.get_network_address(),
        mask_from_prefix(&mut prefixes_vec[0]),
    ));
    for index in 1..prefixes_vec.len() {
        let mask = mask_from_prefix(&mut prefixes_vec[index]);
        let net_address = next_address(&networks[index - 1].get_broadcast());
        let net: Net = Net::new(net_address, mask);
        networks.push(net);
    }
    networks
}
pub fn create_single_net(addr: String, size: crate::NetGroup) {
    if addr.len() < 7 && addr.len() > 15 {
        println!("invalid address");
        std::process::exit(0);
    }
    let net_addr: Ipv4Addr = match addr.parse() {
        Ok(res) => res,
        Err(err) => {
            println!("{err}");
            std::process::exit(0);
        }
    };
    if size.mask != None {
        if size.mask.as_ref().unwrap().len() < 7 && size.mask.as_ref().unwrap().len() > 15 {
            println!("invalid mask");
            std::process::exit(0);
        }
        let mask: Ipv4Addr = match size.mask.as_ref().unwrap().parse() {
            Ok(res) => res,
            Err(err) => {
                println!("{err}");
                std::process::exit(0);
            }
        };
        let created = Net::new(
            Address::new(
                net_addr.octets()[0],
                net_addr.octets()[1],
                net_addr.octets()[2],
                net_addr.octets()[3],
            ),
            Address::new(
                mask.octets()[0],
                mask.octets()[1],
                mask.octets()[2],
                mask.octets()[3],
            ),
        );
        println!("{}", created.__repr__());
        std::process::exit(0);
    }
    if size.prefix != None {
        let mut prefix = size.prefix.unwrap();
        let mask = mask_from_prefix(&mut prefix);

        let created = Net::new(
            Address::new(
                net_addr.octets()[0],
                net_addr.octets()[1],
                net_addr.octets()[2],
                net_addr.octets()[3],
            ),
            mask,
        );
        println!("{}", created.__repr__());
        std::process::exit(0);
    }
}
pub fn create_scaffold(addr: String, size: crate::NetGroup, input: ScaffoldGroup) {
    println!("\n");
    if addr.len() < 7 && addr.len() > 15 {
        println!("invalid address");
        std::process::exit(0);
    }
    let net_addr: Address = {
        let addr: Ipv4Addr = match addr.parse() {
            Ok(res) => res,
            Err(err) => {
                println!("{err}");
                std::process::exit(0);
            }
        };
        Address::new(
            addr.octets()[0],
            addr.octets()[1],
            addr.octets()[2],
            addr.octets()[3])
    };
    let mask: Address = {
        if size.mask != None {
            if size.mask.as_ref().unwrap().len() < 7 && size.mask.as_ref().unwrap().len() > 15 {
                println!("invalid mask");
                std::process::exit(0);
            }
            let mask: Address = {
                let temp_mask: Ipv4Addr = match size.mask.as_ref().unwrap().parse() {
                    Ok(res) => res,
                    Err(err) => {
                        println!("{err}");
                        std::process::exit(0);
                    }
                };
                Address::new(
                    temp_mask.octets()[0],
                    temp_mask.octets()[1],
                    temp_mask.octets()[2],
                    temp_mask.octets()[3],
                )
            };
            mask
        } else {
            let mut prefix = size.prefix.unwrap();
            mask_from_prefix(&mut prefix)
        }
    };
    if input.prefixes != None {
        let prefixes_vec: Vec<u8> =  {
            let mut temp_vec_u8: Vec<u8> = vec![];
            for prefix in input.prefixes.unwrap().split(",") {
            temp_vec_u8.push(u8::from(prefix.trim().parse::<u8>().unwrap()));
            }
            temp_vec_u8

        };
        let scaffolded = scaffold_prefixes(&Net::new(net_addr, mask), prefixes_vec);
        for single in scaffolded {
            println!("\n\n\n{}", single.__repr__());
        }
    }
    else {

        let hosts_vec: Vec<u32> =  {
            let mut temp_vec: Vec<u32> = vec![];
            for hosts in input.hosts.unwrap().split(",") {
                let mut host = u32::from(hosts.trim().parse::<u32>().unwrap());
                normalize_number(&mut host);
                temp_vec.push(host);
            };
            temp_vec

        };
        let scaffolded = scaffold_hosts(&Net::new(net_addr, mask), hosts_vec);
        for single in scaffolded {
            println!("{}\n\n\n", single.__repr__());
        }
    }
}
