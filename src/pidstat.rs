#![warn(dead_code)]
use log::info;
use std::time::Duration;
//
// 1677425713    UID       PID    %usr %system  %guest   %wait    %CPU   CPU  Command
// 1677425714   1000     15521   15.00    1.00    0.00    0.00   16.00     2  ckb
//
// 1677425714    UID       PID   kB_rd/s   kB_wr/s kB_ccwr/s iodelay  Command
// 1677425715   1000     15521      0.00      0.00      0.00       0  ckb

struct IoStat {
    timestamp: u64,
    pid: u64,
    kb_rd: f64,
    kb_wr: f64,
    kb_ccwr: f64,
    iodelay: f64,
}

struct CpuStat {
    timestamp: u64,
    pid: u64,
    user_pct: f64,
    system_pct: f64,
    guest_pct: f64,
    wait_pct: f64,
    cpu_pct: f64,
}

pub struct PidStat {
    pub timestamp: u64,

    cpu_user_pct: f64,
    cpu_system_pct: f64,
    cpu_guest_pct: f64,
    cpu_wait_pct: f64,
    pub cpu_cpu_pct: f64,

    io_kb_rd: f64,
    io_kb_wr: f64,
    io_kb_ccwr: f64,
    io_iodelay: f64,
}

pub fn parse(content: &str, duration: Duration) -> Vec<PidStat> {
    let mut results = Vec::new();
    let mut lines = content.lines();
    let mut last_timestamp = None;
    loop {
        match lines.next() {
            Some(line) => {
                if line.is_empty() {
                    continue;
                }
                if line.contains(
                    "UID       PID    %usr %system  %guest   %wait    %CPU   CPU  Command",
                ) {
                    let next_line = lines.next();
                    if next_line.is_none() {
                        info!("get cpu status header, but next line is none");
                        return results;
                    }
                    let items = next_line.unwrap().split_whitespace().collect::<Vec<_>>();
                    if items.len() != 10 {
                        info!("get cpu status header, but next line's non-whitespace items count is not 10");
                        return results;
                    }

                    // parse second line to CpuStat
                    // 1677425713    UID       PID    %usr %system  %guest   %wait    %CPU   CPU  Command
                    // 1677425714   1000     15521   15.00    1.00    0.00    0.00   16.00     2  ckb

                    let pidstatus = CpuStat {
                        timestamp: items.first().unwrap().parse().unwrap(),
                        pid: items[2].parse().unwrap(),
                        user_pct: items[3].parse().unwrap(),
                        system_pct: items[4].parse().unwrap(),
                        guest_pct: items[5].parse().unwrap(),
                        wait_pct: items[6].parse().unwrap(),
                        cpu_pct: items[7].parse().unwrap(),
                    };
                    let should_empty_line = lines.next();
                    if next_line.is_none() {
                        info!("parsed cpu status header, but next line is none");
                        return results;
                    }
                    if !should_empty_line.unwrap().is_empty() {
                        info!("parsed cpu status header, but next line is not empty");
                        return results;
                    }

                    // IO status
                    // 1677425713    UID       PID   kB_rd/s   kB_wr/s kB_ccwr/s iodelay  Command
                    // 1677425714   1000     15521      0.00      0.00      0.00       0  ckb

                    let next_line = lines.next();
                    if next_line.is_none() {
                        info!("parsed io status header, but next line is none");
                        return results;
                    }

                    if !next_line
                        .unwrap()
                        .contains("UID       PID   kB_rd/s   kB_wr/s kB_ccwr/s iodelay  Command")
                    {
                        info!("parsed io status header, but next line is not io status header");
                        return results;
                    }
                    let io_line = lines.next();
                    if io_line.is_none() {
                        info!("parsed io status header, but next line is none");
                        return results;
                    }
                    let items = io_line.unwrap().split_whitespace().collect::<Vec<_>>();
                    let io_status = IoStat {
                        timestamp: items.first().unwrap().parse().unwrap(),
                        pid: items[2].parse().unwrap(),
                        kb_rd: items[3].parse().unwrap(),
                        kb_wr: items[4].parse().unwrap(),
                        kb_ccwr: items[5].parse().unwrap(),
                        iodelay: items[6].parse().unwrap(),
                    };

                    let pidstat = PidStat {
                        timestamp: io_status.timestamp,
                        cpu_user_pct: pidstatus.user_pct,
                        cpu_system_pct: pidstatus.system_pct,
                        cpu_guest_pct: pidstatus.guest_pct,
                        cpu_wait_pct: pidstatus.wait_pct,
                        cpu_cpu_pct: pidstatus.cpu_pct,
                        io_kb_rd: io_status.kb_rd,
                        io_kb_wr: io_status.kb_wr,
                        io_kb_ccwr: io_status.kb_ccwr,
                        io_iodelay: io_status.iodelay,
                    };
                    if last_timestamp.is_none()
                        || last_timestamp.is_some()
                            && pidstat.timestamp - last_timestamp.unwrap() > duration.as_secs()
                    {
                        last_timestamp = Some(pidstat.timestamp);
                        results.push(pidstat);
                    }
                }
                continue;
            }
            None => {
                return results;
            }
        }
    }
}
