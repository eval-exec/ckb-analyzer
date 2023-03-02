#![warn(dead_code)]
/*
2023-02-27 20:47:56.379 +00:00 ChainService INFO ckb_chain::chain  analyze: contextual verify block 6373135 cost: 7302133
2023-02-28 07:06:07.698 +00:00 ChainService INFO ckb_chain::chain  analyze: process_block done: 7147950, elapsed: 73021339
2023-02-28 08:58:24.927 +00:00 ChainService INFO ckb_chain::chain  analyze: non-contextual verify done: 7302133, elapsed: 23109
2023-02-28 08:58:24.952 +00:00 RayonGlobal-0 INFO ckb_verification_contextual::contextual_block_verifier  analyze: block txs verify block: 7302133 tx: 0/4, cost: 12396
2023-02-28 08:58:24.956 +00:00 RayonGlobal-2 INFO ckb_verification_contextual::contextual_block_verifier  analyze: block txs verify block: 7302133 tx: 2/4, cost: 4035162
2023-02-28 08:58:24.961 +00:00 RayonGlobal-1 INFO ckb_verification_contextual::contextual_block_verifier  analyze: block txs verify block: 7302133 tx: 1/4, cost: 9155325
2023-02-28 08:58:24.962 +00:00 RayonGlobal-3 INFO ckb_verification_contextual::contextual_block_verifier  analyze: block txs verify block: 7302133 tx: 3/4, cost: 9848291
2023-02-28 08:58:24.962 +00:00 ChainService INFO ckb_chain::chain  analyze: contextual verify block 7302133 cost: 34538833
2023-02-28 08:58:24.962 +00:00 ChainService INFO ckb_chain::chain  analyze: process_block done: 7302133, elapsed: 48752545
 */
use crate::ckb_log;
use chrono::NaiveDateTime;
use crossbeam_queue::SegQueue;
use indicatif::ProgressBar;
use itertools::Itertools;
use log::info;
use rayon::prelude::*;
use std::time::Duration;

#[derive(Default, Clone, Debug)]
pub struct TimeCost {
    pub timestamp: chrono::NaiveDateTime,
    pub height: u64,
    pub non_contextual_verify: Duration,
    pub contextual_child_txs_verify: Vec<Duration>,
    pub contextual_verify: Duration,
    pub process_block_chain_service: Duration,
    pub full_block_timecost: Duration,
}

pub fn parse(content: &str) -> Vec<TimeCost> {
    let mut results: Vec<TimeCost> = Vec::new();

    let logs = SegQueue::new();
    let lines = content.lines().collect_vec();

    info!("start read structured logs from log file");
    let pb = ProgressBar::new(lines.len() as u64 * 2);
    lines.par_iter().for_each(|line| {
        let log = ckb_log::parse(line, |log_content| log_content.contains("analyze:"));
        if log.is_none() {
            return;
        }
        logs.push(log.unwrap());
        pb.inc(1);
    });
    pb.finish();

    info!("read struct logs from log file finish");
    info!("start parse timecost from structured log");

    let pb = ProgressBar::new(logs.len() as u64);

    let mut timecost = TimeCost::default();
    timecost.height = 1;

    logs.into_iter()
        .sorted_by(|v0, v1| v0.timestamp.cmp(&v1.timestamp))
        .for_each(|log| {
            pb.inc(1);
            if log
                .log_content
                .contains("analyze: non-contextual verify done:")
            {
                let height = log
                    .log_content
                    .split("analyze: non-contextual verify done: ")
                    .nth(1)
                    .unwrap()
                    .split(",")
                    .nth(0)
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
                let elapsed = log
                    .log_content
                    .split("elapsed: ")
                    .nth(1)
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
                if timecost.height > height {
                    return;
                }
                if timecost.height < height {
                    if let Some(last) = results.last() {
                        if last.timestamp.eq(&NaiveDateTime::default()) {
                            return;
                        }
                        timecost.full_block_timecost = Duration::from_nanos(
                            timecost
                                .timestamp
                                .signed_duration_since(last.timestamp)
                                .num_nanoseconds()
                                .unwrap() as u64,
                        );
                        if timecost.full_block_timecost > Duration::from_secs(100) {
                            panic!("{:#?}\n{:#?}", last, timecost);
                        }
                    } else {
                        timecost.full_block_timecost = Duration::from_nanos(0);
                    }
                    results.push(timecost.clone());
                    timecost = TimeCost::default();
                }
                timecost.timestamp = timecost.timestamp.max(log.timestamp);
                timecost.non_contextual_verify = Duration::from_nanos(elapsed);
                timecost.height = height;
            } else if log.log_content.contains("analyze: block txs verify block:") {
                let height = log
                    .log_content
                    .split("analyze: block txs verify block: ")
                    .nth(1)
                    .unwrap()
                    .split(" tx: ")
                    .nth(0)
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
                let elapsed = log
                    .log_content
                    .split("cost: ")
                    .nth(1)
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
                if height < timecost.height {
                    return;
                }
                if height > timecost.height {
                    results.push(timecost.clone());
                    timecost = TimeCost::default();
                }
                timecost.height = height;
                timecost.timestamp = timecost.timestamp.max(log.timestamp);
                timecost
                    .contextual_child_txs_verify
                    .push(Duration::from_nanos(elapsed));
            } else if log.log_content.contains("analyze: contextual verify block") {
                let height = log
                    .log_content
                    .split("analyze: contextual verify block ")
                    .nth(1)
                    .unwrap()
                    .split(" cost: ")
                    .nth(0)
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
                let elapsed = log
                    .log_content
                    .split("cost: ")
                    .nth(1)
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
                if timecost.height > height {
                    return;
                }
                if timecost.height < height {
                    results.push(timecost.clone());
                    timecost = TimeCost::default();
                }
                timecost.height = height;
                timecost.timestamp = timecost.timestamp.max(log.timestamp);
                timecost.contextual_verify = Duration::from_nanos(elapsed);
            } else if log.log_content.contains("analyze: process_block done:") {
                let height = log
                    .log_content
                    .split("analyze: process_block done: ")
                    .nth(1)
                    .unwrap()
                    .split(",")
                    .nth(0)
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
                let elapsed = log
                    .log_content
                    .split("elapsed: ")
                    .nth(1)
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();

                if timecost.height > height {
                    return;
                }

                if timecost.height < height {
                    results.push(timecost.clone());
                    timecost = TimeCost::default();
                }
                timecost.timestamp = timecost.timestamp.max(log.timestamp);
                timecost.process_block_chain_service = Duration::from_nanos(elapsed);
                timecost.height = height;
            }
        });
    pb.finish();
    results
}
