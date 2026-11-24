// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Anud Inc.

//! `claims-check [path]`: check a commitments document. Defaults to
//! `docs/commitments.md`. Exits non-zero if any rule is broken.

use std::process::ExitCode;

fn main() -> ExitCode {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "docs/commitments.md".to_string());
    let document = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(error) => {
            eprintln!("claims-check: cannot read {path}: {error}");
            return ExitCode::from(2);
        }
    };
    let problems = claims_check::check(&document);
    if problems.is_empty() {
        println!("claims-check: {path} passes");
        return ExitCode::SUCCESS;
    }
    for problem in &problems {
        eprintln!("{path}:{}: {}", problem.line, problem.message);
    }
    ExitCode::FAILURE
}
