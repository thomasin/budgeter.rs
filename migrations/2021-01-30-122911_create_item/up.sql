CREATE TABLE items (
  id INTEGER NOT NULL PRIMARY KEY,
  name VARCHAR NOT NULL UNIQUE,
  duration_unit TEXT NOT NULL CHECK(duration_unit IN ('D','M')),
  duration_amount INTEGER NOT NULL CHECK(duration_amount >= 0),
  cost REAL NOT NULL CHECK(cost >= 0)
)
