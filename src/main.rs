use colored::*; // Importa el trait para usar colores
use std::io;

fn main() {
    // Imprimir el logo en ASCII en rojo
    println!("{}", "▗▄▄▖ ▗▞▀▚▖▄▄▄▄     ■   ▄▄▄ ▗▞▀▜▌▄▄▄▄  ▗▞▀▚▖   ■".red());
    println!("{}", "▐▌ ▐▌▐▛▀▀▘█   █ ▗▄▟▙▄▖█    ▝▚▄▟▌█   █ ▐▛▀▀▘▗▄▟▙▄▖".red());
    println!("{}", "▐▛▀▘ ▝▚▄▄▖█   █   ▐▌  █         █   █ ▝▚▄▄▖  ▐▌".red());
    println!("{}", "▐▌                ▐▌                         ▐▌".red());
    println!("{}", "                  ▐▌                         ▐▌".red());
    println!("{}", "                                                 ".red());
    println!("{}", "               Coded by V3cTr4x                    ".red());

    println!("----------------------------");
    println!("Option menu:");
    println!("1. Network Attack");
    println!("2. Exploit searcher");
    println!("3. Exit");        
    println!("----------------------------");
    let mut option = String::new();
    io::stdin().read_line(&mut option).unwrap();
    let option: i32 = option.trim().parse().unwrap(); 

 
    match option {
        1 => network_attack(),
        2 => exploit_searcher(),
        3 => exit_program(),
        _ => println!("Invalid option! Please choose 1, 2, or 3."),
    }
}

fn network_attack() {
    println!("{}", "Network Attack selected!".green());
    
       // Esperar la entrada del usuario para continuar (esperando que presione Enter)
       println!("{}", "Press Enter to continue to the Network Attack submenu...");
       let mut wait = String::new();
       io::stdin().read_line(&mut wait).unwrap();
   
       // Mostrar un submenú para Network Attack
       println!("----------------------------");
       println!("Network Attack Submenu:");
       println!("1. Scan network (NMAP)");
       println!("2. Perform DOS attack");
       println!("3. Return to main menu");
       println!("----------------------------");
   
       // Leer la opción del submenú
       let mut sub_option = String::new();
       io::stdin().read_line(&mut sub_option).unwrap();
       let sub_option: i32 = sub_option.trim().parse().unwrap();
   
       
       match sub_option {
           1 => scan_network(),
         //  2 => dos_attack(),
         //  3 => return_to_main_menu(),
           _ => println!("Invalid option! Please choose 1, 2, or 3."),
       }
   }


fn exploit_searcher() {
    println!("{}", "Exploit searcher selected!".green());
    
}

fn exit_program() {
    println!("{}", "Exiting program...".yellow());
    
}


