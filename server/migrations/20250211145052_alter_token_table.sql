Alter Table if exists token
Drop Column if exists user_id;

Alter Table if exists token
Add Column if not exists session_id UUID references session_table;