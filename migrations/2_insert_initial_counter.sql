-- ensure we have at least one counter with id 1
INSERT INTO counter (id, value) VALUES (1, 0)
ON CONFLICT (id) DO NOTHING;
