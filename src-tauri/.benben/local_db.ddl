
CREATE TABLE 'task_info' (
  'Id' INTEGER PRIMARY KEY AUTOINCREMENT,
  'No' TEXT NOT NULL,
  'Name' TEXT NOT NULL,
  'Desc' TEXT NOT NULL,
  'Type' TEXT NOT NULL,
  'Status' TEXT NOT NULL,
  'Level' TEXT NOT NULL,
  'Hard' TEXT NOT NULL DEFAULT('1'),
  'StartTime' INTEGER NOT NULL,
  'CashTomato' INTEGER
);


CREATE TABLE 'long_term_goals' (
  'Id' INTEGER PRIMARY KEY AUTOINCREMENT,
  'No' TEXT NOT NULL,
  'Name' TEXT NOT NULL,
  'Desc' TEXT NOT NULL,
  'CashTomato' INTEGER,
  'StartTime' INTEGER NOT NULL
);

CREATE TABLE 'to_do_list' (
  'Id' INTEGER PRIMARY KEY AUTOINCREMENT,
  'No' TEXT NOT NULL,
  'Name' TEXT NOT NULL,
  'Desc' TEXT NOT NULL
);

INSERT INTO long_term_goals
("No", Name, StartTime, "Desc", CashTomato)
VALUES( 'common', '通用', 1726502400, '通用任务', NULL);
