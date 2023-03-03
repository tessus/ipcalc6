//ipcalc6 
//Copyright (C) 2022  Luc Baillargeon <all@200013.net>
//This program is free software: you can redistribute it and/or modify
//it under the terms of the GNU General Public License version 3 as 
//published by the Free Software Foundation.
//This program is distributed in the hope that it will be useful,
//but WITHOUT ANY WARRANTY; without even the implied warranty of
//MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//GNU General Public License for more details.
//You should have received a copy of the GNU General Public License
//along with this program.  If not, see <https://www.gnu.org/licenses/>.


use std::env;
use std::process::exit;
use std::str::FromStr;
use std::net::AddrParseError;
use std::net::IpAddr;
use colored::Colorize;
use std::num::ParseIntError;
use std::collections::VecDeque;

fn help() {
    println!("\n\tUsage :");
    println!("\t\tipcalc6 [ipv6_address]");
    println!("\t\tipcalc6 [ipv6_address]/[prefix]");
    println!("\n\tExample :");
    println!("\t\tipcalc6 fe80::fcba:82ff:fe06:c2f1");
    println!("\t\tipcalc6 fe80::fcba:82ff:fe06:c2f1/64\n");
}


fn error_arg_number(arg_len: usize) -> () {
    eprintln!("error : ipcalc6 requires 1 argument and you provided {} arguments", arg_len - 1);
    help();
    exit(1);
}


fn is_v6_valid(ipv6: &str) -> bool {
    let parse_result: Result<IpAddr, AddrParseError> = IpAddr::from_str(&ipv6);
    if parse_result.is_ok() == true {
        //println!("{:?}", parse_result.ok().unwrap());
        if parse_result.ok().unwrap().is_ipv6() == true {
            //println!("valid ipv6 address");
            return true
        }
        else {
            eprintln!("Invalid address format");
            help();
            exit(1);
        }
    }
    else {
        //println!("{:?}", parse_result.err().unwrap());
        eprintln!("Invalid address format");
        help();
        exit(1);
    }
}


fn is_prefix_valid(prefix_size: i16) -> bool {
    if prefix_size >= 7 && prefix_size <= 128 {
        return true;
    }
    else {
        let message = format!("\nERROR : Invalid prefix /{}, prefix should be an integer between 7 and 128\n\t(smallest prefix is /7 for adress type : Unique Local Addresses)\n\t(maximum possible prefix is 128)\n\n", prefix_size).red();
        eprintln!("{}", message);
        exit(1);
    }
}

fn is_prefix_valid_global_unicast(prefix_size: i16) -> bool {       
  if prefix_size >= 48 && prefix_size <= 128 {
        return true;
  }
  else {
        let message = format!("\nERROR : prefix /{} is smaller than /48, this is used in CIDR routing but not allowed as a prefix for an ip address\n\n", prefix_size).yellow();
        println!("{}", message);
        exit(1);
    }
}

fn is_prefix_valid_local_link(prefix_size: i16) -> bool {
    if prefix_size == 64 {
        return true;
    }
    else {
        let message = format!("\nERROR : prefix /{} is not /64, this is not ok with if ip of type : Link-Local Address\n\n", prefix_size).red();
        println!("{}", message);
        exit(1);
    }
}

fn is_prefix_valid_unique_local_address(prefix_size: i16) -> bool {
    if prefix_size >= 48 && prefix_size <= 128  {
        return true;
    }
    else {
        let message = format!("\nERROR : Invalid prefix /{}, prefix for Unique Local Addresses should be between 48 or higher.\n\n", prefix_size).red();
        eprintln!("{}", message);
        exit(1);
    }
}

fn is_prefix_valid_loopback(prefix_size: i16) -> bool {
    if prefix_size == 128 {
        return true;
    }
    else {
        let message = format!("\nERROR : prefix /{} is not /128, this is not ok with if ip of type : Loopback Address\n\n", prefix_size).red();
        println!("{}", message);
        exit(1);
    }
}


fn pad_section(value: &str) -> String {
    let padded_value: String = format!("{:0>4}", &value);
    return padded_value;
}


