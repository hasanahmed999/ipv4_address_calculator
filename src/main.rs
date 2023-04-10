use std::io::{stdin};

/*
A short program to convert ipv4 to ipv6 in rust
https://www.researchgate.net/figure/IPv4-to-IPv6-Conversion-Method1-In-this-method-firstly-to-convert-the-Decimal-IPv4_fig1_271294793
 */

fn main() {
    // Input an ipv4 address
    let mut ipv4_addr = String::new();
    println!("input ip address to convert (ipv4 format): ");
    stdin().read_line(&mut ipv4_addr).expect("Error");

    //println!("You typed: {}", ip_addr);
    // let octets = ipv4_addr.trim().split(".");
    // // for octet in octets {
    // //     let octet_int: i32 = octet.parse().unwrap();
    // //     let octet_hex = format!("{:X}", octet_int);
    // //     //println!("{}", octet_hex);
    // // }
    // https://stackoverflow.com/questions/26643688/how-do-i-split-a-string-in-rust
    let octets_vec = ipv4_addr.trim().split(".").collect::<Vec<_>>();
    // for octet in octets_vec {
    //     println!("{}", octet);
    // }
    
    // convert to ipv6

    // build final string, append all colons
    // based off formatting from this site 
    // https://dnschecker.org/ipv4-to-ipv6.php


    // Expanded format
    //https://stackoverflow.com/questions/30154541/how-do-i-concatenate-strings
    let mut prefix = "0000:0000:0000:0000:0000:FFFF".to_owned();
    //for octet in octets_vec {//}.step_by(2) {
    for x in (0..octets_vec.len()).step_by(2) {
        prefix.push_str(":");
        let octet_int: i32 = octets_vec[x].parse().unwrap();
        let octet_int_2: i32 = octets_vec[x+1].parse().unwrap();
        //let octet_int: i32 = octet.parse().unwrap();
        let octet_hex = format!("{:0>2X}", octet_int);
        let octet_hex_2 = format!("{:0>2X}", octet_int_2);
        prefix.push_str(&octet_hex);
        prefix.push_str(&octet_hex_2);
        
    }

    // when converting, will always have 0000:0000:0000:0000:0000:FFFF: in front
    println!("{}", prefix);
    
    // Condensed format



}