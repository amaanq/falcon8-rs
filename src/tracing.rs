use crate::Report;

#[cfg(feature = "tracing")]
pub fn debug_report(report: &Report, set: bool) {
    tracing::debug!("{} {report:#?}", if set { "SET" } else { "GET" });
    tracing::trace!(
        "{} Report:\n{}",
        pretty_hex::pretty_hex(report.as_bytes()),
        if set { "SET" } else { "GET" }
    );
}

#[cfg(not(feature = "tracing"))]
pub fn debug_report(_report: &Report) {}