fn expand_address(ipv6: &str) -> VecDeque<String> {
    let mut block_count: usize = 0;
    let mut expand_vector: VecDeque<String> = VecDeque::<String>::new();
    let parts: Vec<&str> = ipv6.split("::").collect();
    // case if we DONT have a section of zeros abreaviated to ::
    if parts.len() == 1 {
        let ipv6_section =  parts[0].split(":");
        for part in ipv6_section {
            let padded_value: String = pad_section(&part);
            expand_vector.push_back(padded_value);
        }
        return expand_vector;
    }
    // case if we have a section of zeros abreaviated to ::
    else {
        // case if consecutive sections of zeros zeroes in the left part -> e.g ::1
        if parts[0].is_empty() {
            let ipv6_section =  parts[1].split(":");
            for part in ipv6_section {
                let padded_value: String = pad_section(&part);
                expand_vector.push_back(padded_value);
                block_count = block_count + 1;
            }
            let mut block_to_add: usize = 8 - block_count;
            while block_to_add > 0 {
                expand_vector.push_front("0000".to_string());
                block_to_add = block_to_add -1;
            }
            return expand_vector;
        }
        // case if consecutive sections of zeroes in the left part -> e.g : 2a01:e34:eca7:c661::
        else if parts[1].is_empty() {
            let ipv6_section =  parts[0].split(":");
            for part in ipv6_section {
                let padded_value: String = pad_section(&part);
                expand_vector.push_back(padded_value);
                block_count = block_count + 1;
            }
            let mut block_to_add: usize = 8 - block_count;
            while block_to_add > 0 {
                expand_vector.push_back("0000".to_string());
                block_to_add = block_to_add -1;
            }
            return expand_vector;
        }
        // case if consecutive sections of zeroes in the middle -> e.g : 2001:db8::ff00:42:8329
        else {
            // let part: i8;
            let ipv6_section_left: Vec<&str> =  parts[0].split(":").collect();
            let ipv6_section_right: Vec<&str> =  parts[1].split(":").collect();
            block_count = block_count + &ipv6_section_left.len();
            block_count = block_count + &ipv6_section_right.len();
            let mut block_to_add: usize = 8 - block_count;
            for part in ipv6_section_left {
                let padded_value: String = pad_section(&part);
                expand_vector.push_back(padded_value);
            }
            while block_to_add > 0 {
                expand_vector.push_back("0000".to_string());
                block_to_add = block_to_add -1;
            }     
            for part in ipv6_section_right {
                let padded_value: String = pad_section(&part);
                expand_vector.push_back(padded_value);
            }
            return expand_vector;
        }
    }
}


fn hex_to_bin (hexa: char) -> String {
    let return_string: String = format!("{:04b}", hexa.to_digit(16).unwrap().to_string().parse::<u32>().unwrap());
    return return_string;
}

fn bin_to_hex (tup:(&str, i16)) -> (String, String, String) {
    let prefix_size: usize = tup.1.try_into().unwrap();
    let mut hex_counter: usize = 0;
    let mut prefix_counter: usize = 0;
    let mut range_start: usize = 0;
    let mut range_end: usize = 4;
    let mut addr_hex_network: String = String::new();
    let mut addr_hex_subnet: String = String::new();
    let mut addr_hex_client: String = String::new();
    while prefix_counter < 128 {
        // skip the dots incoming from the data representation
        //println!("{}", &tup.0);
        //println!("{}", &tup.0[range_start..range_start+1]);
        if &tup.0[range_start..range_start+1] == "." {
          range_start = range_start + 1;
          range_end = range_end + 1;
        }
        if prefix_counter < 48 {
          if (hex_counter % 4) == 0 && hex_counter != 0 {
            addr_hex_network.push_str(":");
          }
          addr_hex_network.push_str(&format!("{:x}", u32::from_str_radix(&tup.0[range_start..range_end], 2).unwrap()));
          hex_counter = hex_counter + 1;
          prefix_counter = prefix_counter + 4;
        }
        else if prefix_counter >= 48 && prefix_counter < prefix_size {
          if (hex_counter % 4) == 0 {
            addr_hex_subnet.push_str(":");
          }
          addr_hex_subnet.push_str(&format!("{:x}", u32::from_str_radix(&tup.0[range_start..range_end], 2).unwrap()));
          hex_counter = hex_counter + 1;
          prefix_counter = prefix_counter + 4;
        }
        else {
          if (hex_counter % 4) == 0 && hex_counter != 128 {
            addr_hex_client.push_str(":");
          }
          addr_hex_client.push_str(&format!("{:x}", u32::from_str_radix(&tup.0[range_start..range_end], 2).unwrap()));
          hex_counter = hex_counter + 1;
          prefix_counter = prefix_counter + 4;
        }
        range_start = range_start + 4;
        range_end = range_end + 4;
    }
    let tuple = (addr_hex_network, addr_hex_subnet, addr_hex_client);
    return tuple
}


