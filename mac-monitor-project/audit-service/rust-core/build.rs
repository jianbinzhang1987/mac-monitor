fn main() {
    // Provide DATABASE_URL for sqlx query macros at compile time.
    println!("cargo:rustc-env=DATABASE_URL=sqlite:///Users/adolf/Desktop/mac-monitor/db/audit.db");
}
