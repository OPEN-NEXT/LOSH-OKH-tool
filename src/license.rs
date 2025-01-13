// SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fmt;

/// An owned/no-lifetimes transcription of `Vec<&spdx::expression::ExpressionReq>`
#[derive(Debug, Clone)]
pub struct EvaluationError {
    // The original expression that the ranges of the expressions reffer to
    pub expression: String,
    /// The list of expressions that failed
    pub failed: Vec<spdx::expression::ExpressionReq>,
}

impl From<(String, Vec<&spdx::expression::ExpressionReq>)> for EvaluationError {
    fn from((expression, failures): (String, Vec<&spdx::expression::ExpressionReq>)) -> Self {
        Self {
            expression,
            failed: failures.iter().map(|req| req.to_owned().clone()).collect(),
        }
    }
}

impl fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "evaluation failure(s) in SPDX expression \"{}\": [",
            self.expression
        ))?;
        for req in &self.failed {
            // f.write_fmt(format_args!("    Failed '{}' at \"{}\"", req.req, self.expression[(req.span.start)..(req.span.end)]))?;
            let expr_part = &self
                .expression
                .get((req.span.start as usize)..(req.span.end as usize))
                .unwrap();
            f.write_fmt(format_args!(
                "{{ '{}' - @({},{}) - \"{expr_part}\" }}, ",
                req.req, req.span.start, req.span.end
            ))?;
        }
        f.write_str("]")?;
        Ok(())
    }
}

impl std::error::Error for EvaluationError {}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("There is no license specified.")]
    NoLicense,

    #[error("The license expression is not in a valid SPDX format; see <>.")]
    ParsingFailed(#[from] spdx::ParseError),

    #[error("The license specifier is valid, but the licensing scheme is not approved.")]
    NotApproved(#[from] EvaluationError),
}

pub fn check_osi_approval(expr: &str, spdx_expr: &spdx::Expression) -> Result<(), Error> {
    spdx_expr
        // .evaluate_with_failures(|req| {
        .evaluate_with_failures(|req| {
            if let spdx::LicenseItem::Spdx { id, .. } = req.license {
                return id.is_osi_approved();
            }
            false
        })
        .map_err(|failures| EvaluationError::from((expr.to_owned(), failures)))?;
    Ok(())
}

pub fn validate_spdx_expr(expr: &str, require_osi_approval: bool) -> Result<(), Error> {
    if expr.is_empty() {
        return Err(Error::NoLicense);
    }
    let spdx_expr = spdx::Expression::parse(expr)?;
    if require_osi_approval {
        return check_osi_approval(expr, &spdx_expr);
    }
    Ok(())
}

// TODO The return could be a Cow, but we do not (yet) need it.
pub fn ensure_spdx_license_id(license_id: &str) -> String {
    if validate_spdx_expr(license_id, false).is_ok() {
        license_id.to_owned()
    } else {
        format!("LicenseRef-{license_id}")
    }
}