fn detect_type (ipv6: &str) -> String {
    let mut addr_type: String = String::new();
    //detect loopback
    if ipv6 == "::1" {
        addr_type = "Loopback Address".to_string();
        return addr_type;
    }
    let first_char = ipv6.chars().nth(0).unwrap().to_digit(16).unwrap();
    let second_char = ipv6.chars().nth(1).unwrap().to_digit(16).unwrap();
    let third_char = ipv6.chars().nth(2).unwrap().to_digit(16).unwrap();
    let fourth_char = ipv6.chars().nth(3).unwrap().to_digit(16).unwrap();
    // detect global unicast
    if first_char == 2 || first_char == 3 {
        addr_type = "Unicast Global".to_string();
    }
    // detect Link-Local Address
    if first_char == 15 && second_char == 14 && third_char == 8 && fourth_char == 0 {
        addr_type = "Link-Local Address".to_string();
    }
    // detect Unique-Local Address
    if first_char == 15 && second_char == 13 {
        addr_type = "Unique-Local Address".to_string();
    }
    // detect Multicast Address
    if first_char == 15 && second_char == 15 {
        addr_type = "Multicast Address".to_string();
    }
    return addr_type;
}


fn print_address(ipv6: &str, prefix_size: i16) {
    let addr_type: String = detect_type(ipv6);
    if addr_type == "Link-Local Address" {
        is_prefix_valid_local_link(prefix_size);
    }
    if addr_type == "Unique-Local Address" {
        is_prefix_valid_unique_local_address(prefix_size);
    }
    if addr_type == "Loopback Address" {
        is_prefix_valid_loopback(prefix_size);
    }
    if addr_type == "Unicast Global" {
        is_prefix_valid_global_unicast(prefix_size);
    }
    let mut prefix_counter: i16 = 0;
    let mut addr: String = String::new();
    let mut addr_binary_network: String = String::new();
    let mut prefix_binary_network: String = String::new();
    let mut addr_binary_subnet: String = String::new();
    let mut prefix_binary_subnet: String = String::new();
    let mut addr_binary_client: String = String::new();
    let mut prefix_binary_client: String = String::new();
    let mut expanded_address: VecDeque<String> = expand_address(ipv6);
    let dot: char = '.';
    while expanded_address.len() > 0 {
        let part: String = expanded_address.pop_front().unwrap();
        for addr_char in part.chars() {
            addr.push(addr_char);
            let bin: String = hex_to_bin(addr_char);
            for bit in bin.chars() {
                if prefix_counter < 48  {
                    addr_binary_network.push(bit);
                    prefix_binary_network.push('1');
                    prefix_counter = prefix_counter + 1;
                    if (prefix_counter % 16) == 0 {
                        addr_binary_network.push(dot);
                        prefix_binary_network.push(dot);
                    }
                }
                else if prefix_counter >= 48 && prefix_counter < prefix_size {
                    addr_binary_subnet.push(bit);
                    prefix_binary_subnet.push('1');
                    prefix_counter = prefix_counter + 1;
                    if (prefix_counter % 16) == 0 {
                        addr_binary_subnet.push(dot);
                        prefix_binary_subnet.push(dot);
                    }
                }
                else {
                    addr_binary_client.push(bit);
                    prefix_binary_client.push('0');
                    prefix_counter = prefix_counter + 1;
                    if (prefix_counter % 16) == 0 {
                        addr_binary_client.push(dot);
                        prefix_binary_client.push(dot);
                    }
                }
            }
        }
        addr.push(':');
    }
    addr.pop();
    addr_binary_client.pop();
    prefix_binary_client.pop();
    let binary_poss: i128 = 2;
    let client_poss: i128 = prefix_binary_client.chars().count().try_into().unwrap();
    let num_of_host: i128 = binary_poss.pow(client_poss.try_into().unwrap());
    let mut concat_address: String = String::new();
    concat_address.push_str(&addr_binary_network);
    concat_address.push_str(&addr_binary_subnet);
    concat_address.push_str(&addr_binary_client);
    let mut concat_min: String = String::new();
    concat_min.push_str(&addr_binary_network);
    concat_min.push_str(&addr_binary_subnet);
    concat_min.push_str(&prefix_binary_client);
    let mut concat_max: String = String::new();
    concat_max.push_str(&addr_binary_network);
    concat_max.push_str(&addr_binary_subnet);
    concat_max.push_str(&prefix_binary_client.replace("0","1"));
    let mut concat_prefix: String = String::new();
    concat_prefix.push_str(&prefix_binary_network);
    concat_prefix.push_str(&prefix_binary_subnet);
    concat_prefix.push_str(&prefix_binary_client);
    let addr_tuple:(&str, i16) = (&concat_address, prefix_size);
    let addr_hex_tuple = bin_to_hex(addr_tuple);
    let prefix_tuple:(&str, i16) = (&concat_prefix, prefix_size);
    let prefix_hex_tuple = bin_to_hex(prefix_tuple);
    let min_tuple:(&str, i16) = (&concat_min, prefix_size);
    let min_hex_tuple = bin_to_hex(min_tuple);
    let max_tuple:(&str, i16) = (&concat_max, prefix_size);
    let max_hex_tuple = bin_to_hex(max_tuple);
    println!("\nType:\t\t{}", addr_type);
    println!("Address:\t{}\t\tPrefix:\t{}", addr, prefix_size);
    println!("Hosts/Net:\t{}\n", num_of_host);
    println!("Address:\t{}{}{}", addr_hex_tuple.0.red(), addr_hex_tuple.1.yellow(), addr_hex_tuple.2.green());
    println!("Prefix:\t\t{}{}{}", prefix_hex_tuple.0.yellow(), prefix_hex_tuple.1.yellow(), prefix_hex_tuple.2.green());
    println!("HostMin:\t{}{}{}", min_hex_tuple.0.red(), min_hex_tuple.1.yellow(), min_hex_tuple.2.green());
    println!("HostMax:\t{}{}{}\n", max_hex_tuple.0.red(), max_hex_tuple.1.yellow(), max_hex_tuple.2.green());
    println!("Address:\t{}{}{}", addr_binary_network.to_string().red(), addr_binary_subnet.to_string().yellow(), addr_binary_client.to_string().green());
    println!("Prefix:\t\t{}{}{}", prefix_binary_network.yellow(), prefix_binary_subnet.yellow(), prefix_binary_client.green());
    println!("HostMin:\t{}{}{}", addr_binary_network.to_string().red(), addr_binary_subnet.to_string().yellow(), prefix_binary_client.green());
    println!("HostMax:\t{}{}{}\n", addr_binary_network.to_string().red(), addr_binary_subnet.to_string().yellow(), prefix_binary_client.replace("0","1").green());
}

