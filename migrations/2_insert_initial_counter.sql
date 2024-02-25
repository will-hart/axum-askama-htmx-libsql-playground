-- ensure we have at least one counter
INSERT INTO counter (id, value) VALUES (1, 0)
ON CONFLICT (id) DO NOTHING;
