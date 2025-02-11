Alter Table if exists token
Drop Column user_id;

Alter Table if exists token
Add Column session_id not null references session_table;