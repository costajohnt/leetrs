//! CLI result formatting for submission and test results.
//!
//! Keeps all `println!` / `eprintln!` calls out of business logic so that
//! [`SubmissionResult`] can later be displayed in the TUI as well.
use std::fmt::Write as _;

use crate::services::submission::{SubmissionResult, SubmissionStatus};

/// Prints a human-readable summary of a submission or test result to stdout.
pub fn format_result(result: &SubmissionResult) {
    println!("\n==================================================");

    if result.is_test {
        let passed = result.correct_answer.unwrap_or(false);
        if passed {
            println!("  ✅ All test cases passed");
        } else {
            println!("  ❌ Test Failed");
        }
        println!("==================================================\n");

        if let (Some(correct), Some(total)) = (result.total_correct, result.total_testcases) {
            println!("🧪 Testcases: {} / {} passed", correct, total);
        }

        match &result.status {
            SubmissionStatus::Accepted if passed => {
                if let Some(runtime) = &result.runtime {
                    println!("⏱️ Runtime: {}", runtime);
                }
                if let Some(memory) = &result.memory {
                    println!("💾 Memory: {}", memory);
                }
                if let Some(mp) = result.memory_percentile {
                    println!("📝 Memory Percentile: {:.2}%", mp);
                }
                if let Some(rp) = result.runtime_percentile {
                    println!("⏰ Runtime Percentile: {:.2}%", rp);
                }
            }
            SubmissionStatus::Accepted => {
                if let (Some(code_answers), Some(expected)) =
                    (&result.code_answer, &result.expected_code_answer)
                {
                    println!("Expected");
                    println!("{}", expected.join("\t"));
                    println!("Output");
                    println!("{}", code_answers.join("\t"));
                }
            }
            SubmissionStatus::RuntimeError => {
                if let Some(err) = &result.full_runtime_error {
                    println!("❌ Error\n{}", err);
                }
            }
            _ => {}
        }
    } else {
        let status_label = match &result.status {
            SubmissionStatus::Accepted => {
                println!("  ✅ Accepted");
                "Accepted"
            }
            SubmissionStatus::WrongAnswer => {
                println!("  ❌ Wrong Answer");
                "Wrong Answer"
            }
            SubmissionStatus::CompileError => {
                println!("  ❌ Compile Error");
                "Compile Error"
            }
            SubmissionStatus::RuntimeError => {
                println!("  ❌ Runtime Error");
                "Runtime Error"
            }
            SubmissionStatus::TimeLimitExceeded => {
                println!("  ❌ Time Limit Exceeded");
                "Time Limit Exceeded"
            }
            SubmissionStatus::Unknown(msg) => {
                println!("  ❌ {}", msg);
                msg.as_str()
            }
        };
        println!("==================================================\n");

        if let (Some(correct), Some(total)) = (result.total_correct, result.total_testcases) {
            println!("🧪 Testcases: {} / {} passed", correct, total);
        }

        match &result.status {
            SubmissionStatus::Accepted => {
                if let Some(runtime) = &result.runtime {
                    println!("⏱️ Runtime: {}", runtime);
                }
                if let Some(memory) = &result.memory {
                    println!("💾 Memory: {}", memory);
                }
                if let Some(mp) = result.memory_percentile {
                    println!("📝 Memory Percentile: {:.2}%", mp);
                }
                if let Some(rp) = result.runtime_percentile {
                    println!("⏰ Runtime Percentile: {:.2}%", rp);
                }
            }
            SubmissionStatus::CompileError => {
                if let Some(err) = &result.compile_error {
                    println!("💥 Compiler Output:\n{}", err);
                }
            }
            SubmissionStatus::WrongAnswer => {
                if let Some(input) = &result.input {
                    print!("INPUT: ");
                    for part in input.split('\n') {
                        print!("{}\t", part);
                    }
                    println!();
                }
                if let (Some(expected), Some(output)) =
                    (&result.expected_output, &result.code_output)
                {
                    println!("Expected: {}\nOutput: {}", expected, output);
                }
            }
            SubmissionStatus::RuntimeError => {
                if let Some(err) = &result.full_runtime_error {
                    println!("❌ Error\n{}", err);
                }
            }
            _ => {
                let _ = status_label;
            }
        }
    }

    if let Some(stdout) = format_stdout(result.std_output_list.as_deref()) {
        println!("\n{}", stdout);
    }
}

