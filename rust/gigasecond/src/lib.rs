use chrono::{DateTime, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let gig = chrono::Duration::seconds(1000000000);

    let new_time = start.checked_add_signed(gig);

    return new_time;
}
