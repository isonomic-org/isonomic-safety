// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Anud Inc.

//! Checks the commitments document against its two rules:
//!
//! 1. Every row under a "Planned" heading carries a date.
//! 2. Every row under any other commitments heading names its source in
//!    its last column.
//!
//! The document is markdown with one table per `##` section. Only those
//! tables are read; prose is ignored.

/// One broken rule, with the line it was found on (1-based).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Problem {
    pub line: usize,
    pub message: String,
}

/// Sections whose rows are commitments. Others (such as non-claims) have
/// no table and are skipped.
const DATED_SECTION: &str = "Planned";

const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

/// Check a whole commitments document.
pub fn check(document: &str) -> Vec<Problem> {
    let mut problems = Vec::new();
    let mut section = String::new();
    let mut table_rows = 0usize;

    for (index, line) in document.lines().enumerate() {
        let number = index + 1;
        let trimmed = line.trim();

        if let Some(heading) = trimmed.strip_prefix("## ") {
            section = heading.trim().to_string();
            table_rows = 0;
            continue;
        }
        if !trimmed.starts_with('|') {
            table_rows = 0;
            continue;
        }

        table_rows += 1;
        // Row 1 is the header and row 2 the separator.
        if table_rows <= 2 || is_separator(trimmed) {
            continue;
        }

        let cells = cells(trimmed);
        if cells.iter().all(|c| c.is_empty()) {
            continue;
        }

        if section == DATED_SECTION {
            if !cells.iter().any(|c| has_date(c)) {
                problems.push(Problem {
                    line: number,
                    message: "a planned commitment has no date".to_string(),
                });
            }
        } else if cells.last().is_none_or(|c| c.is_empty()) {
            problems.push(Problem {
                line: number,
                message: format!("a commitment under \"{section}\" names no source"),
            });
        }
    }
    problems
}

fn is_separator(row: &str) -> bool {
    row.chars().all(|c| matches!(c, '|' | '-' | ':' | ' '))
}

fn cells(row: &str) -> Vec<String> {
    let inner = row.trim().trim_start_matches('|').trim_end_matches('|');
    inner.split('|').map(|c| c.trim().to_string()).collect()
}

/// True if the text contains an ISO date (2027-01-22) or a month and a
/// four-digit year (January 2027, 22 January 2027).
pub fn has_date(text: &str) -> bool {
    has_iso_date(text) || has_month_year(text)
}

fn has_iso_date(text: &str) -> bool {
    let bytes = text.as_bytes();
    bytes.windows(10).any(|w| {
        w[..4].iter().all(u8::is_ascii_digit)
            && w[4] == b'-'
            && w[5..7].iter().all(u8::is_ascii_digit)
            && w[7] == b'-'
            && w[8..].iter().all(u8::is_ascii_digit)
    })
}

fn has_month_year(text: &str) -> bool {
    let words: Vec<&str> = text
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .collect();
    words.windows(2).any(|pair| {
        MONTHS.contains(&pair[0])
            && pair[1].len() == 4
            && pair[1].chars().all(|c| c.is_ascii_digit())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dates_are_recognised() {
        assert!(has_date("2027-01-22"));
        assert!(has_date("by 22 January 2027"));
        assert!(has_date("March 2027"));
        assert!(!has_date("soon"));
        assert!(!has_date("Q1"));
        assert!(!has_date("Desktop 9"));
    }

    #[test]
    fn planned_rows_need_a_date() {
        let doc = "## Planned\n\n| Commitment | Date | Source |\n| --- | --- | --- |\n| Thing | soon | Desktop 9 |\n| Other | 2027-03-01 | Desktop 9 |\n";
        let problems = check(doc);
        assert_eq!(problems.len(), 1);
        assert_eq!(problems[0].line, 5);
    }

    #[test]
    fn other_rows_need_a_source() {
        let doc = "## In the build\n\n| Commitment | Source |\n| --- | --- |\n| Thing |  |\n| Other | Agent stack 1 |\n";
        let problems = check(doc);
        assert_eq!(problems.len(), 1);
        assert_eq!(problems[0].line, 5);
    }

    #[test]
    fn empty_tables_and_prose_pass() {
        let doc = "## Shipped\n\nNothing yet.\n\n| Commitment | Source |\n| --- | --- |\n\n## What we do not claim\n\n- Anything.\n";
        assert!(check(doc).is_empty());
    }

    #[test]
    fn the_real_document_passes() {
        let doc = include_str!("../../../docs/commitments.md");
        assert_eq!(check(doc), Vec::new());
    }
}