/// Formats captured stdout from test cases or submission into a human-readable string,
/// or returns `None` if there is no non-empty stdout.
pub fn format_stdout(std_output_list: Option<&[String]>) -> Option<String> {
    let list = std_output_list?;
    if !list.iter().any(|s| !s.trim().is_empty()) {
        return None;
    }

    if list.len() == 1 {
        let trimmed = list[0].trim_end();
        return Some(format!("📢 Stdout:\n{}", trimmed));
    }

    let mut out = String::new();
    let mut first_section = true;
    for (i, stdout) in list.iter().enumerate() {
        let trimmed = stdout.trim_end();
        if trimmed.trim().is_empty() {
            continue;
        }
        if !first_section {
            out.push_str("\n\n");
        }
        let _ = write!(out, "📢 Stdout (Testcase {}):\n{}", i + 1, trimmed);
        first_section = false;
    }

    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_stdout_none_or_empty() {
        assert_eq!(format_stdout(None), None);
        assert_eq!(format_stdout(Some(&[])), None);
        let empty_list = vec!["".to_string(), "   \n".to_string()];
        assert_eq!(format_stdout(Some(&empty_list)), None);
    }

    #[test]
    fn test_format_stdout_single_testcase() {
        let list = vec!["debug statement\n".to_string()];
        assert_eq!(
            format_stdout(Some(&list)),
            Some("📢 Stdout:\ndebug statement".to_string())
        );
    }

    #[test]
    fn test_format_stdout_multiple_testcases() {
        let list = vec!["case 1\n".to_string(), "case 2\n".to_string()];
        assert_eq!(
            format_stdout(Some(&list)),
            Some("📢 Stdout (Testcase 1):\ncase 1\n\n📢 Stdout (Testcase 2):\ncase 2".to_string())
        );
    }

    #[test]
    fn test_format_stdout_multiple_with_some_empty() {
        let list = vec!["".to_string(), "case 2\n".to_string()];
        assert_eq!(
            format_stdout(Some(&list)),
            Some("📢 Stdout (Testcase 2):\ncase 2".to_string())
        );
    }

    #[test]
    fn test_format_result_test_passed_with_stdout() {
        let res = SubmissionResult {
            status: SubmissionStatus::Accepted,
            is_test: true,
            correct_answer: Some(true),
            code_answer: None,
            expected_code_answer: None,
            total_correct: Some(3),
            total_testcases: Some(3),
            runtime: Some("0 ms".to_string()),
            memory: Some("2 MB".to_string()),
            memory_percentile: Some(99.0),
            runtime_percentile: Some(100.0),
            compile_error: None,
            full_runtime_error: None,
            input: None,
            expected_output: None,
            code_output: None,
            std_output_list: Some(vec!["hello testcase 1\n".to_string()]),
        };
        // Calling format_result should run without panicking and print test summary and stdout
        format_result(&res);
    }

    #[test]
    fn test_format_result_test_failed_with_expected_and_output() {
        let res = SubmissionResult {
            status: SubmissionStatus::Accepted,
            is_test: true,
            correct_answer: Some(false),
            code_answer: Some(vec!["[1]".to_string()]),
            expected_code_answer: Some(vec!["[2]".to_string()]),
            total_correct: Some(1),
            total_testcases: Some(2),
            runtime: None,
            memory: None,
            memory_percentile: None,
            runtime_percentile: None,
            compile_error: None,
            full_runtime_error: None,
            input: None,
            expected_output: None,
            code_output: None,
            std_output_list: Some(vec!["fail debug\n".to_string()]),
        };
        format_result(&res);
    }

    #[test]
    fn test_format_result_test_runtime_error() {
        let res = SubmissionResult {
            status: SubmissionStatus::RuntimeError,
            is_test: true,
            correct_answer: Some(false),
            code_answer: None,
            expected_code_answer: None,
            total_correct: None,
            total_testcases: None,
            runtime: None,
            memory: None,
            memory_percentile: None,
            runtime_percentile: None,
            compile_error: None,
            full_runtime_error: Some("index out of bounds".to_string()),
            input: None,
            expected_output: None,
            code_output: None,
            std_output_list: None,
        };
        format_result(&res);
    }

    #[test]
    fn test_format_result_submit_accepted_with_stdout() {
        let res = SubmissionResult {
            status: SubmissionStatus::Accepted,
            is_test: false,
            correct_answer: None,
            code_answer: None,
            expected_code_answer: None,
            total_correct: Some(57),
            total_testcases: Some(57),
            runtime: Some("4 ms".to_string()),
            memory: Some("12 MB".to_string()),
            memory_percentile: Some(80.0),
            runtime_percentile: Some(75.0),
            compile_error: None,
            full_runtime_error: None,
            input: None,
            expected_output: None,
            code_output: None,
            std_output_list: Some(vec!["submit success output\n".to_string()]),
        };
        format_result(&res);
    }

    #[test]
    fn test_format_result_submit_wrong_answer() {
        let res = SubmissionResult {
            status: SubmissionStatus::WrongAnswer,
            is_test: false,
            correct_answer: None,
            code_answer: None,
            expected_code_answer: None,
            total_correct: Some(10),
            total_testcases: Some(57),
            runtime: None,
            memory: None,
            memory_percentile: None,
            runtime_percentile: None,
            compile_error: None,
            full_runtime_error: None,
            input: Some("1\n2".to_string()),
            expected_output: Some("3".to_string()),
            code_output: Some("4".to_string()),
            std_output_list: None,
        };
        format_result(&res);
    }

    #[test]
    fn test_format_result_submit_compile_error() {
        let res = SubmissionResult {
            status: SubmissionStatus::CompileError,
            is_test: false,
            correct_answer: None,
            code_answer: None,
            expected_code_answer: None,
            total_correct: None,
            total_testcases: None,
            runtime: None,
            memory: None,
            memory_percentile: None,
            runtime_percentile: None,
            compile_error: Some("syntax error on line 5".to_string()),
            full_runtime_error: None,
            input: None,
            expected_output: None,
            code_output: None,
            std_output_list: None,
        };
        format_result(&res);
    }

    #[test]
    fn test_format_result_submit_runtime_error() {
        let res = SubmissionResult {
            status: SubmissionStatus::RuntimeError,
            is_test: false,
            correct_answer: None,
            code_answer: None,
            expected_code_answer: None,
            total_correct: None,
            total_testcases: None,
            runtime: None,
            memory: None,
            memory_percentile: None,
            runtime_percentile: None,
            compile_error: None,
            full_runtime_error: Some("panic at unwrap".to_string()),
            input: None,
            expected_output: None,
            code_output: None,
            std_output_list: None,
        };
        format_result(&res);
    }

    #[test]
    fn test_format_result_submit_time_limit_and_unknown() {
        let res_tle = SubmissionResult {
            status: SubmissionStatus::TimeLimitExceeded,
            is_test: false,
            correct_answer: None,
            code_answer: None,
            expected_code_answer: None,
            total_correct: None,
            total_testcases: None,
            runtime: None,
            memory: None,
            memory_percentile: None,
            runtime_percentile: None,
            compile_error: None,
            full_runtime_error: None,
            input: None,
            expected_output: None,
            code_output: None,
            std_output_list: None,
        };
        format_result(&res_tle);

        let res_unknown = SubmissionResult {
            status: SubmissionStatus::Unknown("Internal Server Error".to_string()),
            is_test: false,
            correct_answer: None,
            code_answer: None,
            expected_code_answer: None,
            total_correct: None,
            total_testcases: None,
            runtime: None,
            memory: None,
            memory_percentile: None,
            runtime_percentile: None,
            compile_error: None,
            full_runtime_error: None,
            input: None,
            expected_output: None,
            code_output: None,
            std_output_list: None,
        };
        format_result(&res_unknown);
    }
}
