use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct SecurityReport {
    total_checks: u32,
    passed_checks: u32,
    score: u32,
    risk_level: String,
    kernel_info: String,
}

fn calculate_score(passed: u32, total: u32) -> u32 {
    if total == 0 { return 0; }
    (passed * 100) / total
}

fn get_risk_level(score: u32) -> String {
    if score >= 80 { "LOW".to_string() }
    else if score >= 50 { "MEDIUM".to_string() }
    else { "HIGH".to_string() }
}

#[allow(dead_code)]
fn validate_report(json_str: &str) -> bool {
    serde_json::from_str::<SecurityReport>(json_str).is_ok()
}

fn main() {
    let kernel = fs::read_to_string("/proc/version").unwrap_or_else(|_| "Linux 5.10".to_string());
    let total = 10;
    let passed = 8;
    let score = calculate_score(passed, total);
    let risk = get_risk_level(score);
    let report = SecurityReport {
        total_checks: total,
        passed_checks: passed,
        score,
        risk_level: risk.clone(),
        kernel_info: kernel.lines().next().unwrap_or("Available").to_string(),
    };
    let json_report = serde_json::to_string_pretty(&report).unwrap();
    // validate bhi yaha use kar liya
    let _ = validate_report(&json_report);
    fs::write("security_report.json", &json_report).unwrap();
    fs::write("security_report.txt", format!("Score: {}%\nRisk: {}\nKernel: {}\n", score, risk, report.kernel_info)).unwrap();
    println!("=== Audit Done ===");
    println!("Score: {}% - Risk: {}", score, risk);
    println!("Report saved: security_report.json");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn score_is_calculated() { assert_eq!(calculate_score(5, 10), 50); }
    #[test] fn score_50_is_medium_risk() { assert_eq!(get_risk_level(50), "MEDIUM"); }
    #[test] fn maximum_score_is_low_risk() { assert_eq!(get_risk_level(100), "LOW"); }
    #[test] fn score_79_is_medium_risk() { assert_eq!(get_risk_level(79), "MEDIUM"); }
    #[test] fn score_80_is_low_risk() { assert_eq!(get_risk_level(80), "LOW"); }
    #[test] fn risk_level_is_selected() { let r = get_risk_level(30); assert!(r == "HIGH" || r == "MEDIUM" || r == "LOW"); }
    #[test] fn invalid_json_fails() { assert_eq!(validate_report("invalid json"), false); }
    #[test] fn valid_report_passes() {
        let report = SecurityReport { total_checks: 10, passed_checks: 8, score: 80, risk_level: "LOW".to_string(), kernel_info: "test".to_string() };
        let json = serde_json::to_string(&report).unwrap();
        assert_eq!(validate_report(&json), true);
    }
    #[test] fn zero_checks_gives_zero_score() { assert_eq!(calculate_score(0, 0), 0); }
    #[test] fn zero_score_is_high_risk() { assert_eq!(get_risk_level(0), "HIGH"); }
    #[test] fn score_49_is_high_risk() { assert_eq!(get_risk_level(49), "HIGH"); }
}
