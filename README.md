# Week 3 - Secure Coding Practical Submission  
**Student:** Daniel Felipe Rodriguez Avila  
**Project Name:** safe_backup  
**GitHub Repo:** [https://github.com/Dan0322-23/safe_backup_rust](https://github.com/Dan0322-23/safe_backup_rust)  
**Language:** Rust  
**Summary:**

This Rust project, `safe_backup`, demonstrates secure coding techniques in file handling. The program safely backs up a file (e.g., `sample.txt`) to a `.bak` version, logs the backup time, and gracefully handles possible errors using `match` statements. It adheres to secure practices by:
- Avoiding panic-prone operations
- Using `std::fs::copy` for reliable file duplication
- Writing logs using safe `OpenOptions` with explicit error handling
- Separating logic into clear steps

All operations were tested locally and version-controlled with Git.  
Initial commit and push were done to the repository above.
