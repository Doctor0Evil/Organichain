use std::path::PathBuf;
use clap::Parser;
use rayon::prelude::*;
use csv::ReaderBuilder;
use std::fs;
use std::collections::HashSet;

#[derive(Parser, Debug)]
#[command(name = "aln-cli")]
#[command(author = "AU.BioAug Consortium")]
#[command(version = "1.0.0")]
#[command(about = "ALN syntax and compliance validator", long_about = None)]
enum Command {
    #[command(name = "validate-aln")]
    ValidateAln {
        #[arg(long)]
        root: PathBuf,
        #[arg(long)]
        rules: PathBuf,
    },
}

fn main() {
    let cmd = Command::parse();
    match cmd {
        Command::ValidateAln { root, rules } => {
            if let Err(e) = run_validate(root, rules) {
                eprintln!("validation error: {e}");
                std::process::exit(1);
            }
        }
    }
}

#[derive(Clone, Debug)]
struct RuleSets {
    protocol_whitelist: HashSet<String>,
    compliance_whitelist: HashSet<String>,
}

fn load_rules(path: &PathBuf) -> anyhow::Result<RuleSets> {
    let content = fs::read_to_string(path)?;
    let mut protocol_whitelist = HashSet::new();
    let mut compliance_whitelist = HashSet::new();

    for line in content.lines() {
        if line.starts_with("ROW,") {
            let cols: Vec<&str> = line.split(',').collect();
            if cols.len() == 2 && cols[0] == "ROW" {
                continue;
            }
            if cols.len() == 3 {
                match cols[1] {
                    "value" => { /* header row */ }
                    _ => {}
                }
            }
        }
        if line.starts_with("ROW,") && line.contains("PROTOCOL_WHITELIST") {
            // ignored; handled via dedicated section
        }
    }

    // Simple parser: iterate sections explicitly
    let mut section = String::new();
    for line in content.lines() {
        if line.starts_with("SECTION,") {
            section = line["SECTION,".len()..].to_string();
            continue;
        }
        if line.starts_with("ROW,") {
            let cols: Vec<&str> = line.split(',').collect();
            if section == "PROTOCOL_WHITELIST" && cols.len() == 2 && cols[1] != "value" {
                protocol_whitelist.insert(cols[1].trim().to_string());
            }
            if section == "COMPLIANCE_WHITELIST" && cols.len() == 2 && cols[1] != "value" {
                compliance_whitelist.insert(cols[1].trim().to_string());
            }
        }
    }

    Ok(RuleSets {
        protocol_whitelist,
        compliance_whitelist,
    })
}

fn run_validate(root: PathBuf, rules_path: PathBuf) -> anyhow::Result<()> {
    let rules = load_rules(&rules_path)?;
    let mut aln_files = Vec::new();

    for entry in walkdir::WalkDir::new(&root).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension() {
                if ext == "aln" {
                    aln_files.push(entry.path().to_path_buf());
                }
            }
        }
    }

    let errors: Vec<String> = aln_files
        .par_iter()
        .filter_map(|path| validate_single(path, &rules).err().map(|e| format!("{path:?}: {e}")))
        .collect();

    if !errors.is_empty() {
        for e in errors {
            eprintln!("{e}");
        }
        anyhow::bail!("ALN validation failed");
    }

    Ok(())
}

fn validate_single(path: &PathBuf, rules: &RuleSets) -> anyhow::Result<()> {
    let content = fs::read_to_string(path)?;
    for line in content.lines() {
        if line.starts_with("ROW,protocol_stack,") {
            let cols: Vec<&str> = line.split(',').collect();
            if cols.len() >= 3 {
                let stack = cols[2].trim();
                for proto in stack.split('+') {
                    if !rules.protocol_whitelist.contains(proto.trim()) {
                        anyhow::bail!("unknown protocol {proto} in {path:?}");
                    }
                }
            }
        }
        if line.starts_with("ROW,compliance_check,") || line.starts_with("ROW,compliance,") {
            let cols: Vec<&str> = line.split(',').collect();
            if cols.len() >= 3 {
                let list = cols[2].trim();
                for c in list.split('+') {
                    if !rules.compliance_whitelist.contains(c.trim()) {
                        anyhow::bail!("unknown compliance tag {c} in {path:?}");
                    }
                }
            }
        }
    }
    Ok(())
}
