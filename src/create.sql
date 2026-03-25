CREATE TABLE IF NOT EXISTS sqlar(
    name TEXT PRIMARY KEY,
    mode INT,
    mtime INT,
    sz INT,
    data BLOB
);