fn scan_network() {
    loop {
        println!(" ------------------------------------------");
        println!("| >Unleashing the power of the network...< |");
        println!(" ------------------------------------------");

        println!("\n1. Basic Scans");
        println!("\t[1] Active Host Scan");
        println!("\t[2] Open Port Scan");
        println!("\t[3] Specific Port Scan");
        println!("\t[4] List Scan");

        println!("\n2. Advanced Scans");
        println!("\t[5] OS Scan");
        println!("\t[6] Full Network Scan (OS, Ports and Services)");
        println!("\t[7] Vulnerability Scan");
        println!("\t[8] Firewall Detection Scan");
        println!("\t[9] Traceroute Scan");
        println!("\t[10] IDS Detection Scan");

        println!("\n3. Port and Service Scans");
        println!("\t[12] TCP Port Scan");
        println!("\t[13] UDP Port Scan");
        println!("\t[14] Service and Version Scan");
        println!("\t[15] Stealth Scan (SYN)");
        println!("\t[16] Banner Grabbing Scan");
        println!("\t[17] Idle Scan");
        println!("\t[18] SCTP Init Scan");
        println!("\t[19] SCTP COOKIE-ECHO Scan");

        println!("\n4. Aggressive Scans");
        println!("\t[20] Intensive Scan");
        println!("\t[21] Full Aggressive Scan");

        println!("\n5. Scripted Scans");
        println!("\t[22] HTTP Enumeration");
        println!("\t[23] SMB OS Discovery");
        println!("\t[24] DNS Brute Force");
        println!("\t[25] DNS Zone Transfer");
        println!("\t[26] FTP Anonymous Login");
        println!("\t[27] SNMP Information");
        println!("\t[28] SSL/TLS Scan");
        println!("\t[29] NTP Monlist");
        println!("\t[30] SMB Vulnerability Scan");

        println!("\n6. Evasion and Fragmentation");
        println!("\t[31] Fragmentation Scan");
        println!("\t[32] FTP Bounce Scan");
        println!("\t[33] Decoy Scan");
        println!("\t[34] Randomize Hosts Order");
        println!("\t[35] Slow Scan");
        println!("\t[36] MAC Address Spoofing");
        println!("\t[37] Bad TCP Checksum Scan");
        println!("\t[38] IP Protocol Scan");

        println!("\n7. Specific Purpose Scans");
        println!("\t[39] IPv6 Scan");
        println!("\t[41] Timing Templates");

        println!("\n8. Saving Results");
        println!("\t[42] Save Results in XML");
        println!("\t[43] Save Results in Normal Text");
        println!("\t[44] Save Results in Grepable Format");

        println!("\n9. Show Menu");
        println!("\t[45] Multiple Options");

        println!("\n10. Exit Network Attack");
        println!("\t[46] Exit Network Attack");

       

        let mut option = String::new();
        io::stdin().read_line(&mut option).unwrap();
        let option: i32 = option.trim().parse().unwrap();

        match option {
            //Active Host Scan
            1 => {
                use std::io::{self, Write};
                use std::process::Command;
            
                let mut target = String::new();
                print!("\nEnter the target IP or network name: ");
                io::stdout().flush().unwrap(); // Asegura que el mensaje se imprima antes de leer
            
                io::stdin().read_line(&mut target).unwrap();
                let target = target.trim(); // Remueve el salto de línea
            
                let output = Command::new("nmap")
                    .arg("-sn")
                    .arg(target)
                    .output()
                    .expect("failed to execute nmap");
            
                println!("\nOutput:\n{}", String::from_utf8_lossy(&output.stdout));
                break;
            },
            
            // 2. Open Port Scan
            2 => {
                let mut host = String::new();
                print!("\nEnter the target IP or hostname: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut host).unwrap();
                let host = host.trim();

                let output = Command::new("nmap")
                    .arg(host)
                    .output()
                    .expect("Failed to execute nmap");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 3. Specific Port Scan
            3 => {
                let mut ports = String::new();
                let mut target = String::new();

                print!("\nEnter the port(s) to scan (e.g 80,443): ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut ports).unwrap();

                print!("\nEnter the target IP or hostname: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut target).unwrap();

                let ports = ports.trim();
                let target = target.trim();

                let output = Command::new("nmap")
                    .args(["-p", ports, target])
                    .output()
                    .expect("Failed to execute nmap");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 4. List Scan
            4 => {
                let mut target = String::new();
                print!("\nEnter the network IP or domain name: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut target).unwrap();
                let target = target.trim();

                let output = Command::new("nmap")
                    .args(["-sL", target])
                    .output()
                    .expect("Failed to execute nmap");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 5. OS Scan
            5 => {
                let mut host = String::new();
                print!("\nEnter the host IP or name: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut host).unwrap();
                let host = host.trim();

                let output = Command::new("nmap")
                    .args(["-O", host])
                    .output()
                    .expect("Failed to execute nmap");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 6. Full Network Scan (OS, Ports and Services)
            6 => {
                let mut host = String::new();
                print!("\nEnter the host IP or hostname: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut host).unwrap();
                let host = host.trim();

                let output = Command::new("nmap")
                    .args(["-A", host])
                    .output()
                    .expect("Failed to execute nmap");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 7. Vulnerability Scan
            7 => {
                let mut host = String::new();
                print!("\nEnter the host IP or hostname: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut host).unwrap();
                let host = host.trim();

                let output = Command::new("nmap")
                    .args(["--script=vuln", host])
                    .output()
                    .expect("Failed to execute nmap");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },
            // 8. Firewall Detection Scan (-sA)
            8 => {
                let mut host = String::new();
                print!("\nEnter the host IP or hostname: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut host).unwrap();
                let host = host.trim();

                let output = Command::new("nmap")
                    .args(["-sA", host])
                    .output()
                    .expect("Failed to execute nmap");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 9. Traceroute Scan (--traceroute)
            9 => {
                let mut host = String::new();
                print!("\nEnter the host IP or hostname: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut host).unwrap();
                let host = host.trim();

                let output = Command::new("nmap")
                    .args(["--traceroute", host])
                    .output()
                    .expect("Failed to execute nmap");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 10. IDS Detection Scan (firewalk script)
            10 => {
                let mut host = String::new();
                print!("\nEnter the host IP or hostname: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut host).unwrap();
                let host = host.trim();

                let output = Command::new("nmap")
                    .args(["--script=firewalk", host])
                    .output()
                    .expect("Failed to execute nmap");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 12. TCP Port Scan (-sT)
            12 => {
                let mut host = String::new();
                print!("\nEnter the host IP or hostname: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut host).unwrap();
                let host = host.trim();

                let output = Command::new("nmap")
                    .args(["-sT", host])
                    .output()
                    .expect("Failed to execute nmap");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 13. UDP Port Scan (-sU)
            13 => {
                let mut host = String::new();
                print!("\nEnter the host IP or hostname: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut host).unwrap();
                let host = host.trim();

                let output = Command::new("nmap")
                    .args(["-sU", host])
                    .output()
                    .expect("Failed to execute nmap");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 14. Service and Version Scan (-sV)
            14 => {
                let mut host = String::new();
                print!("\nEnter the host IP or hostname: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut host).unwrap();
                let host = host.trim();

                let output = Command::new("nmap")
                    .args(["-sV", host])
                    .output()
                    .expect("Failed to execute nmap");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },
            // 15. Stealth Scan (-sS)
            15 => {
                let mut host = String::new();
                print!("\nEnter the host IP or hostname: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut host).unwrap();
                let host = host.trim();

                let output = Command::new("nmap")
                    .args(["-sS", host])
                    .output()
                    .expect("Failed to execute stealth scan");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 16. Banner Grabbing Scan (-sV --script=banner)
            16 => {
                let mut host = String::new();
                print!("\nEnter the host IP or hostname: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut host).unwrap();
                let host = host.trim();

                let output = Command::new("nmap")
                    .args(["-sV", "--script=banner", host])
                    .output()
                    .expect("Failed to execute banner grabbing scan");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 17. Idle Scan (-sI <zombie> <target>)
            17 => {
                let mut zombie = String::new();
                let mut target = String::new();
                print!("\nEnter the zombie host IP: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut zombie).unwrap();
                print!("\nEnter the target IP or hostname: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut target).unwrap();

                let zombie = zombie.trim();
                let target = target.trim();

                let output = Command::new("nmap")
                    .args(["-sI", zombie, target])
                    .output()
                    .expect("Failed to execute idle scan");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 18. SCTP INIT Scan (-sY)
            18 => {
                let mut host = String::new();
                print!("\nEnter the host IP or hostname: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut host).unwrap();
                let host = host.trim();

                let output = Command::new("nmap")
                    .args(["-sY", host])
                    .output()
                    .expect("Failed to execute SCTP INIT scan");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 19. SCTP COOKIE-ECHO Scan (-sZ)
            19 => {
                let mut host = String::new();
                print!("\nEnter the host IP or hostname: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut host).unwrap();
                let host = host.trim();

                let output = Command::new("nmap")
                    .args(["-sZ", host])
                    .output()
                    .expect("Failed to execute SCTP COOKIE-ECHO scan");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 20. Intensive Aggressive Scan (-T4 -A -v)
            20 => {
                let mut host = String::new();
                print!("\nEnter the host IP or hostname: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut host).unwrap();
                let host = host.trim();

                let output = Command::new("nmap")
                    .args(["-T4", "-A", "-v", host])
                    .output()
                    .expect("Failed to execute aggressive scan");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 21. Full Aggressive Scan (-A)
            21 => {
                let mut host = String::new();
                print!("\nEnter the host IP or hostname: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut host).unwrap();
                let host = host.trim();

                let output = Command::new("nmap")
                    .args(["-A", host])
                    .output()
                    .expect("Failed to execute full aggressive scan");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 22. HTTP Enumeration (--script=http-enum)
            22 => {
                let mut target = String::new();
                print!("\nEnter the target IP or domain name: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut target).unwrap();
                let target = target.trim();

                let output = Command::new("nmap")
                    .args(["--script=http-enum", target])
                    .output()
                    .expect("Failed to execute HTTP enumeration scan");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 23. SMB OS Discovery (--script=smb-os-discovery)
            23 => {
                let mut host = String::new();
                print!("\nEnter the host IP or hostname: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut host).unwrap();
                let host = host.trim();

                let output = Command::new("nmap")
                    .args(["--script=smb-os-discovery", host])
                    .output()
                    .expect("Failed to execute SMB OS Discovery scan");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 24. DNS Brute Force (--script=dns-brute)
            24 => {
                let mut target = String::new();
                print!("\nEnter the target IP or domain name: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut target).unwrap();
                let target = target.trim();

                let output = Command::new("nmap")
                    .args(["--script=dns-brute", target])
                    .output()
                    .expect("Failed to execute DNS Brute Force scan");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 25. DNS Zone Transfer (--script=dns-zone-transfer)
            25 => {
                let mut target = String::new();
                print!("\nEnter the target IP or domain name: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut target).unwrap();
                let target = target.trim();

                let output = Command::new("nmap")
                    .args(["--script=dns-zone-transfer", target])
                    .output()
                    .expect("Failed to execute DNS Zone Transfer scan");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 26. FTP Anonymous Login (--script=ftp-anon)
            26 => {
                let mut target = String::new();
                print!("\nEnter the target IP or domain name: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut target).unwrap();
                let target = target.trim();

                let output = Command::new("nmap")
                    .args(["--script=ftp-anon", target])
                    .output()
                    .expect("Failed to execute FTP Anonymous Login scan");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 27. SNMP Information scan
            27 => {
                let mut port = String::new();
                let mut host = String::new();
                print!("\nEnter the port to scan (default SNMP is 161): ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut port).unwrap();
                print!("\nEnter the device IP: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut host).unwrap();
                let port = port.trim();
                let host = host.trim();

                let output = Command::new("nmap")
                    .args(["-sU", "-p", port, "--script=snmp-info", host])
                    .output()
                    .expect("Failed to execute SNMP Info scan");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 28. SSL/TLS scan
            28 => {
                let mut host = String::new();
                print!("\nEnter the host IP or hostname: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut host).unwrap();
                let host = host.trim();

                let output = Command::new("nmap")
                    .args(["--script=ssl-cert,ssl-enum-ciphers", host])
                    .output()
                    .expect("Failed to execute SSL/TLS scan");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 29. NTP monlist scan
            29 => {
                let mut port = String::new();
                let mut host = String::new();
                print!("\nEnter the port to scan (default NTP is 123): ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut port).unwrap();
                print!("\nEnter the device IP: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut host).unwrap();
                let port = port.trim();
                let host = host.trim();

                let output = Command::new("nmap")
                    .args(["-sU", "-p", port, "--script=ntp-monlist", host])
                    .output()
                    .expect("Failed to execute NTP monlist scan");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 30. SMB Vulnerability scan
            30 => {
                let mut host = String::new();
                print!("\nEnter the host IP or hostname: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut host).unwrap();
                let host = host.trim();

                let output = Command::new("nmap")
                    .args(["--script=smb-vuln-*", host])
                    .output()
                    .expect("Failed to execute SMB vulnerability scan");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 31. Fragmentation scan
            31 => {
                let mut host = String::new();
                print!("\nEnter the host IP or hostname: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut host).unwrap();
                let host = host.trim();

                let output = Command::new("nmap")
                    .args(["-f", host])
                    .output()
                    .expect("Failed to execute fragmentation scan");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 32. FTP Bounce scan
            32 => {
                let mut ftp_relay = String::new();
                let mut target = String::new();
                print!("\nEnter the FTP relay host: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut ftp_relay).unwrap();
                print!("\nEnter the target device IP: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut target).unwrap();
                let ftp_relay = ftp_relay.trim();
                let target = target.trim();

                let output = Command::new("nmap")
                    .args(["-b", ftp_relay, target])
                    .output()
                    .expect("Failed to execute FTP bounce scan");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 33. Decoy scan
            33 => {
                let mut host = String::new();
                print!("\nEnter the host IP or hostname: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut host).unwrap();
                let host = host.trim();

                let output = Command::new("nmap")
                    .args(["-D", "RND:10", host])
                    .output()
                    .expect("Failed to execute decoy scan");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 34. Randomize host order scan
            34 => {
                let mut host = String::new();
                print!("\nEnter the host IP or hostname: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut host).unwrap();
                let host = host.trim();

                let output = Command::new("nmap")
                    .args(["--randomize-hosts", host])
                    .output()
                    .expect("Failed to execute randomized host scan");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 35. Slow scan (-T0)
            35 => {
                let mut host = String::new();
                print!("\nEnter the host IP or hostname: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut host).unwrap();
                let host = host.trim();

                let output = Command::new("nmap")
                    .args(["-T0", host])
                    .output()
                    .expect("Failed to execute slow scan");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 36. MAC address spoofing
            36 => {
                let mut mac = String::new();
                let mut host = String::new();
                print!("\nEnter the MAC address or vendor name to spoof: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut mac).unwrap();
                print!("\nEnter the device IP or host: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut host).unwrap();
                let mac = mac.trim();
                let host = host.trim();

                let output = Command::new("nmap")
                    .args(["--spoof-mac", mac, host])
                    .output()
                    .expect("Failed to execute MAC spoofing scan");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 37. Bad TCP checksum scan
            37 => {
                let mut host = String::new();
                print!("\nEnter the host IP or hostname: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut host).unwrap();
                let host = host.trim();

                let output = Command::new("nmap")
                    .args(["--badsum", host])
                    .output()
                    .expect("Failed to execute bad checksum scan");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 38. IP Protocol scan
            38 => {
                let mut host = String::new();
                print!("\nEnter the host IP or hostname: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut host).unwrap();
                let host = host.trim();

                let output = Command::new("nmap")
                    .args(["-sO", host])
                    .output()
                    .expect("Failed to execute IP protocol scan");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 39. IPv6 scan
            39 => {
                let mut host = String::new();
                print!("\nEnter the host IP or hostname: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut host).unwrap();
                let host = host.trim();

                let output = Command::new("nmap")
                    .args(["-6", host])
                    .output()
                    .expect("Failed to execute IPv6 scan");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 41. Timing templates
            41 => {
                let mut timing = String::new();
                let mut host = String::new();
                print!("\nEnter timing template (0-5): ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut timing).unwrap();
                print!("\nEnter the host IP or hostname: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut host).unwrap();
                let timing = timing.trim();
                let host = host.trim();

                let output = Command::new("nmap")
                    .args([&format!("-T{}", timing), host])
                    .output()
                    .expect("Failed to execute timing template scan");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 42. Save results as XML
            42 => {
                let mut filename = String::new();
                let mut host = String::new();
                print!("\nEnter the filename for XML output: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut filename).unwrap();
                print!("\nEnter the host IP: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut host).unwrap();
                let filename = filename.trim();
                let host = host.trim();

                let output = Command::new("nmap")
                    .args(["-oX", &format!("{}.xml", filename), host])
                    .output()
                    .expect("Failed to save XML results");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 43. Save results as TXT
            43 => {
                let mut filename = String::new();
                let mut host = String::new();
                print!("\nEnter the filename for TXT output: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut filename).unwrap();
                print!("\nEnter the host IP: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut host).unwrap();
                let filename = filename.trim();
                let host = host.trim();

                let output = Command::new("nmap")
                    .args(["-oN", &format!("{}.txt", filename), host])
                    .output()
                    .expect("Failed to save TXT results");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            // 44. Save results in grepable format
            44 => {
                let mut filename = String::new();
                let mut host = String::new();
                print!("\nEnter the filename for GNMAP output: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut filename).unwrap();
                print!("\nEnter the device IP: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut host).unwrap();
                let filename = filename.trim();
                let host = host.trim();

                let output = Command::new("nmap")
                    .args(["-oG", &format!("{}.gnmap", filename), host])
                    .output()
                    .expect("Failed to save GNMAP results");

                println!("\n{}", String::from_utf8_lossy(&output.stdout));
            },

            46 => break,  
            _ => println!("Invalid option! Please choose a valid option."),
        }
    }
}
