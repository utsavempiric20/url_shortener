use chrono::{ DateTime, Utc };

struct Link {
    slug: String,
    long_url: String,
    created_at: DateTime<Utc>,
    clicks: u64,
}