fn main() {
    // SECTION : USER INPUT & VALIDATION
    let args: Vec<String> = env::args().collect();
    let arg_len: usize = args.len();
    if arg_len !=2 {
        error_arg_number(arg_len);
    }
    if &args[1] == "help" {
        help();
        exit(0);
    }
    // this condition if the user input is ip/prefix (e.g fe80::fcba:82ff:fe06:c2f1/64)
    if args[1].contains("/") {
        //we have to split the string on the "/" and collect the parts in a vector
        let user_input: Vec<&str> = args[1].split("/").collect();
        let ipv6: &str = user_input[0];
        let prefix: Result<i16, ParseIntError> = user_input[1].parse::<i16>();
        if prefix.is_err() {
            let message = format!("ERROR : Invalid prefix, prefix should be an integer between 7 and 128").red();
            eprintln!("{}", message);
            exit(1);
        }
        else {
            if is_v6_valid(ipv6) == true {
                let prefix_size: i16 = prefix.ok().unwrap();
                if is_prefix_valid(prefix_size) {
                    print_address(ipv6, prefix_size);
                }
                else {
                    let message = format!("ERROR : Unexpected error").red();
                    eprintln!("{}", message);
                    exit(1);
                }
            }
            
            else {
                let message = format!("ERROR : Unexpected error").red();
                eprintln!("{}", message);
                exit(1);
            }
        }
    }
    // user added an ip only (e.g fe80::fcba:82ff:fe06:c2f1) we will set the prefix to 64
    else {
        let ipv6: &str = &args[1];
        if is_v6_valid(ipv6) == true {
            let prefix_size: i16 = 64;
            print_address(ipv6, prefix_size);
        }
        else {
            let message = format!("ERROR : Unexpected error").red();
            eprintln!("{}", message);
            exit(1);
        }
    }
}
