// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

pub struct ReportCard {
    pub grade: f32,
    pub student_name: String,
    pub student_age: u8,
    pub grade_type: String,
}

impl ReportCard {
    pub fn print(&self) -> String {
        if self.grade_type == "numeric" {
            format!(
                "{} ({}) - achieved a grade of {}",
                &self.student_name, &self.student_age, &self.grade
            )
        } else {
            let grade = match self.grade {
                x if (x - 5.0).abs() < f32::EPSILON => "A+",
                x if x > 4.5 => "A",
                x if x > 4.0 => "B",
                x if x > 3.5 => "B-",
                x if x > 3.0 => "C",
                x if x > 2.5 => "C-",
                x if x > 2.0 => "D",
                x if x > 1.5 => "D-",
                x if x > 1.0 => "F",
                _ => "Invalid grade",
            };
            format!(
                "{} ({}) - achieved a grade of {}",
                &self.student_name, &self.student_age, grade
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
            grade_type: "numeric".to_string(),
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: 5.0,
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
            grade_type: "alphabetic".to_string(),
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
